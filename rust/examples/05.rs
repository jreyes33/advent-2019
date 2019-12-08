use crate::Mode::*;
use crate::Op::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

enum Op {
    Sum(Mode, Mode, Mode),
    Mult(Mode, Mode, Mode),
    In(Mode),
    Out(Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equals(Mode, Mode, Mode),
    Halt,
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

impl Default for Mode {
    fn default() -> Self {
        Position
    }
}

impl Mode {
    fn get(&self, program: &[i32], i: usize) -> i32 {
        match self {
            Immediate => program[i],
            Position => {
                let idx = program[i] as usize;
                program[idx]
            }
        }
    }
}

fn parse_input(path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    let numbers = contents
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();
    Ok(numbers)
}

fn compute(
    program_arg: Vec<i32>,
    inputs: Vec<i32>,
) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let mut program = program_arg;
    let mut outputs = vec![];
    let mut inputs_iter = inputs.into_iter();
    let mut i = 0;
    loop {
        let op = parse_instruction(program[i])?;
        match op {
            Sum(mode1, mode2, _) => {
                let idx = program[i + 3] as usize;
                program[idx] = mode1.get(&program, i + 1) + mode2.get(&program, i + 2);
                i += 4;
            }
            Mult(mode1, mode2, _) => {
                let idx = program[i + 3] as usize;
                program[idx] = mode1.get(&program, i + 1) * mode2.get(&program, i + 2);
                i += 4;
            }
            In(_) => {
                let idx = program[i + 1] as usize;
                program[idx] = inputs_iter.next().expect("should have inputs");
                i += 2;
            }
            Out(mode1) => {
                outputs.push(mode1.get(&program, i + 1));
                i += 2;
            }
            JumpIfTrue(mode1, mode2) => {
                if mode1.get(&program, i + 1) != 0 {
                    i = mode2.get(&program, i + 2) as usize;
                } else {
                    i += 3;
                }
            }
            JumpIfFalse(mode1, mode2) => {
                if mode1.get(&program, i + 1) == 0 {
                    i = mode2.get(&program, i + 2) as usize;
                } else {
                    i += 3;
                }
            }
            LessThan(mode1, mode2, _) => {
                let idx = program[i + 3] as usize;
                program[idx] = if mode1.get(&program, i + 1) < mode2.get(&program, i + 2) {
                    1
                } else {
                    0
                };
                i += 4;
            }
            Equals(mode1, mode2, _) => {
                let idx = program[i + 3] as usize;
                program[idx] = if mode1.get(&program, i + 1) == mode2.get(&program, i + 2) {
                    1
                } else {
                    0
                };
                i += 4;
            }
            Halt => return Ok((program, outputs)),
        }
    }
}

fn parse_instruction(instruction: i32) -> Result<Op, Box<dyn Error>> {
    let opcode = instruction % 100;
    let mut modescode = instruction / 100;
    let mut modes = vec![];

    while modescode > 0 {
        let mode = match modescode % 10 {
            0 => Position,
            1 => Immediate,
            weird_modecode => {
                let err = From::from(format!("Not a valid modecode: {}", weird_modecode));
                return Err(err);
            }
        };
        modes.push(mode);
        modescode /= 10;
    }

    let mut modes_iter = modes.into_iter();
    let op = match opcode {
        1 => Sum(
            modes_iter.next().unwrap_or_default(),
            modes_iter.next().unwrap_or_default(),
            modes_iter.next().unwrap_or_default(),
        ),
        2 => Mult(
            modes_iter.next().unwrap_or_default(),
            modes_iter.next().unwrap_or_default(),
            modes_iter.next().unwrap_or_default(),
        ),
        3 => In(modes_iter.next().unwrap_or_default()),
        4 => Out(modes_iter.next().unwrap_or_default()),
        5 => JumpIfTrue(
            modes_iter.next().unwrap_or_default(),
            modes_iter.next().unwrap_or_default(),
        ),
        6 => JumpIfFalse(
            modes_iter.next().unwrap_or_default(),
            modes_iter.next().unwrap_or_default(),
        ),
        7 => LessThan(
            modes_iter.next().unwrap_or_default(),
            modes_iter.next().unwrap_or_default(),
            modes_iter.next().unwrap_or_default(),
        ),
        8 => Equals(
            modes_iter.next().unwrap_or_default(),
            modes_iter.next().unwrap_or_default(),
            modes_iter.next().unwrap_or_default(),
        ),
        _ => Halt,
    };
    Ok(op)
}

fn part1() -> Result<i32, Box<dyn Error>> {
    let program = parse_input("../inputs/05-input.txt")?;
    let inputs = vec![1];
    let (_program_result, outputs) = compute(program, inputs)?;
    Ok(*outputs.last().ok_or("No outputs")?)
}

fn part2() -> Result<i32, Box<dyn Error>> {
    let program = parse_input("../inputs/05-input.txt")?;
    let inputs = vec![5];
    let (_program_result, outputs) = compute(program, inputs)?;
    Ok(*outputs.last().ok_or("No outputs")?)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", part1()?);
    println!("{}", part2()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_compute() {
        assert_eq!(
            (vec![2, 0, 0, 0, 99], vec![]),
            compute(vec![1, 0, 0, 0, 99], vec![1]).unwrap()
        );
        assert_eq!(
            (vec![2, 3, 0, 6, 99], vec![]),
            compute(vec![2, 3, 0, 3, 99], vec![1]).unwrap()
        );
        assert_eq!(
            (vec![1002, 4, 3, 4, 99], vec![]),
            compute(vec![1002, 4, 3, 4, 33], vec![1]).unwrap()
        );
        assert_eq!(
            (vec![1101, 100, -1, 4, 99], vec![]),
            compute(vec![1101, 100, -1, 4, 0], vec![1]).unwrap(),
        );
    }

    #[test]
    fn test_compute_with_new_ops_from_part2() {
        let program = parse_input("../inputs/05-example.txt").unwrap();
        let inputs = vec![8];
        assert_eq!(vec![1000], compute(program, inputs).unwrap().1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(7286649, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(15724522, part2().unwrap());
    }
}
