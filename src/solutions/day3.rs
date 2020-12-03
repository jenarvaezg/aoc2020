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
        trees_in_slope(&input, (3, 1))
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        slopes
            .into_iter()
            .fold(1, |acc, slope| acc * trees_in_slope(input, slope))
    }
}

fn trees_in_slope(grid: &Grid, slope: (usize, usize)) -> usize {
    (0..grid.h)
        .step_by(slope.1)
        .enumerate()
        .filter(|(i, y)| {
            let x = (i * slope.0) % grid.w;
            match grid.get((x, *y)) {
                Some('#') => true,
                _ => false,
            }
        })
        .count()
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
