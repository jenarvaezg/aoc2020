use crate::solver::Solver;
use io::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

#[derive(PartialEq, Debug)]
pub struct PasswordCheck {
    password: String,
    letter: char,
    bounds: (usize, usize),
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (.+): (.+)").unwrap();
}

impl PasswordCheck {
    fn from_str(s: String) -> Self {
        if !RE.is_match(&s) {
            panic!("Bad format: {}", s);
        }
        let captures = RE.captures(&s).unwrap();
        PasswordCheck {
            password: String::from(&captures[4]),
            letter: captures[3].chars().next().unwrap(),
            bounds: (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
        }
    }

    fn is_valid(&self) -> bool {
        let char_count = self.password.chars().filter(|c| *c == self.letter).count();
        (self.bounds.0..=self.bounds.1).contains(&char_count)
    }

    fn is_valid_2(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        (chars[self.bounds.0 - 1] == self.letter) ^ (chars[self.bounds.1 - 1] == self.letter)
    }
}

impl Solver for Problem {
    type Input = Vec<PasswordCheck>;
    type Output = usize;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .map(|s| PasswordCheck::from_str(s))
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        input.iter().filter(|check| check.is_valid()).count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        input.iter().filter(|check| check.is_valid_2()).count()
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
            letter: 't',
            bounds: (3, 4),
        };
        assert_eq!(expected, PasswordCheck::from_str(String::from(input)));
    }

    #[test]
    fn test_password_check_is_valid() {
        let c1 = PasswordCheck::from_str(String::from("3-4 t: dtttt"));
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
