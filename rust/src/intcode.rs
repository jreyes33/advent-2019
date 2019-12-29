use self::Mode::*;
use self::Op::*;
use crate::Result;
use std::collections::VecDeque;
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
    AdjustRelBase(Mode),
    Halt,
}

enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Default for Mode {
    fn default() -> Self {
        Position
    }
}

pub struct Execution {
    program: Vec<i64>,
    inputs: VecDeque<i64>,
    i: usize,
    relative_base: i64,
}

impl Execution {
    pub fn new(program_arg: Vec<i64>, inputs_arg: Vec<i64>) -> Self {
        let mut program = program_arg.clone();
        // TODO: resize dynamically
        program.resize(program_arg.len() + 500, 0);
        Execution {
            program,
            inputs: inputs_arg.into(),
            i: 0,
            relative_base: 0,
        }
    }

    pub fn program_at(&self, idx: usize) -> Result<i64> {
        if let Some(n) = self.program.get(idx) {
            Ok(*n)
        } else {
            Err(format!("No such idx: {}", idx).into())
        }
    }

    pub fn add_input(&mut self, input: i64) {
        self.inputs.push_back(input);
    }

    pub fn run(&mut self) -> Result<Vec<i64>> {
        let mut outputs = vec![];
        loop {
            let op = Self::parse_instruction(self.program[self.i])?;
            match op {
                Sum(mode1, mode2, mode3) => {
                    // TODO: remove duplication
                    let idx = match mode3 {
                        Relative => self.program[self.i + 3] + self.relative_base,
                        _ => self.program[self.i + 3],
                    } as usize;
                    self.program[idx] = self.get(1, mode1) + self.get(2, mode2);
                    self.i += 4;
                }
                Mult(mode1, mode2, mode3) => {
                    let idx = match mode3 {
                        Relative => self.program[self.i + 3] + self.relative_base,
                        _ => self.program[self.i + 3],
                    } as usize;
                    self.program[idx] = self.get(1, mode1) * self.get(2, mode2);
                    self.i += 4;
                }
                In(mode1) => {
                    let idx = match mode1 {
                        Relative => self.program[self.i + 1] + self.relative_base,
                        _ => self.program[self.i + 1],
                    } as usize;
                    if let Some(input) = self.inputs.pop_front() {
                        self.program[idx] = input;
                    } else {
                        return Ok(outputs);
                    }
                    self.i += 2;
                }
                Out(mode1) => {
                    outputs.push(self.get(1, mode1));
                    self.i += 2;
                }
                JumpIfTrue(mode1, mode2) => {
                    if self.get(1, mode1) != 0 {
                        self.i = self.get(2, mode2) as usize;
                    } else {
                        self.i += 3;
                    }
                }
                JumpIfFalse(mode1, mode2) => {
                    if self.get(1, mode1) == 0 {
                        self.i = self.get(2, mode2) as usize;
                    } else {
                        self.i += 3;
                    }
                }
                LessThan(mode1, mode2, mode3) => {
                    let idx = match mode3 {
                        Relative => self.program[self.i + 3] + self.relative_base,
                        _ => self.program[self.i + 3],
                    } as usize;
                    self.program[idx] = if self.get(1, mode1) < self.get(2, mode2) {
                        1
                    } else {
                        0
                    };
                    self.i += 4;
                }
                Equals(mode1, mode2, mode3) => {
                    let idx = match mode3 {
                        Relative => self.program[self.i + 3] + self.relative_base,
                        _ => self.program[self.i + 3],
                    } as usize;
                    self.program[idx] = if self.get(1, mode1) == self.get(2, mode2) {
                        1
                    } else {
                        0
                    };
                    self.i += 4;
                }
                AdjustRelBase(mode1) => {
                    self.relative_base += self.get(1, mode1);
                    self.i += 2;
                }
                Halt => return Ok(outputs),
            }
        }
    }

    fn get(&self, offset: usize, mode: Mode) -> i64 {
        match mode {
            Immediate => self.program[self.i + offset],
            Relative => {
                let idx = (self.relative_base + self.program[self.i + offset]) as usize;
                self.program[idx]
            }
            Position => {
                let idx = self.program[self.i + offset] as usize;
                self.program[idx]
            }
        }
    }

    fn parse_instruction(instruction: i64) -> Result<Op> {
        let opcode = instruction % 100;
        let mut modescode = instruction / 100;
        let mut modes = vec![];

        while modescode > 0 {
            let mode = match modescode % 10 {
                0 => Position,
                1 => Immediate,
                2 => Relative,
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
            9 => AdjustRelBase(modes_iter.next().unwrap_or_default()),
            99 => Halt,
            weird_opcode => {
                return Err(format!("Not a valid opcode: {}", weird_opcode).into());
            }
        };
        Ok(op)
    }
}

pub fn compute_get_at(program_arg: Vec<i64>, inputs: Vec<i64>, idx: usize) -> Result<i64> {
    let mut execution = Execution::new(program_arg, inputs);
    execution.run()?;
    execution.program_at(idx)
}

pub fn compute(program_arg: Vec<i64>, inputs: Vec<i64>) -> Result<Vec<i64>> {
    let mut execution = Execution::new(program_arg, inputs);
    execution.run()
}

pub fn parse_input(path: &str) -> Result<Vec<i64>> {
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

    #[test]
    fn test_quine() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        assert_eq!(program.clone(), compute(program, vec![]).unwrap());
    }

    #[test]
    fn test_relative_input() {
        let program = vec![109, 3, 203, 2, 104, 0, 99];
        assert_eq!(vec![42], compute(program, vec![42]).unwrap());
    }

    #[test]
    fn test_large_numbers() {
        let program = vec![104, 1125899906842624, 99];
        assert_eq!(vec![1125899906842624], compute(program, vec![]).unwrap());
    }

    #[test]
    fn test_other_large_numbers() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        assert_eq!(vec![1219070632396864], compute(program, vec![]).unwrap());
    }
}
