use self::Mode::*;
use self::Op::*;
use crate::Result;
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

pub struct Execution {
    program: Vec<i32>,
    inputs: Vec<i32>,
    i: usize,
}

impl Execution {
    pub fn new(program: Vec<i32>, inputs: Vec<i32>) -> Self {
        Execution {
            program,
            inputs,
            i: 0,
        }
    }

    pub fn program_at(&self, idx: usize) -> Result<i32> {
        if let Some(n) = self.program.get(idx) {
            Ok(*n)
        } else {
            Err(format!("No such idx: {}", idx).into())
        }
    }

    pub fn add_input(&mut self, input: i32) {
        self.inputs.push(input);
    }

    pub fn run(&mut self) -> Result<Vec<i32>> {
        let prog = &mut self.program;
        let mut outputs = vec![];
        let mut inputs_iter = self.inputs.drain(..);
        loop {
            let op = Self::parse_instruction(prog[self.i])?;
            match op {
                Sum(mode1, mode2, _) => {
                    let idx = prog[self.i + 3] as usize;
                    prog[idx] = mode1.get(&prog, self.i + 1) + mode2.get(&prog, self.i + 2);
                    self.i += 4;
                }
                Mult(mode1, mode2, _) => {
                    let idx = prog[self.i + 3] as usize;
                    prog[idx] = mode1.get(&prog, self.i + 1) * mode2.get(&prog, self.i + 2);
                    self.i += 4;
                }
                In(_) => {
                    let idx = prog[self.i + 1] as usize;
                    if let Some(input) = inputs_iter.next() {
                        prog[idx] = input;
                    } else {
                        return Ok(outputs);
                    }
                    self.i += 2;
                }
                Out(mode1) => {
                    outputs.push(mode1.get(&prog, self.i + 1));
                    self.i += 2;
                }
                JumpIfTrue(mode1, mode2) => {
                    if mode1.get(&prog, self.i + 1) != 0 {
                        self.i = mode2.get(&prog, self.i + 2) as usize;
                    } else {
                        self.i += 3;
                    }
                }
                JumpIfFalse(mode1, mode2) => {
                    if mode1.get(&prog, self.i + 1) == 0 {
                        self.i = mode2.get(&prog, self.i + 2) as usize;
                    } else {
                        self.i += 3;
                    }
                }
                LessThan(mode1, mode2, _) => {
                    let idx = prog[self.i + 3] as usize;
                    prog[idx] = if mode1.get(&prog, self.i + 1) < mode2.get(&prog, self.i + 2) {
                        1
                    } else {
                        0
                    };
                    self.i += 4;
                }
                Equals(mode1, mode2, _) => {
                    let idx = prog[self.i + 3] as usize;
                    prog[idx] = if mode1.get(&prog, self.i + 1) == mode2.get(&prog, self.i + 2) {
                        1
                    } else {
                        0
                    };
                    self.i += 4;
                }
                Halt => return Ok(outputs),
            }
        }
    }

    fn parse_instruction(instruction: i32) -> Result<Op> {
        let opcode = instruction % 100;
        let mut modescode = instruction / 100;
        let mut modes = vec![];

        while modescode > 0 {
            let mode = match modescode % 10 {
                0 => Position,
                1 => Immediate,
                weird_modecode => {
                    return Err(format!("Not a valid modecode: {}", weird_modecode).into());
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
            99 => Halt,
            weird_opcode => {
                return Err(format!("Not a valid opcode: {}", weird_opcode).into());
            }
        };
        Ok(op)
    }
}

pub fn compute_get_at(program_arg: Vec<i32>, inputs: Vec<i32>, idx: usize) -> Result<i32> {
    let mut execution = Execution::new(program_arg, inputs);
    execution.run()?;
    execution.program_at(idx)
}

pub fn compute(program_arg: Vec<i32>, inputs: Vec<i32>) -> Result<Vec<i32>> {
    let mut execution = Execution::new(program_arg, inputs);
    execution.run()
}

pub fn parse_input(path: &str) -> Result<Vec<i32>> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    let numbers = contents
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();
    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_compute() {
        assert_eq!(2, compute_get_at(vec![1, 0, 0, 0, 99], vec![], 0).unwrap());
        assert_eq!(6, compute_get_at(vec![2, 3, 0, 3, 99], vec![], 3).unwrap());
    }

    #[test]
    fn test_longer_compute() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(30, compute_get_at(program, vec![], 0).unwrap());
    }

    #[test]
    fn test_new_ops_compute() {
        assert_eq!(2, compute_get_at(vec![1, 0, 0, 0, 99], vec![1], 0).unwrap());
        assert_eq!(6, compute_get_at(vec![2, 3, 0, 3, 99], vec![1], 3).unwrap());
        assert_eq!(
            99,
            compute_get_at(vec![1002, 4, 3, 4, 33], vec![1], 4).unwrap()
        );
        assert_eq!(
            99,
            compute_get_at(vec![1101, 100, -1, 4, 0], vec![1], 4).unwrap()
        );
    }

    #[test]
    fn test_compute_with_new_ops_from_part2() {
        let program = parse_input("../inputs/05-example.txt").unwrap();
        let inputs = vec![8];
        assert_eq!(vec![1000], compute(program, inputs).unwrap());
    }
}
