use bitvec::{order::Lsb0, slice::BitSlice};
use io::Result;

use crate::solver::Solver;
use std::{
    collections::{HashMap, VecDeque},
    io::{self, BufRead, BufReader},
};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref MEM_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
}

pub struct Problem;

type Mask = HashMap<usize, Option<bool>>;

#[derive(Debug)]
pub enum Instruction {
    Mem(u64, u64),
    Mask(String),
}

impl Solver for Problem {
    type Input = Vec<Instruction>;
    type Output = u64;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .map(|x| {
                if x.contains("mask") {
                    Instruction::Mask(String::from(x.split(' ').nth(2).unwrap()))
                } else {
                    let captures = MEM_RE.captures(&x).unwrap();
                    Instruction::Mem(captures[1].parse().unwrap(), captures[2].parse().unwrap())
                }
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        let mut mask = Mask::new();
        let mut memory = HashMap::<u64, u64>::new();
        for instruction in input.iter().clone() {
            match instruction {
                Instruction::Mask(mask_values) => {
                    mask = mask_from_str(mask_values);
                }
                Instruction::Mem(address, value) => {
                    memory.insert(*address, apply_mask(&mask, *value));
                }
            }
        }
        memory.values().sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let mut mask = Mask::new();
        let mut memory = HashMap::<u64, u64>::new();
        for instruction in input.iter().clone() {
            match instruction {
                Instruction::Mask(mask_values) => {
                    mask = mask_from_str_with_x(mask_values);
                }
                Instruction::Mem(address, value) => {
                    for new_address in apply_address_mask(&mask, *address).iter() {
                        memory.insert(*new_address, *value);
                    }
                }
            }
        }
        memory.values().sum()
    }
}

fn apply_mask(mask: &Mask, value: u64) -> u64 {
    let mut cloned_value = value.clone();
    let bits = BitSlice::<Lsb0, _>::from_element_mut(&mut cloned_value);
    for (pos, value) in mask.iter() {
        bits.set(*pos, value.unwrap());
    }
    cloned_value
}

fn apply_address_mask(mask: &Mask, value: u64) -> Vec<u64> {
    let mut cloned_value = value.clone();
    let mut xs = Vec::<usize>::new();

    for (i, mask_val) in mask {
        match mask_val {
            Some(true) => cloned_value |= 2u64.pow(*i as u32),
            Some(false) => {}
            None => xs.push(*i),
        }
    }

    let mut values = VecDeque::<u64>::new();
    values.push_back(cloned_value);
    for x in xs {
        for _ in 0..values.len() {
            let val = values.pop_front().unwrap();
            let mask_val = 2u64.pow(x as u32);
            values.push_back(val | mask_val);
            values.push_back(val & !(mask_val))
        }
    }

    values.iter().map(|&x| x).collect()
}

fn mask_from_str_with_x(s: &str) -> Mask {
    let mut mask = Mask::new();

    for (i, x) in s.chars().rev().enumerate() {
        let v = match x {
            '1' => Some(true),
            '0' => Some(false),
            'X' => None,
            _ => unreachable!(),
        };
        mask.insert(i, v);
    }
    mask
}

fn mask_from_str(s: &str) -> Mask {
    let mut mask = Mask::new();

    for (i, x) in s.chars().rev().enumerate().filter(|(_, x)| *x != 'X') {
        mask.insert(i, if x == '1' { Some(true) } else { Some(false) });
    }
    mask
}
