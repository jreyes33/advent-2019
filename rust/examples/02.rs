use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn parse_input() -> Result<Vec<usize>, Box<dyn Error>> {
    let mut contents = String::new();
    File::open("../inputs/02-input.txt")?.read_to_string(&mut contents)?;
    let numbers = contents
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();
    Ok(numbers)
}

fn compute(input: Vec<usize>) -> Vec<usize> {
    let mut program = input;
    let mut i = 0;
    loop {
        match program.get(i..i + 4) {
            Some(&[1, arg1, arg2, arg3]) => {
                program[arg3] = program[arg1] + program[arg2];
            }
            Some(&[2, arg1, arg2, arg3]) => {
                program[arg3] = program[arg1] * program[arg2];
            }
            _ => {
                break;
            }
        }
        i += 4;
    }
    program
}

fn part1() -> Result<usize, Box<dyn Error>> {
    let mut program = parse_input()?;
    program[1] = 12;
    program[2] = 2;
    let output = compute(program);
    Ok(output[0])
}

fn part2() -> Result<usize, Box<dyn Error>> {
    let input = parse_input()?;
    for i in 0..100 {
        for j in 0..100 {
            let mut program = input.clone();
            program[1] = i;
            program[2] = j;
            let output = compute(program);
            if output[0] == 19_690_720 {
                return Ok(100 * i + j);
            }
        }
    }
    Err(From::from("No inputs produced expected output"))
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
        assert_eq!(vec![2, 0, 0, 0, 99], compute(vec![1, 0, 0, 0, 99]));
        assert_eq!(vec![2, 3, 0, 6, 99], compute(vec![2, 3, 0, 3, 99]));
    }

    #[test]
    fn test_longer_compute() {
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(expected, compute(program));
    }

    #[test]
    fn test_part1() {
        assert_eq!(2890696, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(8226, part2().unwrap());
    }
}
