use core::fmt;
use std::{
    fmt::{Display, Formatter},
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

extern crate itertools;

pub trait Coord {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
    fn coords(&self) -> (usize, usize) {
        (self.x(), self.y())
    }
}

#[derive(Debug)]
pub struct GridPoint {
    x: usize,
    y: usize,
}

impl Coord for GridPoint {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

impl Coord for (usize, usize) {
    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }

    fn coords(&self) -> (usize, usize) {
        *self
    }
}

const DIRS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    cells: Vec<char>,
    pub w: usize,
    pub h: usize,
}

impl Grid {
    pub fn from_reader<R: Read>(r: R) -> Result<Self, ()> {
        let lines: Vec<String> = BufReader::new(r).lines().filter_map(Result::ok).collect();
        let h = lines.len();
        let w = lines[0].len();
        let cells: Vec<char> = lines.iter().flat_map(|s| s.chars()).collect();

        Ok(Grid { cells, h, w })
    }

    pub fn get(&self, c: impl Coord) -> Option<&char> {
        if c.x() < self.w && c.y() < self.h {
            self.cells.get(c.x() + c.y() * self.w)
        } else {
            None
        }
    }

    pub fn set(&mut self, c: impl Coord, v: char) {
        if let Some(e) = self.cells.get_mut(c.x() + c.y() * self.w) {
            *e = v;
        }
    }

    pub fn count_type(&self, c: char) -> usize {
        self.cells.iter().cloned().filter(|&x| c == x).count()
    }

    pub fn count_adjacents(&self, c: impl Coord, target: char) -> usize {
        let mut result: Vec<char> = Vec::new();

        for y in c.y().saturating_sub(1)..=c.y() + 1 {
            for x in c.x().saturating_sub(1)..=c.x() + 1 {
                if x == c.x() && y == c.y() {
                    continue;
                }

                if let Some(&c) = self.get((x, y)) {
                    if c == target {
                        result.push(c);
                    }
                }
            }
        }

        result.len()
    }

    pub fn get_first_in_line(&self, c: &impl Coord, line: (i64, i64)) -> Option<char> {
        let (mut x, mut y) = (c.x() as i64, c.y() as i64);
        loop {
            x += line.0;
            y += line.1;
            if x < 0 || y < 0 {
                break;
            }
            match self.get((x as usize, y as usize)) {
                Some('.') => continue,
                Some(&c) => return Some(c),
                None => break,
            }
        }
        None
    }

    pub fn count_line_of_sight(&self, c: &impl Coord, target: char) -> usize {
        DIRS.iter()
            .filter_map(|&(x, y)| self.get_first_in_line(c, (x, y)))
            .filter(|&v| v == target)
            .count()
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Grid::from_reader(s.as_bytes())
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        for row in self.cells.chunks(self.w) {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
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
    fn test_count_adjacents() {
        let grid: Grid = test_grid();
        assert_eq!(1, grid.count_adjacents((0, 0), '#'));
        assert_eq!(0, grid.count_adjacents((grid.w - 1, 0), '#'));
        assert_eq!(1, grid.count_adjacents((grid.w - 1, grid.h - 1), '#'));
        assert_eq!(2, grid.count_adjacents((0, grid.h - 1), '#'));
        assert_eq!(3, grid.count_adjacents((1, 1), '#'));
    }

    #[test]
    fn test_get_first_in_line() {
        let grid = test_grid();
        assert_eq!('#', grid.get_first_in_line(&(0, 0), (0, 1)).unwrap());
        assert!(grid.get_first_in_line(&(0, 0), (-1, 0)).is_none());
    }
}
