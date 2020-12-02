use io::Result;

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

#[derive(PartialEq, Debug)]
pub struct PasswordCheck {
    password: String,
    required_char: char,
    min: u32,
    max: u32,
}

impl PasswordCheck {
    fn is_valid(&self) -> bool {
        let required_char_count: u32 = self
            .password
            .chars()
            .filter(|c| *c == self.required_char)
            .count() as u32;
        required_char_count >= self.min && required_char_count <= self.max
    }
}

impl Solver for Problem {
    type Input = Vec<PasswordCheck>;
    type Output = u32;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .map(|s| str_to_password_check(s))
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        input.iter().filter(|check| check.is_valid()).count() as u32
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        2
    }
}

fn str_to_password_check(s: String) -> PasswordCheck {
    // {min}-{max} {char}: {password}
    let first_split: Vec<&str> = s.split(':').collect();
    let pass = first_split[1].strip_prefix(" ").unwrap();
    let second_split: Vec<&str> = s.split(' ').collect();
    let required_char = second_split[1].chars().next().unwrap();
    let range_split: Vec<u32> = second_split[0]
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect();
    PasswordCheck {
        password: String::from(pass),
        required_char: required_char,
        min: range_split[0],
        max: range_split[1],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_password_check() {
        let input = "3-4 t: dttt";
        let expected = PasswordCheck {
            password: String::from("dttt"),
            required_char: 't',
            min: 3,
            max: 4,
        };
        assert_eq!(expected, str_to_password_check(String::from(input)));
    }

    #[test]
    fn test_password_check_is_valid() {
        let c1 = str_to_password_check(String::from("3-4 t: dttt"));
        let c2 = str_to_password_check(String::from("1-3 b: cdefg"));
        let c3 = str_to_password_check(String::from("2-9 c: ccccccccc"));
        assert!(c1.is_valid());
        assert!(!c2.is_valid());
        assert!(c3.is_valid());
    }
}
