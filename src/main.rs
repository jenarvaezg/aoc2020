mod grid;
mod passport;
mod solutions;
mod solver;
use crate::solutions::solve;
use std::env;

fn main() {
    let day: u32 = env::args()
        .nth(1)
        .unwrap_or(String::from("17"))
        .parse()
        .unwrap();

    solve(day);
}
