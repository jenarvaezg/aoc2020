use crate::solver::Solver;

use std::{
    collections::HashSet,
    io::{self, BufReader, Read},
};

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
            .map(|answers_str| get_answers_set_join(answers_str))
            .map(|answers| answers.len())
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        input
            .into_iter()
            .map(|answers_str| get_answers_set_intersection(answers_str))
            .map(|answers| answers.len())
            .sum()
    }
}

fn get_answers_set_join(s: &str) -> HashSet<char> {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn get_answers_set_intersection(s: &str) -> HashSet<char> {
    let mut sets = s.lines().map(|x| x.chars().collect::<HashSet<char>>());
    sets.next()
        .map(|set| {
            sets.fold(set, |agg, other| {
                agg.intersection(&other).copied().collect()
            })
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! set {
        ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
            {
                let mut temp_set = HashSet::new();  // Create a mutable HashSet
                $(
                    temp_set.insert($x); // Insert each item matched into the HashSet
                )*
                temp_set // Return the populated HashSet
            }
        };
    }

    #[test]
    fn test_get_answer_set_join() {
        assert_eq!(set!['a', 'b', 'c'], get_answers_set_join("abc"));
        assert_eq!(set!['a', 'b', 'c'], get_answers_set_join("ab\nac"));
        assert_eq!(set!['a'], get_answers_set_join("a\na\na\na"));
        assert_eq!(set!['b'], get_answers_set_join("b"));
    }

    #[test]
    fn test_get_answer_set_intersect() {
        assert_eq!(set!['a', 'b', 'c'], get_answers_set_intersection("abc"));
        assert_eq!(set!['a'], get_answers_set_intersection("ab\nac"));
        assert_eq!(set!['a'], get_answers_set_intersection("a\na\na\na"));
        assert_eq!(set!['b'], get_answers_set_intersection("b"));
    }
}
