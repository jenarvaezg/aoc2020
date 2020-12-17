use io::Result;
use itertools::iproduct;

use crate::solver::Solver;
use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader},
};

pub struct Problem;

impl Solver for Problem {
    type Input = HashSet<(isize, isize, isize, isize)>;
    type Output = usize;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_x, c)| *c == '#')
                    .map(|(x, _c)| (x as isize, y as isize, 0, 0))
                    .collect::<HashSet<(isize, isize, isize, isize)>>()
            })
            .fold(
                HashSet::<(isize, isize, isize, isize)>::new(),
                |acc, current| {
                    acc.union(&current)
                        .map(|x| *x)
                        .collect::<HashSet<(isize, isize, isize, isize)>>()
                },
            )
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        let init_height = input
            .iter()
            .max_by(|&one, &other| one.1.cmp(&other.1))
            .unwrap()
            .1 as isize;
        let init_width = input
            .iter()
            .max_by(|&one, &other| one.0.cmp(&other.0))
            .unwrap()
            .0 as isize;

        let mut grid = input.clone();
        for cycle in 1isize..=6 {
            let mut new_grid = HashSet::<(isize, isize, isize, isize)>::new();
            for (z, y, x) in iproduct!(
                -cycle..=cycle,
                -cycle..=init_height + cycle,
                -cycle..=init_width + cycle
            ) {
                let count = iproduct!(x - 1..=x + 1, y - 1..=y + 1, z - 1..=z + 1)
                    .filter(|&(i, j, k)| {
                        grid.contains(&(i, j, k, 0)) && (i, j, k, 0) != (x, y, z, 0)
                    })
                    .count();
                if (2..=3).contains(&count) && grid.contains(&(x, y, z, 0)) {
                    new_grid.insert((x, y, z, 0));
                } else if count == 3 && !grid.contains(&(x, y, z, 0)) {
                    new_grid.insert((x, y, z, 0));
                }
            }
            grid = new_grid.clone();
        }
        grid.iter().count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let init_height = input
            .iter()
            .max_by(|&one, &other| one.1.cmp(&other.1))
            .unwrap()
            .1 as isize;
        let init_width = input
            .iter()
            .max_by(|&one, &other| one.0.cmp(&other.0))
            .unwrap()
            .0 as isize;

        let mut grid = input.clone();
        for cycle in 1isize..=6 {
            let mut new_grid = HashSet::<(isize, isize, isize, isize)>::new();
            for (w, z, y, x) in iproduct!(
                -cycle..=cycle,
                -cycle..=cycle,
                -cycle..=init_height + cycle,
                -cycle..=init_width + cycle
            ) {
                let count = iproduct!(x - 1..=x + 1, y - 1..=y + 1, z - 1..=z + 1, w - 1..=w + 1)
                    .filter(|&(i, j, k, l)| {
                        grid.contains(&(i, j, k, l)) && (i, j, k, l) != (x, y, z, w)
                    })
                    .count();
                if (2..=3).contains(&count) && grid.contains(&(x, y, z, w)) {
                    new_grid.insert((x, y, z, w));
                } else if count == 3 && !grid.contains(&(x, y, z, w)) {
                    new_grid.insert((x, y, z, w));
                }
            }
            grid = new_grid.clone();
        }
        grid.iter().count()
    }
}

#[allow(dead_code)]
fn print_grid(grid: &HashSet<(isize, isize, isize, isize)>) {
    let init_height = grid
        .iter()
        .max_by(|&one, &other| one.1.cmp(&other.1))
        .unwrap()
        .1 as isize;
    let init_width = grid
        .iter()
        .max_by(|&one, &other| one.0.cmp(&other.0))
        .unwrap()
        .0 as isize;
    let init_depth = grid
        .iter()
        .max_by(|&one, &other| one.2.cmp(&other.2))
        .unwrap()
        .2 as isize;

    let init_w = grid
        .iter()
        .max_by(|&one, &other| one.3.cmp(&other.3))
        .unwrap()
        .3 as isize;

    for z in -init_depth..=init_depth {
        for w in -init_w..=init_w {
            println!("Z = {} W = {}", z, w);
            for y in -init_height..=init_height {
                for x in -init_width..=init_width {
                    if grid.contains(&(x, y, z, w)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }
        }
    }
}
