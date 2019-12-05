use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn parse_input() -> Result<(u32, u32), Box<dyn Error>> {
    let mut contents = String::new();
    File::open("../inputs/04-input.txt")?.read_to_string(&mut contents)?;
    let range = contents
        .trim()
        .split('-')
        .map(|n| n.parse().expect("Should be a number"))
        .collect::<Vec<_>>();
    Ok((range[0], range[1]))
}

struct PasswordGenerator {
    max: u32,
    current: u32,
    strict_adjacent_digits: bool,
}

impl PasswordGenerator {
    fn new(min: u32, max: u32, strict_adjacent_digits: bool) -> Self {
        let current = min - 1;
        PasswordGenerator {
            max,
            current,
            strict_adjacent_digits,
        }
    }

    fn is_current_valid(&self) -> bool {
        let mut num = self.current;
        let mut prev_digit = 42; // lol
        let mut repeated_count = 1;
        let mut found_two = false;
        while num > 0 {
            let rem = num % 10;

            if rem > prev_digit {
                return false;
            }

            if prev_digit == rem {
                repeated_count += 1;
            } else if self.strict_adjacent_digits {
                if repeated_count == 2 {
                    found_two = true;
                }
                repeated_count = 1;
            }

            prev_digit = rem;
            num /= 10;
        }

        if self.strict_adjacent_digits {
            found_two || repeated_count == 2
        } else {
            repeated_count >= 2
        }
    }
}

impl Iterator for PasswordGenerator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current += 1;
            if self.current > self.max {
                return None;
            }
            if self.is_current_valid() {
                return Some(self.current);
            }
        }
    }
}

fn part1() -> Result<usize, Box<dyn Error>> {
    let (min, max) = parse_input()?;
    let pass_gen = PasswordGenerator::new(min, max, false);
    Ok(pass_gen.count())
}

fn part2() -> Result<usize, Box<dyn Error>> {
    let (min, max) = parse_input()?;
    let pass_gen = PasswordGenerator::new(min, max, true);
    Ok(pass_gen.count())
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
    fn test_valid_password() {
        let valid = 111111;
        let mut pass_gen = PasswordGenerator::new(valid, valid, false);
        assert_eq!(Some(valid), pass_gen.next());
    }

    #[test]
    fn test_invalid_decreasing_password() {
        let invalid = 223450;
        let mut pass_gen = PasswordGenerator::new(invalid, invalid, false);
        assert_eq!(None, pass_gen.next());
    }

    #[test]
    fn test_invalid_no_repeat_password() {
        let invalid = 123789;
        let mut pass_gen = PasswordGenerator::new(invalid, invalid, false);
        assert_eq!(None, pass_gen.next());
    }

    #[test]
    fn test_valid_password_strict() {
        let valid = 112233;
        let mut pass_gen = PasswordGenerator::new(valid, valid, true);
        assert_eq!(Some(valid), pass_gen.next());
    }

    #[test]
    fn test_valid_password_strict_group_at_beginning() {
        let valid = 223333;
        let mut pass_gen = PasswordGenerator::new(valid, valid, true);
        assert_eq!(Some(valid), pass_gen.next());
    }

    #[test]
    fn test_valid_password_strict_group_at_end() {
        let valid = 111122;
        let mut pass_gen = PasswordGenerator::new(valid, valid, true);
        assert_eq!(Some(valid), pass_gen.next());
    }

    #[test]
    fn test_invalid_password_strict() {
        let invalid = 123444;
        let mut pass_gen = PasswordGenerator::new(invalid, invalid, true);
        assert_eq!(None, pass_gen.next());
    }

    #[test]
    fn test_part1() {
        assert_eq!(481, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(299, part2().unwrap());
    }
}
