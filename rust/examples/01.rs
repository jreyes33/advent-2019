use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_input() -> Result<Vec<u32>, Box<dyn Error>> {
    let buf_reader = BufReader::new(File::open("../inputs/01-input.txt")?);
    let mut numbers = vec![];
    for line in buf_reader.lines() {
        numbers.push(line?.parse()?);
    }
    Ok(numbers)
}

fn fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn better_fuel(mass: u32) -> u32 {
    let mut result = 0;
    let mut m = mass;
    loop {
        let f = fuel(m);
        if f == 0 {
            break;
        }
        result += f;
        m = f;
    }
    result
}

fn part1() -> Result<u32, Box<dyn Error>> {
    let result = parse_input()?.iter().map(|m| fuel(*m)).sum();
    Ok(result)
}

fn part2() -> Result<u32, Box<dyn Error>> {
    let result = parse_input()?.iter().map(|m| better_fuel(*m)).sum();
    Ok(result)
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
    fn test_fuel() {
        assert_eq!(0, fuel(3));
        assert_eq!(2, fuel(12));
        assert_eq!(2, fuel(14));
        assert_eq!(654, fuel(1969));
        assert_eq!(33583, fuel(100756));
    }

    #[test]
    fn test_better_fuel() {
        assert_eq!(2, better_fuel(14));
        assert_eq!(966, better_fuel(1969));
        assert_eq!(50346, better_fuel(100756));
    }

    #[test]
    fn test_part1() {
        assert_eq!(3423279, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(5132018, part2().unwrap());
    }
}
