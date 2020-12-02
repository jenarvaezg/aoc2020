use io::Result;
extern crate regex;

use regex::Regex;

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
    fn from_str(s: String) -> Self {
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

    fn from_str_regex(s: String) -> Self {
        // TODO
        PasswordCheck {
            password: String::from("a"),
            required_char: 'a',
            min: 1,
            max: 2,
        }
    }

    fn is_valid(&self) -> bool {
        let required_char_count: u32 = self
            .password
            .chars()
            .filter(|c| *c == self.required_char)
            .count() as u32;
        required_char_count >= self.min && required_char_count <= self.max
    }

    fn is_valid_2(&self) -> bool {
        self.password
            .chars()
            .enumerate()
            .filter(|e| {
                (e.0 as u32 == self.max - 1 || e.0 as u32 == self.min - 1)
                    && e.1 == self.required_char
            })
            .count()
            == 1
    }
}

impl Solver for Problem {
    type Input = Vec<PasswordCheck>;
    type Output = u32;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .map(|s| PasswordCheck::from_str(s))
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        input.iter().filter(|check| check.is_valid()).count() as u32
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        input.iter().filter(|check| check.is_valid_2()).count() as u32
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
        assert_eq!(expected, PasswordCheck::from_str(String::from(input)));
    }

    #[test]
    fn test_password_check_is_valid() {
        let c1 = PasswordCheck::from_str(String::from("3-4 t: dttt"));
        let c2 = PasswordCheck::from_str(String::from("1-3 b: cdefg"));
        let c3 = PasswordCheck::from_str(String::from("2-9 c: ccccccccc"));
        assert!(c1.is_valid());
        assert!(!c2.is_valid());
        assert!(c3.is_valid());
    }

    #[test]
    fn test_password_check_is_valid_2() {
        let c1 = PasswordCheck::from_str(String::from("3-4 t: dttt"));
        let c2 = PasswordCheck::from_str(String::from("1-3 b: cdefg"));
        let c3 = PasswordCheck::from_str(String::from("2-9 c: ccccccccc"));
        let c4 = PasswordCheck::from_str(String::from("1-3 a: abcde"));
        assert!(!c1.is_valid_2());
        assert!(!c2.is_valid_2());
        assert!(!c3.is_valid_2());
        assert!(c4.is_valid_2());
    }
}