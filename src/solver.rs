use std::{fmt::Display, io, time::Instant};

pub trait Solver {
    type Input;
    type Output: Display;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input;
    fn solve_first(&self, input: &Self::Input) -> Self::Output;
    fn solve_second(&self, input: &Self::Input) -> Self::Output;

    fn timed_first(&self, input: &Self::Input) -> Self::Output {
        let before = Instant::now();
        let solution = self.solve_first(input);

        println!("Part 1: {:?}", before.elapsed());
        solution
    }

    fn timed_second(&self, input: &Self::Input) -> Self::Output {
        let before = Instant::now();
        let solution = self.solve_second(input);

        println!("Part 2: {:?}", before.elapsed());
        solution
    }

    fn solve<R: io::Read + io::Seek>(&self, r: R) {
        let input = self.parse_input(r);
        let s1 = self.timed_first(&input);
        let s2 = self.timed_second(&input);
        println!("Solution 1: {}", s1);
        println!("Solution 2: {}", s2);
    }
}
