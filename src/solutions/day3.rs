use crate::grid::Grid;
use crate::solver::Solver;
use std::io::{self};

pub struct Problem;

impl Solver for Problem {
    type Input = Grid;
    type Output = i64;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        Grid::from_reader(r).unwrap()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        trees_in_slope(&input, (3, 1))
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut total = 1;
        for slope in slopes {
            total *= trees_in_slope(input, slope);
        }
        total
    }
}

fn trees_in_slope(grid: &Grid, slope: (usize, usize)) -> i64 {
    let mut count = 0;
    for y in (0..grid.h).step_by(slope.1) {
        let x = (y * slope.0 / slope.1) % grid.w;
        if let Some(val) = grid.get((x, y)) {
            if *val == '#' {
                count += 1
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_grid() -> Grid {
        String::from(
            "..##.......\n\
            #...#...#..\n\
            .#....#..#.\n\
            ..#.#...#.#\n\
            .#...##..#.\n\
            ..#.##.....\n\
            .#.#.#....#\n\
            .#........#\n\
            #.##...#...\n\
            #...##....#\n\
            .#..#...#.#",
        )
        .parse()
        .unwrap()
    }

    #[test]
    fn test_trees_in_slope() {
        let grid: Grid = test_grid();
        assert_eq!(2, trees_in_slope(&grid, (1, 1)));
        assert_eq!(7, trees_in_slope(&grid, (3, 1)));
        assert_eq!(3, trees_in_slope(&grid, (5, 1)));
        assert_eq!(4, trees_in_slope(&grid, (7, 1)));
        assert_eq!(2, trees_in_slope(&grid, (1, 2)));
    }
}
