use advent_2019::intcode::{compute_get_at, parse_input};
use advent_2019::Result;

fn part1() -> Result<i64> {
    let mut program = parse_input("../inputs/02-input.txt")?;
    program[1] = 12;
    program[2] = 2;
    Ok((compute_get_at(program, vec![], 0))?)
}

fn part2() -> Result<i64> {
    let input = parse_input("../inputs/02-input.txt")?;
    for i in 0..100 {
        for j in 0..100 {
            let mut program = input.clone();
            program[1] = i;
            program[2] = j;
            if compute_get_at(program, vec![], 0)? == 19_690_720 {
                return Ok(100 * i + j);
            }
        }
    }
    Err(From::from("No inputs produced expected output"))
}

fn main() -> Result<()> {
    println!("{}", part1()?);
    println!("{}", part2()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2890696, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(8226, part2().unwrap());
    }
}
