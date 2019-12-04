use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn parse_input() -> Result<String, Box<dyn Error>> {
    let mut contents = String::new();
    File::open("../inputs/02-input.txt")?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn part1() -> Result<u32, Box<dyn Error>> {
    parse_input()?;
    Ok(1)
}

fn part2() -> Result<u32, Box<dyn Error>> {
    Ok(2)
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
    fn test_part1() {
        assert_eq!(1, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(2, part2().unwrap());
    }
}
