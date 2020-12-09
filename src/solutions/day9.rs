use io::Result;

use crate::solver::Solver;
use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<isize>;
    type Output = isize;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .flat_map(|l| l.parse())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        let preamble_count = 25;
        input
            .windows(preamble_count)
            .filter_map(|window| {
                find_pair(&window[0..preamble_count].to_vec(), window[preamble_count])
            })
            .next()
            .unwrap()
            .iter()
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let target = 85848519;
        let max_pos = input.iter().position(|&x| x == target).unwrap();

        for n in 2..max_pos {
            for window in input[..max_pos].windows(n) {
                if window.iter().sum::<isize>() == target {
                    return window.iter().min().unwrap() + window.iter().max().unwrap();
                }
            }
        }

        panic!("Not found!");
    }
}

fn find_pair(input: &Vec<isize>, target: isize) -> Option<Vec<isize>> {
    let set: HashSet<isize> = input.iter().cloned().collect();
    let value = set.iter().find(|&x| set.get(&(target - x)).is_some());

    match value {
        Some(x) => Some(vec![*x, target - x]),
        None => None,
    }
}
