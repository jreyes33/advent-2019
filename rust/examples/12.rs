use advent_2019::Result;
use std::cmp::Ordering::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn abs_sum(&self) -> u32 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u32
    }
}

#[derive(PartialEq)]
struct Body {
    pos: Point,
    vel: Point,
}

impl Body {
    fn new(x: i32, y: i32, z: i32) -> Self {
        let pos = Point { x, y, z };
        let vel = Point { x: 0, y: 0, z: 0 };
        Body { pos, vel }
    }

    fn next_vel(&self, others: &[Self]) -> Point {
        let mut vel = self.vel.clone();
        for other in others {
            if other == self {
                continue;
            }
            match self.pos.x.cmp(&other.pos.x) {
                Less => vel.x += 1,
                Greater => vel.x -= 1,
                _ => (),
            }
            match self.pos.y.cmp(&other.pos.y) {
                Less => vel.y += 1,
                Greater => vel.y -= 1,
                _ => (),
            }
            match self.pos.z.cmp(&other.pos.z) {
                Less => vel.z += 1,
                Greater => vel.z -= 1,
                _ => (),
            }
        }
        vel
    }

    fn apply_vel(&mut self, vel: &Point) {
        self.pos.x += vel.x;
        self.pos.y += vel.y;
        self.pos.z += vel.z;
        self.vel = vel.clone();
    }
}

#[rustfmt::skip]
fn parse_input() -> Result<Vec<Body>> {
    let buf_reader = BufReader::new(File::open("../inputs/12-input.txt")?);
    let mut moons = vec![];
    for line in buf_reader.lines() {
        let l = line?;
        let mut splitted = l.split('=').skip(1);
        let x = splitted.next().ok_or("no x")?.split(',').next().ok_or("no x")?.parse()?;
        let y = splitted.next().ok_or("no y")?.split(',').next().ok_or("no y")?.parse()?;
        let z = splitted.next().ok_or("no z")?.split('>').next().ok_or("no z")?.parse()?;
        let moon = Body::new(x, y, z);
        moons.push(moon);
    }
    Ok(moons)
}

fn simulate_steps(bodies: &mut [Body], n: u32) {
    for _ in 0..n {
        let next_vels: Vec<_> = bodies.iter().map(|b| b.next_vel(bodies)).collect();
        for (i, body) in bodies.iter_mut().enumerate() {
            body.apply_vel(&next_vels[i]);
        }
    }
}

fn total_energy(bodies: &[Body]) -> u32 {
    bodies
        .iter()
        .fold(0, |acc, body| acc + body.pos.abs_sum() * body.vel.abs_sum())
}

// Bodies' Positions and Velocities: When Do They Repeat? Do They Ever Repeat?? Let's Find Out!
#[rustfmt::skip]
fn bpav_wdtr_dter_lfo(bodies: &mut [Body]) -> Option<usize> {
    let mut x_period = None;
    let mut y_period = None;
    let mut z_period = None;
    let mut x_seen = HashSet::new();
    let mut y_seen = HashSet::new();
    let mut z_seen = HashSet::new();
    for i in 0.. {
        check_period(&mut x_period, &mut x_seen, bodies, |b| (b.pos.x, b.vel.x), i);
        check_period(&mut y_period, &mut y_seen, bodies, |b| (b.pos.y, b.vel.y), i);
        check_period(&mut z_period, &mut z_seen, bodies, |b| (b.pos.z, b.vel.z), i);
        if x_period.is_some() && y_period.is_some() && z_period.is_some() {
            break;
        }
        simulate_steps(bodies, 1);
    }
    bruteforce_lcm(&[x_period.unwrap(), y_period.unwrap(), z_period.unwrap()])
}

// Just to make it DRY™
fn check_period(
    period: &mut Option<usize>,
    seen: &mut HashSet<Vec<(i32, i32)>>,
    bodies: &mut [Body],
    map_fn: fn(&Body) -> (i32, i32),
    i: usize,
) {
    if period.is_none() {
        let state: Vec<_> = bodies.iter().map(map_fn).collect();
        if !seen.contains(&state) {
            seen.insert(state);
        } else {
            *period = Some(i);
        }
    }
}

fn bruteforce_lcm(nums: &[usize]) -> Option<usize> {
    let min = nums.iter().min().expect("Should have a min");
    for i in (*min..).step_by(*min) {
        if nums.iter().all(|x| i % x == 0) {
            return Some(i);
        }
    }
    None
}

fn part1() -> Result<u32> {
    let mut moons = parse_input()?;
    simulate_steps(&mut moons, 1000);
    Ok(total_energy(&moons))
}

fn part2() -> Result<usize> {
    let mut moons = parse_input()?;
    let res = bpav_wdtr_dter_lfo(&mut moons).ok_or("They don't repeat")?;
    Ok(res)
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
    fn test_simulate_steps() {
        let mut moons = vec![];
        moons.push(Body::new(-1, 0, 2));
        moons.push(Body::new(2, -10, -7));
        moons.push(Body::new(4, -8, 8));
        moons.push(Body::new(3, 5, -1));
        simulate_steps(&mut moons, 10);
        assert_eq!(Point { x: 2, y: 1, z: -3 }, moons[0].pos);
        assert_eq!(Point { x: -3, y: -2, z: 1 }, moons[0].vel);
        assert_eq!(Point { x: 1, y: -8, z: 0 }, moons[1].pos);
        assert_eq!(Point { x: -1, y: 1, z: 3 }, moons[1].vel);
        assert_eq!(Point { x: 3, y: -6, z: 1 }, moons[2].pos);
        assert_eq!(Point { x: 3, y: 2, z: -3 }, moons[2].vel);
        assert_eq!(Point { x: 2, y: 0, z: 4 }, moons[3].pos);
        assert_eq!(Point { x: 1, y: -1, z: -1 }, moons[3].vel);
    }

    #[test]
    fn test_total_energy() {
        let mut moons = vec![];
        moons.push(Body::new(-1, 0, 2));
        moons.push(Body::new(2, -10, -7));
        moons.push(Body::new(4, -8, 8));
        moons.push(Body::new(3, 5, -1));
        simulate_steps(&mut moons, 10);
        assert_eq!(179, total_energy(&moons));
    }

    #[test]
    fn test_bpav_wdtr_dter_lfo() {
        let mut moons = vec![];
        moons.push(Body::new(-1, 0, 2));
        moons.push(Body::new(2, -10, -7));
        moons.push(Body::new(4, -8, 8));
        moons.push(Body::new(3, 5, -1));
        assert_eq!(2772, bpav_wdtr_dter_lfo(&mut moons).unwrap());
    }

    #[test]
    fn test_part1() {
        assert_eq!(9876, part1().unwrap());
    }

    #[test]
    #[ignore]
    fn test_part2() {
        // Too slow to run in CI
        assert_eq!(307_043_147_758_488, part2().unwrap());
    }
}
