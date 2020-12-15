use io::Result;

use crate::solver::Solver;
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
};

pub struct Problem;

type SayMap = HashMap<u64, Vec<usize>>;

impl Solver for Problem {
    type Input = Vec<u64>;
    type Output = u64;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        let l = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .next()
            .unwrap();

        l.split(',').map(|x| x.parse().unwrap()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        let (mut say_map, mut last) = init_say_map(input);

        get_number_at(2000, &mut say_map, &mut last, input.len() as u64 + 1)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let (mut say_map, mut last) = init_say_map(input);

        get_number_at(30000000, &mut say_map, &mut last, input.len() as u64 + 1)
    }
}

fn init_say_map(input: &Vec<u64>) -> (SayMap, u64) {
    let mut say_map = SayMap::new();
    let mut last = 0;
    for (turn, x) in input.into_iter().cloned().enumerate() {
        say_map
            .entry(x)
            .or_insert(Vec::<usize>::new())
            .push(turn + 1);
        last = x;
    }
    (say_map, last)
}

fn get_number_at(target: u64, say_map: &mut SayMap, last: &mut u64, offset: u64) -> u64 {
    for turn in offset..=target {
        let last_says = say_map.get(&last).unwrap();
        if last_says.len() == 1 {
            *last = 0;
        } else {
            *last = (last_says[last_says.len() - 1] - last_says[last_says.len() - 2]) as u64;
        }

        say_map
            .entry(*last)
            .or_insert(Vec::<usize>::new())
            .push(turn as usize);
    }
    *last
}
