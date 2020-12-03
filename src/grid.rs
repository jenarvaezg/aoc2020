use std::{
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

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

#[derive(Debug)]
pub struct Grid {
    cells: Vec<char>,
    pub w: usize,
    pub h: usize,
}

impl Grid {
    pub fn from_reader<R: Read>(r: R) -> Self {
        let lines: Vec<String> = BufReader::new(r).lines().filter_map(Result::ok).collect();
        let h = lines.len();
        let w = lines.first().map_or(0, |c| c.len());
        let cells: Vec<char> = lines.iter().map(|s| s.chars()).flatten().collect();

        Grid { cells, h, w }
    }

    pub fn get(&self, c: impl Coord) -> Option<&char> {
        self.cells.get(c.x() + c.y() * self.w)
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n").collect();
        let h = lines.len();
        let w = lines.first().map_or(0, |c| c.len());
        let cells: Vec<char> = lines.iter().map(|s| s.chars()).flatten().collect();

        Ok(Grid { cells, h, w })
    }
}
