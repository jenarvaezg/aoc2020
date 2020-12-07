use crate::solver::Solver;
use lazy_static::lazy_static;
use regex::Regex;

use std::{
    collections::BTreeMap,
    io::{self, BufRead, BufReader},
};

lazy_static! {
    static ref BAG_RE: Regex = Regex::new(r"(\d) (.*) bag").unwrap();
}

type BagMap = BTreeMap<String, Vec<BagContainment>>;

#[derive(Debug, PartialEq)]
pub struct BagContainment {
    bag: String,
    count: usize,
}

pub struct Problem;

impl Solver for Problem {
    type Input = BagMap;
    type Output = usize;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .fold(BTreeMap::new(), |mut acc, s| {
                let (k, v) = line_to_bags(&s);
                acc.insert(k, v);

                acc
            })
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        input
            .into_iter()
            .filter(|(bag, _x)| contains_color(input, bag, "shiny gold"))
            .count()
            - 1 // ignore shiny gold itself
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        bag_count(input, "shiny gold") - 1 // ignore shiny gold itself
    }
}

fn line_to_bags(s: &str) -> (String, Vec<BagContainment>) {
    let bags: Vec<&str> = s.trim().split(" bags contain ").collect();
    let other_bags = bags[1].split(", ").fold(Vec::new(), |mut acc, x| {
        if x != "no other bags." {
            let captures = BAG_RE.captures(x).unwrap();
            acc.push(BagContainment {
                count: captures[1].parse().unwrap(),
                bag: captures[2].to_string(),
            })
        }
        acc
    });

    (String::from(bags[0]), other_bags)
}

fn contains_color(bags: &BagMap, current: &str, color: &str) -> bool {
    if current == color {
        return true;
    }

    if let Some(containments) = bags.get(current) {
        return containments
            .iter()
            .any(|x| contains_color(bags, &x.bag, color));
    }
    false
}

fn bag_count(bags: &BagMap, current: &str) -> usize {
    bags[current]
        .iter()
        .map(|x| x.count * bag_count(bags, &x.bag))
        .sum::<usize>()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_bags() {
        let expected_0 = (
            String::from("light red bags"),
            vec![
                BagContainment {
                    count: 1,
                    bag: String::from("bright white"),
                },
                BagContainment {
                    count: 2,
                    bag: String::from("muted yellow"),
                },
            ],
        );
        let result_0 =
            line_to_bags("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        assert_eq!(expected_0.0, result_0.0);
        assert_eq!(expected_0.1[0], result_0.1[0]);
        assert_eq!(expected_0.1[1], result_0.1[1]);

        assert_eq!(0, line_to_bags("x contain no other bags.").1.len());
    }
}
