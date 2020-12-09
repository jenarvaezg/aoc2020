use io::Result;

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};
use std::{cmp::Ordering, collections::HashSet};

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
            .windows(preamble_count + 1)
            .filter(|window| {
                find_pair(&window[0..preamble_count].to_vec(), window[preamble_count]).is_none()
            })
            .next()
            .unwrap()[preamble_count]
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let (mut left, mut right, mut sum) = (0, 0, 0);
        let target = 85848519;
        loop {
            match sum.cmp(&target) {
                Ordering::Greater => {
                    sum -= input[left];
                    left += 1
                }
                Ordering::Less => {
                    sum += input[right];
                    right += 1
                }
                Ordering::Equal => {
                    let window = &input[left..right];
                    return window.iter().min().unwrap() + window.iter().max().unwrap();
                }
            }
        }
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
