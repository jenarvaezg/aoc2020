mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
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
        d => println!("Day {} has not been solved yet", d),
    }
}
