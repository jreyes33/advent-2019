use self::Dir::*;
use advent_2019::intcode::{parse_input, Execution};
use advent_2019::Result;
use std::collections::HashMap;

enum Dir {
    Up,
    Right,
    Down,
    Left,
}

const DIRS: [Dir; 4] = [Up, Right, Down, Left];

fn paint_robot_paint(exec: &mut Execution) -> HashMap<(i32, i32), i64> {
    let mut x = 0;
    let mut y = 0;
    let mut color;
    let mut dir_idx: usize = 0;
    let mut painted = HashMap::new();
    while let Ok(output) = exec.run() {
        color = match output.get(0) {
            Some(c) => *c,
            None => break,
        };
        painted.insert((x, y), color);
        dir_idx = match output.get(1) {
            Some(0) => dir_idx.checked_sub(1).unwrap_or(DIRS.len() - 1),
            Some(1) => (dir_idx + 1) % DIRS.len(),
            None | Some(_) => break,
        };
        match DIRS.get(dir_idx) {
            Some(Up) => y -= 1,
            Some(Down) => y += 1,
            Some(Left) => x -= 1,
            Some(Right) => x += 1,
            None => break,
        }
        let next_input = painted.get(&(x, y)).unwrap_or(&0);
        exec.add_input(*next_input);
    }
    painted
}

fn part1() -> Result<usize> {
    let program = parse_input("../inputs/11-input.txt")?;
    let mut exec = Execution::new(program, vec![0]);
    let painted = paint_robot_paint(&mut exec);
    Ok(painted.len())
}

fn part2() -> Result<()> {
    let program = parse_input("../inputs/11-input.txt")?;
    let mut exec = Execution::new(program, vec![1]);
    let painted = paint_robot_paint(&mut exec);
    let mut max_x = 0;
    let mut max_y = 0;
    for &(x, y) in painted.keys() {
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }
    for i in 0..=max_y {
        for j in 0..=max_x {
            let dot = match painted.get(&(j, i)) {
                Some(1) => '*',
                Some(_) | None => ' ',
            };
            print!("{}", dot);
        }
        println!();
    }
    dbg!(max_x, max_y, painted.len());
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
        assert_eq!(1907, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        part2().unwrap();
    }
}
