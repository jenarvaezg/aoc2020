use io::Result;

use crate::solver::Solver;
use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead, BufReader},
    ops::RangeInclusive,
};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref CONSTRAINT_RE: Regex =
        Regex::new(r"^(\D+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
}
type Ticket = Vec<u64>;
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TicketConstraint {
    label: String,
    ranges: (RangeInclusive<u64>, RangeInclusive<u64>),
}

impl TicketConstraint {
    fn is_value_ok(&self, value: &u64) -> bool {
        self.ranges.0.contains(value) || self.ranges.1.contains(value)
    }
}

#[derive(Debug)]
pub struct Input {
    nearby_tickets: Vec<Ticket>,
    my_ticket: Ticket,
    constraints: Vec<TicketConstraint>,
}

impl Input {
    fn ticket_error_rate(&self, ticket: &Ticket) -> usize {
        ticket
            .iter()
            .filter(|&x| {
                self.constraints
                    .iter()
                    .all(|constraint| !constraint.is_value_ok(x))
            })
            .map(|&x| x as usize)
            .sum()
    }

    fn is_ticket_ok(&self, ticket: &Ticket) -> bool {
        let result = ticket.iter().all(|x| {
            self.constraints
                .iter()
                .any(|constraint| constraint.is_value_ok(x))
        });

        result
    }

    fn scanning_error_rate(&self) -> usize {
        self.nearby_tickets
            .iter()
            .map(|x| self.ticket_error_rate(x))
            .sum()
    }

    fn get_my_parsed_ticket(&self) -> HashMap<&String, u64> {
        let valid_tickets: Vec<&Ticket> = self
            .nearby_tickets
            .iter()
            .filter(|&x| self.is_ticket_ok(x))
            .collect();

        let mut constraint_possibilities = HashMap::<&String, Vec<usize>>::new();
        for constraint in self.constraints.iter() {
            for i in 0..valid_tickets[0].len() {
                if valid_tickets.iter().all(|&t| constraint.is_value_ok(&t[i])) {
                    constraint_possibilities
                        .entry(&constraint.label)
                        .or_insert(Vec::<usize>::new())
                        .push(i);
                }
            }
        }

        let mut final_positions = HashMap::<usize, &String>::new();
        loop {
            for (constraint, mut positions) in constraint_possibilities.clone() {
                let values: HashSet<&usize> = final_positions.keys().collect();
                positions.retain(|x| !values.contains(x));
                constraint_possibilities.insert(constraint, positions.clone());
                if positions.len() == 1 {
                    final_positions.insert(positions[0], constraint);
                    constraint_possibilities.remove(constraint);
                }
            }

            if final_positions.len() == self.constraints.len() {
                break;
            }
        }

        self.my_ticket.iter().enumerate().fold(
            HashMap::<&String, u64>::new(),
            |mut acc, (index, &value)| {
                acc.insert(*final_positions.get(&index).unwrap(), value);
                acc
            },
        )
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Input;
    type Output = usize;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        let mut iter = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .filter(|x| x != "");

        let mut constraints = Vec::<TicketConstraint>::new();
        loop {
            let line = iter.next().unwrap();
            if !CONSTRAINT_RE.is_match(&line) {
                break;
            }
            let captures = CONSTRAINT_RE.captures(&line).unwrap();
            constraints.push(TicketConstraint {
                label: captures[1].to_owned(),
                ranges: (
                    RangeInclusive::new(captures[2].parse().unwrap(), captures[3].parse().unwrap()),
                    RangeInclusive::new(captures[4].parse().unwrap(), captures[5].parse().unwrap()),
                ),
            })
        }
        let my_ticket = iter
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        iter.next();
        let nearby_tickets = iter
            .map(|x| x.split(",").map(|x| x.parse::<u64>().unwrap()).collect())
            .collect();

        Input {
            constraints,
            my_ticket,
            nearby_tickets,
        }
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        input.scanning_error_rate()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        input
            .get_my_parsed_ticket()
            .iter()
            .filter(|&(k, _v)| k.starts_with("departure"))
            .map(|(_k, v)| v)
            .fold(1, |acc, x| acc * x) as usize
    }
}
