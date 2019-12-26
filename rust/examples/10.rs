use advent_2019::Result;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;

type Point = (i32, i32);

#[derive(Debug, Copy, Clone)]
struct Slope {
    delta_x: f64,
    delta_y: f64,
}

impl Slope {
    fn val(&self) -> f64 {
        self.delta_y / self.delta_x
    }

    fn angle(&self) -> f64 {
        if self.delta_x == 0.0 {
            if self.delta_y > 0.0 {
                return PI;
            } else {
                return 0.0;
            }
        }
        let adjustment = if self.delta_x > 0.0 {
            PI / 2.0
        } else {
            1.5 * PI
        };
        self.val().atan() + adjustment
    }
}

impl Hash for Slope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.angle().to_string().hash(state);
    }
}

impl PartialEq for Slope {
    fn eq(&self, other: &Self) -> bool {
        self.angle() == other.angle()
    }
}

impl Eq for Slope {}

impl Ord for Slope {
    fn cmp(&self, other: &Self) -> Ordering {
        self.angle().partial_cmp(&other.angle()).unwrap()
    }
}

impl PartialOrd for Slope {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(path: &str) -> Result<Vec<Point>> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    Ok(parse_asteroids(&contents))
}

fn parse_asteroids(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .map(|(j, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(i, _)| (i as i32, j as i32))
        })
        .flatten()
        .collect()
}

fn best_loc(asteroids: &[Point]) -> (Point, usize) {
    let best_asteroid = asteroids
        .iter()
        .max_by(|a, b| {
            calc_others(**a, &asteroids)
                .len()
                .cmp(&calc_others(**b, &asteroids).len())
        })
        .expect("Should have a max");
    (
        *best_asteroid,
        calc_others(*best_asteroid, &asteroids).len(),
    )
}

fn calc_others(asteroid: Point, others: &[Point]) -> HashMap<Slope, Vec<Point>> {
    let mut blocked = HashMap::new();
    for (x, y) in others {
        if *x == asteroid.0 && *y == asteroid.1 {
            continue;
        }
        let slope = Slope {
            delta_x: (x - asteroid.0) as f64,
            delta_y: (y - asteroid.1) as f64,
        };
        let entry = blocked.entry(slope).or_insert_with(|| vec![]);
        entry.push((*x, *y));
    }
    blocked
}

fn nth_vaporized(n: usize, base: Point, asteroids: &[Point]) -> Point {
    let mut others = calc_others(base, asteroids);
    for (_, v) in others.iter_mut() {
        v.sort_by(|a, b| distance(base, *b).cmp(&distance(base, *a)));
    }
    let mut others_entries: Vec<(Slope, Vec<Point>)> = others.into_iter().collect();
    others_entries.sort_by_key(|(slope, _)| *slope);
    let mut i = 0;
    let mut count = 0;
    let mut res = (0, 0);
    while count < n {
        if let Some(popped) = others_entries.get_mut(i).unwrap().1.pop() {
            count += 1;
            res = popped;
        }
        i = (i + 1) % others_entries.len();
    }
    res
}

fn distance(from: Point, to: Point) -> i32 {
    (from.0 - to.0).abs() + (from.1 - from.0).abs()
}

fn part1() -> Result<usize> {
    let asteroids = parse_input("../inputs/10-input.txt")?;
    Ok(best_loc(&asteroids).1)
}

fn part2() -> Result<i32> {
    let asteroids = parse_input("../inputs/10-input.txt")?;
    let base = (25, 31);
    let res = nth_vaporized(200, base, &asteroids);
    Ok(res.0 * 100 + res.1)
}

fn main() -> Result<()> {
    println!("{:?}", part1()?);
    println!("{:?}", part2()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_loc() {
        let input = ".#..#\n.....\n#####\n....#\n...##";
        assert_eq!(((3, 4), 8), best_loc(&parse_asteroids(input)));
    }

    #[test]
    fn test_part1() {
        assert_eq!(329, part1().unwrap());
    }

    #[test]
    fn test_nth_vaporized() {
        let asteroids = parse_input("../inputs/10-example.txt").unwrap();
        let base = (11, 13);
        assert_eq!((11, 12), nth_vaporized(1, base, &asteroids));
        assert_eq!((10, 16), nth_vaporized(100, base, &asteroids));
        assert_eq!((8, 2), nth_vaporized(200, base, &asteroids));
        assert_eq!((11, 1), nth_vaporized(299, base, &asteroids));
    }

    #[test]
    fn test_part2() {
        assert_eq!(512, part2().unwrap());
    }
}
