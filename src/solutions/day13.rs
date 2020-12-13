use io::Result;

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = (i64, Vec<i64>);
    type Output = i64;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        let lines: Vec<String> = BufReader::new(r).lines().filter_map(Result::ok).collect();

        let first: i64 = lines[0].parse().unwrap();
        let second: Vec<i64> = lines[1]
            .split(",")
            .map(|x| x.parse().unwrap_or(0))
            .collect();

        (first, second)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        let (initial_time, buses) = input;
        let buses_to_use: Vec<i64> = buses.into_iter().map(|&x| x).filter(|&x| x != 0).collect();
        for i in 0..*buses_to_use.iter().min().unwrap() {
            match buses_to_use.iter().find(|&x| (initial_time + i) % x == 0) {
                Some(x) => {
                    return i * x;
                }
                None => {}
            }
        }
        unreachable!();
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let (_, buses) = input;
        let buses_and_offsets: Vec<(i64, i64)> = buses
            .into_iter()
            .enumerate()
            .filter(|(_, &x)| x != 0)
            .map(|(i, &x)| (i as i64, x))
            .collect();

        let (mut value, mut product) = buses_and_offsets[0];
        for &(i, bus) in buses_and_offsets.iter().skip(1) {
            while (value + i) % bus != 0 {
                value += product;
            }
            product *= bus;
        }
        value
    }
}
