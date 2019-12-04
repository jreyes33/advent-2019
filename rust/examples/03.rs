use crate::Direction::*;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;

type WireInput = Vec<Segment>;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Segment {
    direction: Direction,
    length: u32,
}

#[derive(Debug, Eq)]
struct Point {
    x: i32,
    y: i32,
    i: u32,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Wire {
    points: HashSet<Point>,
}

impl Wire {
    fn new(input: &[Segment]) -> Self {
        let mut points = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        let mut i = 0;
        for s in input {
            for _ in 0..s.length {
                i += 1;
                match s.direction {
                    Up => y += 1,
                    Down => y -= 1,
                    Left => x -= 1,
                    Right => x += 1,
                }
                points.insert(Point { x, y, i });
            }
        }
        Wire { points }
    }

    fn closest_intersection_distance(&self, other: &Self) -> u32 {
        self.points
            .intersection(&other.points)
            .map(|p| p.x.abs() + p.y.abs())
            .min()
            .expect("Should have a minimum") as u32
    }

    fn least_steps_intersection_distance(&self, other: &Self) -> u32 {
        self.points
            .intersection(&other.points)
            .map(|p| self.points.get(p).unwrap().i + other.points.get(p).unwrap().i)
            .min()
            .expect("Should have a minimum")
    }
}

fn parse_input() -> Result<(WireInput, WireInput), Box<dyn Error>> {
    let mut contents = String::new();
    File::open("../inputs/03-input.txt")?.read_to_string(&mut contents)?;
    let mut lines = contents.lines();
    let input_a = parse_line(lines.next().ok_or("Line missing")?);
    let input_b = parse_line(lines.next().ok_or("Line missing")?);
    Ok((input_a, input_b))
}

fn parse_line(line: &str) -> WireInput {
    line.split(',')
        .map(|s| match (s.get(0..1), s.get(1..).unwrap().parse()) {
            (Some("U"), Ok(length)) => Segment {
                direction: Up,
                length,
            },
            (Some("D"), Ok(length)) => Segment {
                direction: Down,
                length,
            },
            (Some("L"), Ok(length)) => Segment {
                direction: Left,
                length,
            },
            (Some("R"), Ok(length)) => Segment {
                direction: Right,
                length,
            },
            _ => panic!("Invalid input: {}", s),
        })
        .collect()
}

fn part1() -> Result<u32, Box<dyn Error>> {
    let (input_a, input_b) = parse_input()?;
    let wire_a = Wire::new(&input_a);
    let wire_b = Wire::new(&input_b);
    Ok(wire_a.closest_intersection_distance(&wire_b))
}

fn part2() -> Result<u32, Box<dyn Error>> {
    let (input_a, input_b) = parse_input()?;
    let wire_a = Wire::new(&input_a);
    let wire_b = Wire::new(&input_b);
    Ok(wire_a.least_steps_intersection_distance(&wire_b))
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
        assert_eq!(1064, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(25676, part2().unwrap());
    }
}
