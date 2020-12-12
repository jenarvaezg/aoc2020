use crate::grid::Grid;
use crate::solver::Solver;
use std::io::{self};

pub struct Problem;

impl Solver for Problem {
    type Input = Grid;
    type Output = usize;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        Grid::from_reader(r).unwrap()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        let adjacency = |g: &Grid, coord: (usize, usize)| g.count_adjacents(coord, '#');
        process_until_balance(input.clone(), adjacency, 5).count_type('#')
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let adjacency = |g: &Grid, coord: (usize, usize)| g.count_line_of_sight(&coord, '#');
        process_until_balance(input.clone(), adjacency, 5).count_type('#')
    }
}

fn process_until_balance<F: Fn(&Grid, (usize, usize)) -> usize>(
    mut grid: Grid,
    adjacents_fn: F,
    tolerance: usize,
) -> Grid {
    loop {
        let mut changed = false;
        let grid_snapshot = grid.clone();

        for x in 0..grid.w {
            for y in 0..grid.h {
                let current = *grid_snapshot.get((x, y)).unwrap();
                if current == '.' {
                    continue;
                }
                let adjacents = adjacents_fn(&grid_snapshot, (x, y));
                if let (new, true) = run_step(current, adjacents, tolerance) {
                    grid.set((x, y), new);
                    changed = true
                }
            }
        }

        if !changed {
            break;
        };
    }

    grid
}

fn run_step(current: char, adjacents: usize, occupied_tolerance: usize) -> (char, bool) {
    match current {
        'L' => {
            if adjacents == 0 {
                ('#', true)
            } else {
                ('L', false)
            }
        }
        '#' => {
            if adjacents >= occupied_tolerance {
                ('L', true)
            } else {
                ('#', false)
            }
        }
        '.' => ('.', false),
        _ => unreachable!(),
    }
}
