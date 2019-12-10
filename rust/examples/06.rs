use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Orbits = HashMap<String, String>;

fn parse_input(path: &str) -> Result<Orbits, Box<dyn Error>> {
    let buf_reader = BufReader::new(File::open(path)?);
    let mut orbits = HashMap::new();
    for line in buf_reader.lines() {
        let l = line?;
        let mut splitted = l.split(')');
        let orbited = splitted.next().ok_or("No orbited object")?.to_string();
        let orbiting = splitted.next().ok_or("No orbiting object")?.to_string();
        orbits.insert(orbiting, orbited);
    }
    Ok(orbits)
}

fn total_orbits(orbits: &Orbits) -> Result<u32, Box<dyn Error>> {
    let mut total = orbits.len() as u32;
    for orbited in orbits.values() {
        let mut current = orbited;
        while let Some(orbited) = orbits.get(current) {
            total += 1;
            current = orbited;
        }
    }
    Ok(total)
}

fn min_transfers(orbits: &Orbits, from: &str, to: &str) -> Result<u32, Box<dyn Error>> {
    let mut ancestors: HashMap<_, u32> = HashMap::new();
    let mut total = 0;
    let mut current = from;
    while let Some(orbited) = orbits.get(current) {
        total += 1;
        current = orbited;
        ancestors.insert(current, total);
    }

    total = 0;
    current = to;
    while let Some(orbited) = orbits.get(current) {
        total += 1;
        current = orbited;
        if let Some(other_steps) = ancestors.get(current) {
            return Ok(total + other_steps - 2);
        }
    }
    Err(From::from("No route found, weird"))
}

fn part1() -> Result<u32, Box<dyn Error>> {
    let orbits = parse_input("../inputs/06-input.txt")?;
    Ok(total_orbits(&orbits)?)
}

fn part2() -> Result<u32, Box<dyn Error>> {
    let orbits = parse_input("../inputs/06-input.txt")?;
    Ok(min_transfers(&orbits, "YOU", "SAN")?)
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
    fn test_total_orbits() {
        let orbits = parse_input("../inputs/06-example.txt").unwrap();
        assert_eq!(54, total_orbits(&orbits).unwrap());
    }

    #[test]
    fn test_min_transfers() {
        let orbits = parse_input("../inputs/06-example.txt").unwrap();
        assert_eq!(4, min_transfers(&orbits, "YOU", "SAN").unwrap());
    }

    #[test]
    fn test_part1() {
        assert_eq!(322508, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(496, part2().unwrap());
    }
}
