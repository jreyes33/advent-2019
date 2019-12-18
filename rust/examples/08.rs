use advent_2019::Result;
use std::fs::File;
use std::io::prelude::*;

fn parse_input() -> Result<Vec<u32>> {
    let mut contents = String::new();
    File::open("../inputs/08-input.txt")?.read_to_string(&mut contents)?;
    let numbers = contents
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Should be a number"))
        .collect();
    Ok(numbers)
}

fn count_digits(layer: &[&[u32]], digit: u32) -> u32 {
    layer
        .iter()
        .copied()
        .flatten()
        .filter(|x| *x == &digit)
        .count() as u32
}

fn part1() -> Result<u32> {
    let colors = parse_input()?;
    let rows: Vec<_> = colors.chunks_exact(25).collect();
    let layers = rows.chunks_exact(6);
    let less_zeroes = layers
        .min_by(|x, y| count_digits(x, 0).cmp(&count_digits(y, 0)))
        .ok_or("Should have a min")?;
    Ok(count_digits(less_zeroes, 1) * count_digits(less_zeroes, 2))
}

fn part2() -> Result<()> {
    let colors = parse_input()?;
    let rows: Vec<_> = colors.chunks_exact(25).collect();
    let layers: Vec<_> = rows.chunks_exact(6).collect();
    for i in 0..6 {
        for j in 0..25 {
            for layer in layers.iter() {
                if layer[i][j] != 2 {
                    print!("{}", if layer[i][j] == 0 { ' ' } else { '*' });
                    break;
                }
            }
        }
        println!();
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", part1()?);
    part2()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1950, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        part2().unwrap();
    }
}
