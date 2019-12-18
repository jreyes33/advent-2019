use advent_2019::intcode::{compute, parse_input};
use advent_2019::Result;

fn part1() -> Result<i64> {
    let program = parse_input("../inputs/05-input.txt")?;
    let outputs = compute(program, vec![1])?;
    Ok(*outputs.last().ok_or("No outputs")?)
}

fn part2() -> Result<i64> {
    let program = parse_input("../inputs/05-input.txt")?;
    let outputs = compute(program, vec![5])?;
    Ok(*outputs.last().ok_or("No outputs")?)
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
        assert_eq!(7286649, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(15724522, part2().unwrap());
    }
}
