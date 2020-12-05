use crate::solver::Solver;

use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output = usize;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        BufReader::new(r).lines().filter_map(Result::ok).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        input
            .into_iter()
            .map(|seat| get_seat_id(seat))
            .max()
            .unwrap()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let mut ids: Vec<usize> = input.into_iter().map(|seat| get_seat_id(seat)).collect();
        ids.sort();

        ids.windows(2)
            .filter(|&w| w[1] - w[0] != 1)
            .map(|w| w[1] - 1)
            .next()
            .unwrap()
    }
}

fn get_seat_id(seat: &str) -> usize {
    let row = &seat[..7];
    let column = &seat[7..];

    let rowid = seat_bin_search(row, ('F', 'B'));
    let colid = seat_bin_search(column, ('L', 'R'));

    rowid * 8 + colid
}

fn seat_bin_search(s: &str, matchers: (char, char)) -> usize {
    let mut low = 0;
    let mut high = (1 << s.len()) - 1;
    let mut mid = (low + high) / 2;

    for c in s.chars() {
        if c == matchers.0 {
            high = mid;
        } else {
            low = mid + 1;
        }

        mid = (low + high) / 2;
    }
    mid
}

#[allow(dead_code)]
fn seat_from_bin_shift(s: &str, matchers: (char, char)) -> usize {
    let mut val: usize = 0;
    for (i, c) in s.chars().rev().enumerate() {
        if c == matchers.1 {
            val ^= 1 << i;
        }
    }
    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_seat_id() {
        assert_eq!(567, get_seat_id("BFFFBBFRRR"));
        assert_eq!(119, get_seat_id("FFFBBBFRRR"));
        assert_eq!(820, get_seat_id("BBFFBBFRLL"));
        assert_eq!(38, get_seat_id("FFFFBFFRRL"));
    }
}
