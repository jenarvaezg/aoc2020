use crate::solver::Solver;
use lazy_static::lazy_static;
use regex::Regex;

use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader},
};

lazy_static! {
    static ref INSTRUCTION_RE: Regex = Regex::new(r"(.*) ([+-]\d+)").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}
#[derive(Debug)]
pub enum ExitMode {
    Loop,
    Normal,
}
#[derive(Debug, PartialEq)]
pub struct InstructionResult {
    acc_modifier: i32,
    pointer_modifier: i32,
}
#[derive(Debug)]
pub struct ProgramResult {
    acc: i32,
    exit_mode: ExitMode,
}

type Program = Vec<Instruction>;

pub struct Problem;

impl Solver for Problem {
    type Input = Program;
    type Output = i32;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .map(|s| parse_instruction(&s))
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        run_until_completion(input).acc
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let mut cloned_input = input.clone();
        for i in 0..input.len() {
            let instruction = &input[i];
            cloned_input[i] = match instruction {
                Instruction::Jmp(v) => Instruction::Nop(*v),
                Instruction::Nop(v) => Instruction::Jmp(*v),
                Instruction::Acc(_) => continue,
            };
            if let ProgramResult {
                exit_mode: ExitMode::Normal,
                acc,
            } = run_until_completion(&cloned_input)
            {
                return acc;
            }
            cloned_input[i] = *instruction;
        }
        panic!("No result found!");
    }
}

fn parse_instruction(s: &str) -> Instruction {
    let captures = INSTRUCTION_RE.captures(s).unwrap();
    let value = captures[2].parse().unwrap();

    match captures[1].as_ref() {
        "nop" => Instruction::Nop(value),
        "jmp" => Instruction::Jmp(value),
        "acc" => Instruction::Acc(value),
        _ => panic!("panik :/ {}", s),
    }
}

fn process_instruction(inst: &Instruction) -> InstructionResult {
    match inst {
        Instruction::Nop(_) => InstructionResult {
            acc_modifier: 0,
            pointer_modifier: 1,
        },
        Instruction::Acc(v) => InstructionResult {
            acc_modifier: *v,
            pointer_modifier: 1,
        },
        Instruction::Jmp(v) => InstructionResult {
            acc_modifier: 0,
            pointer_modifier: *v,
        },
    }
}

fn run_until_completion(p: &Program) -> ProgramResult {
    let mut acc = 0;
    let mut pointer = 0;
    let mut visited_instructions: HashSet<usize> = HashSet::new();
    loop {
        if visited_instructions.contains(&pointer) {
            return ProgramResult {
                acc,
                exit_mode: ExitMode::Loop,
            };
        } else if pointer == p.len() {
            return ProgramResult {
                acc,
                exit_mode: ExitMode::Normal,
            };
        }
        let result = process_instruction(&p[pointer]);
        visited_instructions.insert(pointer);
        acc += result.acc_modifier;
        pointer = (pointer as i32 + result.pointer_modifier) as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(Instruction::Nop(0), parse_instruction("nop +0"));
        assert_eq!(Instruction::Acc(1), parse_instruction("acc +1"));
        assert_eq!(Instruction::Jmp(3), parse_instruction("jmp +3"));
        assert_eq!(Instruction::Acc(-99), parse_instruction("acc -99"));
    }
    #[test]
    fn test_process_instruction() {
        assert_eq!(
            InstructionResult {
                acc_modifier: 0,
                pointer_modifier: 1
            },
            process_instruction(&Instruction::Nop(10))
        );
        assert_eq!(
            InstructionResult {
                acc_modifier: 10,
                pointer_modifier: 1
            },
            process_instruction(&Instruction::Acc(10))
        );
        assert_eq!(
            InstructionResult {
                acc_modifier: 0,
                pointer_modifier: 10
            },
            process_instruction(&Instruction::Jmp(10))
        );
    }
}
