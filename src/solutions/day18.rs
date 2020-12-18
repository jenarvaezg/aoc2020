use io::Result;

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<char>>;
    type Output = usize;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.chars().filter(|c| !c.is_whitespace()).collect())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        input.iter().map(|x| eval_p1(&x[..])).sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        input.iter().map(|x| eval_p2(&x[..])).sum()
    }
}

fn find_matching_parenthesis(s: &[char]) -> usize {
    let mut offset = 0;
    for (i, &c) in s.iter().enumerate() {
        match c {
            '(' => offset += 1,
            ')' => offset -= 1,
            _ => {}
        }
        if offset == 0 {
            return i;
        }
    }
    unreachable!()
}

fn eval_p1(s: &[char]) -> usize {
    let (mut val, mut i) = (0, 0);
    let mut op = '+';
    while i < s.len() {
        match s[i] {
            '+' => op = '+',
            '*' => op = '*',
            '0'..='9' => match op {
                '+' => val += s[i].to_digit(10).unwrap() as usize,
                '*' => val *= s[i].to_digit(10).unwrap() as usize,
                _ => unreachable!(),
            },
            '(' => {
                let end = i + find_matching_parenthesis(&s[i..]);
                let v = eval_p1(&s[(i + 1)..end]);
                match op {
                    '+' => val += v,
                    '*' => val *= v,
                    _ => unreachable!(),
                }
                i = end;
            }
            _ => unreachable!(),
        }
        i += 1;
    }
    val
}

fn eval_term(s: &[char]) -> (usize, usize) {
    if s[0] == '(' {
        let j = find_matching_parenthesis(s);
        (eval_p2(&s[1..j]), j)
    } else {
        (s[0].to_digit(10).unwrap() as usize, 0)
    }
}

fn eval_p2(s: &[char]) -> usize {
    let (mut val, mut i) = (1, 0);
    while i < s.len() {
        let (mut v, step) = eval_term(&s[i..]);
        i += step;

        while let Some('+') = s.get(i + 1) {
            let (tmp, step) = eval_term(&s[(i + 2)..]);
            v += tmp;
            i += step + 2;
        }

        val *= v;
        i += 2;
    }
    val
}
