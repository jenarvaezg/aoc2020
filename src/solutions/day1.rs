use io::Result;

use crate::solver::Solver;
use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .flat_map(|l| l.parse())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        let values = find_pair(input, 2020).unwrap();
        values.into_iter().fold(1, |a, b| a * b)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let values = find_three(input, 2020);
        values.into_iter().fold(1, |a, b| a * b)
    }
}

fn find_pair(input: &Vec<i32>, target: i32) -> Option<Vec<i32>> {
    let set: HashSet<i32> = input.iter().cloned().collect();
    let value = set.iter().find(|&x| set.get(&(target - x)).is_some());

    match value {
        Some(x) => Some(vec![*x, target - x]),
        None => None,
    }
}

fn find_three(input: &Vec<i32>, target: i32) -> Vec<i32> {
    let set: HashSet<i32> = input.iter().cloned().collect();
    for a in &set {
        let subset: Vec<i32> = input.iter().cloned().filter(|&b| b != *a).collect();
        let pair = find_pair(&subset, target - a);
        if let Some(pair) = pair {
            return pair.into_iter().chain(vec![*a]).collect();
        }
    }

    panic!("Not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_from_vec(input: &Vec<i32>) -> HashSet<i32> {
        input.iter().cloned().collect()
    }

    #[test]
    fn test_fin_pair() {
        let result = set_from_vec(&find_pair(&vec![1, 2, 3], 5).unwrap());
        assert_eq!(result, set_from_vec(&vec![2, 3]));
    }

    #[test]
    fn test_find_three() {
        let example: Vec<i32> = vec![1721, 979, 366, 299, 675, 1456];
        let result = set_from_vec(&find_three(&example, 2020));
        assert_eq!(result, set_from_vec(&vec![979, 366, 675]));
    }
}
