use crate::passport::*;
use crate::solver::Solver;

use std::io::{self, BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output = usize;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        let mut buf_r = BufReader::new(r);
        let mut buffer = String::new();
        buf_r.read_to_string(&mut buffer).unwrap();

        buffer.split("\n\n").map(|s| String::from(s)).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        input
            .into_iter()
            .filter(|&pass| passport_has_fields(pass))
            .count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        input
            .into_iter()
            .filter(|&pass| pass.parse::<Passport>().is_ok())
            .count()
    }
}

fn passport_has_fields(pass: &str) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_fields
        .iter()
        .filter(|&field| pass.contains(field))
        .count()
        == required_fields.len()
}
