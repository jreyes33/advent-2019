use advent_2019::intcode::{compute, parse_input, Execution};
use advent_2019::Result;

// ¯\_(ツ)_/¯ https://rosettacode.org/wiki/Permutations#Rust
fn permutations(min: usize, size: usize) -> Permutations {
    Permutations {
        idxs: (min..min + size).collect(),
        swaps: vec![0; size],
        i: 0,
    }
}

struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() {
                    return None;
                }
                if self.swaps[self.i] < self.i {
                    break;
                }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}

fn max_amp_signal(program: Vec<i64>) -> Result<(i64, Vec<usize>)> {
    let mut max_output = std::i64::MIN;
    let mut max_phase_settings = vec![];
    for phase_settings in permutations(0, 5) {
        let mut outputs = vec![0];
        for phase_setting in &phase_settings {
            let inputs = vec![*phase_setting as i64, *outputs.get(0).ok_or("no output")?];
            outputs = compute(program.clone(), inputs)?;
        }
        let output = outputs.get(0).ok_or("no output")?;
        if *output > max_output {
            max_output = *output;
            max_phase_settings = phase_settings;
        }
    }
    Ok((max_output, max_phase_settings))
}

fn max_feedback_signal(program: Vec<i64>) -> Result<(i64, Vec<usize>)> {
    let mut max_output = std::i64::MIN;
    let mut max_phase_settings = vec![];
    for phase_settings in permutations(5, 5) {
        let mut execs: Vec<_> = phase_settings
            .iter()
            .map(|phase_setting| Execution::new(program.clone(), vec![*phase_setting as i64]))
            .collect();
        let mut outputs = vec![0];
        for i in (0..execs.len()).cycle() {
            execs[i].add_input(*outputs.get(0).ok_or("no output")?);
            let run_outputs = execs[i].run()?;
            if run_outputs.is_empty() {
                break; // It halted.
            } else {
                outputs = run_outputs;
            }
        }
        let output = outputs.get(0).ok_or("no output")?;
        if *output > max_output {
            max_output = *output;
            max_phase_settings = phase_settings;
        }
    }
    Ok((max_output, max_phase_settings))
}

fn part1() -> Result<i64> {
    let program = parse_input("../inputs/07-input.txt")?;
    let (max_output, _max_phase_settings) = max_amp_signal(program)?;
    Ok(max_output)
}

fn part2() -> Result<i64> {
    let program = parse_input("../inputs/07-input.txt")?;
    let (max_output, _max_phase_settings) = max_feedback_signal(program)?;
    Ok(max_output)
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
    fn test_max_signal() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(
            (43210, vec![4, 3, 2, 1, 0]),
            max_amp_signal(program).unwrap()
        );
    }

    #[test]
    fn test_max_feedback_signal() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(
            (139629729, vec![9, 8, 7, 6, 5]),
            max_feedback_signal(program).unwrap()
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(46248, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(54163586, part2().unwrap());
    }
}
