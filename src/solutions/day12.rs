use io::Result;

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref RE: Regex = Regex::new(r"^(\D)(\d+)\b").unwrap();
}

pub struct Problem;

#[derive(Debug)]
pub enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}
#[derive(Debug)]
struct Location {
    position: (i32, i32),
    bearing: Action, // dont care about values, but we reuse NSEW
}

impl Solver for Problem {
    type Input = Vec<Action>;
    type Output = i32;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .map(|x| {
                let captures = RE.captures(&x).unwrap();
                match &captures[1] {
                    "N" => Action::North(captures[2].parse().unwrap()),
                    "S" => Action::South(captures[2].parse().unwrap()),
                    "E" => Action::East(captures[2].parse().unwrap()),
                    "W" => Action::West(captures[2].parse().unwrap()),
                    "L" => Action::Left(captures[2].parse().unwrap()),
                    "R" => Action::Right(captures[2].parse().unwrap()),
                    "F" => Action::Forward(captures[2].parse().unwrap()),
                    _ => unreachable!(),
                }
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        let initial_location = Location {
            position: (0, 0),
            bearing: Action::East(0),
        };
        let final_location = input.into_iter().fold(initial_location, process_movement);
        final_location.position.0.abs() + final_location.position.1.abs()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let initial_location = Location {
            position: (0, 0),
            bearing: Action::East(0),
        };
        let initial_waypoint = (10, 1);
        let (final_location, _final_waypoint) = input.into_iter().fold(
            (initial_location, initial_waypoint),
            process_movement_with_waypoint,
        );
        final_location.position.0.abs() + final_location.position.1.abs()
    }
}

fn process_movement(initial: Location, action: &Action) -> Location {
    match action {
        Action::North(val) => Location {
            position: (initial.position.0, initial.position.1 + val),
            bearing: initial.bearing,
        },
        Action::South(val) => Location {
            position: (initial.position.0, initial.position.1 - val),
            bearing: initial.bearing,
        },
        Action::East(val) => Location {
            position: (initial.position.0 - val, initial.position.1),
            bearing: initial.bearing,
        },
        Action::West(val) => Location {
            position: (initial.position.0 + val, initial.position.1),
            bearing: initial.bearing,
        },
        Action::Forward(val) => {
            let movement = match initial.bearing {
                Action::North(_) => Action::North(*val),
                Action::South(_) => Action::South(*val),
                Action::East(_) => Action::East(*val),
                Action::West(_) => Action::West(*val),
                _ => unreachable!(),
            };
            process_movement(initial, &movement)
        }
        Action::Left(val) => Location {
            position: initial.position,
            bearing: turn(&initial.bearing, *val * -1),
        },
        Action::Right(val) => Location {
            position: initial.position,
            bearing: turn(&initial.bearing, *val),
        },
    }
}

fn process_movement_with_waypoint(
    input: (Location, (i32, i32)),
    action: &Action,
) -> (Location, (i32, i32)) {
    let (initial, waypoint) = input;

    match action {
        Action::North(val) => (initial, (waypoint.0, waypoint.1 + val)),
        Action::South(val) => (initial, (waypoint.0, waypoint.1 - val)),
        Action::East(val) => (initial, (waypoint.0 + val, waypoint.1)),
        Action::West(val) => (initial, (waypoint.0 - val, waypoint.1)),
        Action::Forward(val) => {
            let dx = waypoint.0 * val;
            let dy = waypoint.1 * val;
            (
                Location {
                    position: (initial.position.0 + dx, initial.position.1 + dy),
                    bearing: initial.bearing,
                },
                waypoint,
            )
        }
        Action::Left(val) => (initial, turn_waypoint(waypoint, *val * -1)),
        Action::Right(val) => (initial, turn_waypoint(waypoint, *val)),
    }
}

fn turn(initial: &Action, amount: i32) -> Action {
    let initial_degrees = match initial {
        Action::North(_) => 270,
        Action::South(_) => 90,
        Action::East(_) => 0,
        Action::West(_) => 180,
        _ => unreachable!(),
    };

    let degrees = (initial_degrees + amount + 360) % 360;

    match degrees {
        0 => Action::East(0),
        90 => Action::South(0),
        180 => Action::West(0),
        270 => Action::North(0),
        _ => unreachable!(),
    }
}

fn turn_waypoint(initial: (i32, i32), degrees: i32) -> (i32, i32) {
    let absolute_degrees = (degrees + 360) % 360;

    match absolute_degrees {
        90 => (initial.1, initial.0 * -1),
        180 => (initial.0 * -1, initial.1 * -1),
        270 => (initial.1 * -1, initial.0),
        _ => unreachable!(),
    }
}
