use io::Result;

use crate::solver::Solver;
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u64>;
    type Output = u64;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        let mut input: Vec<u64> = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .flat_map(|l| l.parse())
            .collect();

        input.sort();
        input.push(*input.iter().max().unwrap() + 3);
        let mut other = vec![0];

        other.extend(input);
        other
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        let diffs: HashMap<u64, u64> = input.windows(2).fold(HashMap::new(), |mut acc, x| {
            let diff = x[1] - x[0];
            let count = acc.get(&diff).unwrap_or(&0) + 1;
            acc.insert(diff, count);
            acc
        });
        (diffs.get(&1).unwrap()) * (diffs.get(&3).unwrap())
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let mut counts = vec![0u64; input.len()];
        counts[0] = 1;
        for i in 1..counts.len() {
            for j in i.saturating_sub(3)..i {
                if input[i] - input[j] <= 3 {
                    counts[i] += counts[j]
                }
            }
        }
        *counts.last().unwrap()
    }
}
