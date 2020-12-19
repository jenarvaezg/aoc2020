use std::collections::HashMap;

use io::Result;

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub enum Rule {
    Ref(usize),
    Or(Box<Rule>, Box<Rule>),
    Ch(char),
    And3(Box<Rule>, Box<Rule>, Box<Rule>),
    And(Box<Rule>, Box<Rule>),
}

impl Rule {
    fn matches<'a>(
        &self,
        rules: &'a HashMap<usize, Rule>,
        unparsed: &'a [char],
    ) -> Vec<&'a [char]> {
        if unparsed.is_empty() {
            return vec![];
        }
        match self {
            Rule::Ref(i) => rules.get(i).unwrap().matches(rules, unparsed),
            Rule::Ch(c) => {
                if unparsed[0] == *c {
                    vec![&unparsed[1..]]
                } else {
                    vec![]
                }
            }
            Rule::Or(a, b) => {
                let mut r = Vec::new();
                for a in a.matches(rules, unparsed).into_iter() {
                    r.push(a);
                }
                for b in b.matches(rules, unparsed).into_iter() {
                    r.push(b);
                }

                r
            }
            Rule::And3(a, b, c) => {
                let mut r = Vec::new();
                for m in a.matches(rules, unparsed).into_iter() {
                    for n in b.matches(rules, m) {
                        for o in c.matches(rules, n) {
                            r.push(o);
                        }
                    }
                }
                r
            }
            Rule::And(a, b) => {
                let mut r = Vec::new();
                for m in a.matches(rules, unparsed).into_iter() {
                    for n in b.matches(rules, m) {
                        r.push(n);
                    }
                }
                r
            }
        }
    }
}
pub struct Problem;

impl Solver for Problem {
    type Input = (Vec<Vec<char>>, HashMap<usize, Rule>);
    type Output = usize;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        let mut lines = BufReader::new(r).lines().filter_map(Result::ok);
        let mut rules: HashMap<usize, Rule> = HashMap::new();

        loop {
            let line = lines.next().unwrap();
            if line == "" {
                break;
            }
            let split: Vec<_> = line.split(": ").collect();
            let id = split[0].parse().unwrap();
            let rule = parse_rule(split[1]);
            rules.insert(id, rule);
        }
        let messages: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();

        (messages, rules)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        let (messages, rules) = input;
        let initial_rule = rules.get(&0).unwrap();

        let mut c = 0;
        for msg in messages {
            for m in initial_rule.matches(&rules, &msg).into_iter() {
                if m.is_empty() {
                    c += 1;
                    break;
                }
            }
        }
        c
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let (messages, rules) = input;
        let initial_rule = rules.get(&0).unwrap();

        let mut c = 0;
        for msg in messages {
            for m in initial_rule.matches(&rules, &msg).into_iter() {
                if m.is_empty() {
                    c += 1;
                    break;
                }
            }
        }
        c
    }
}

fn parse_rule(s: &str) -> Rule {
    if s.contains(" | ") {
        let parts: Vec<_> = s.split(" | ").collect();

        Rule::Or(
            Box::new(parse_rule(parts[0])),
            Box::new(parse_rule(parts[1])),
        )
    } else if s.starts_with('"') {
        Rule::Ch(s.chars().nth(1).unwrap())
    } else if let Ok(i) = s.parse() {
        Rule::Ref(i)
    } else {
        let parts: Vec<_> = s.split(' ').collect();
        if parts.len() == 3 {
            Rule::And3(
                Box::new(parse_rule(parts[0])),
                Box::new(parse_rule(parts[1])),
                Box::new(parse_rule(parts[2])),
            )
        } else if parts.len() == 2 {
            Rule::And(
                Box::new(parse_rule(parts[0])),
                Box::new(parse_rule(parts[1])),
            )
        } else {
            panic!();
        }
    }
}
