mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
use crate::solver::Solver;
use std::fs::File;

fn load_day(day: u32) -> File {
    let path = format!("inputs/day{}.txt", day);
    match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file) => file,
    }
}

pub fn solve(day: u32) {
    let day_file = load_day(day);
    println!("Day {}", day);
    match day {
        1 => day1::Problem {}.solve(day_file),
        2 => day2::Problem {}.solve(day_file),
        3 => day3::Problem {}.solve(day_file),
        4 => day4::Problem {}.solve(day_file),
        5 => day5::Problem {}.solve(day_file),
        6 => day6::Problem {}.solve(day_file),
        7 => day7::Problem {}.solve(day_file),
        8 => day8::Problem {}.solve(day_file),
        9 => day9::Problem {}.solve(day_file),
        10 => day10::Problem {}.solve(day_file),
        11 => day11::Problem {}.solve(day_file),
        12 => day12::Problem {}.solve(day_file),
        13 => day13::Problem {}.solve(day_file),
        14 => day14::Problem {}.solve(day_file),
        d => println!("Day {} has not been solved yet", d),
    }
}
