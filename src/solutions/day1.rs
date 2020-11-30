use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u64>;
    type Output = u64;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines().flatten().flat_map(|l| l.parse()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        let x = input.iter().cloned();
        x.map(module_fuel).sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let x = input.iter().cloned();
        x.map(total_fuel_mass).sum()
    }
}

fn module_fuel(mass: u64) -> u64 {
    ((mass as f64 / 3f64).floor() - 2f64) as u64
}

fn total_fuel_mass(mass: u64) -> u64 {
    match mass {
        0 => 0,
        _ => {
            let fuel = module_fuel(mass);
            fuel + total_fuel_mass(fuel).max(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_fuel() {
        assert_eq!(module_fuel(12), 2);
        assert_eq!(module_fuel(14), 2);
        assert_eq!(module_fuel(1969), 654);
        assert_eq!(module_fuel(100_756), 33583);
        assert_eq!(module_fuel(1), 0);
    }

    #[test]
    fn test_total_fuel_mass() {
        assert_eq!(total_fuel_mass(14), 2);
        assert_eq!(total_fuel_mass(1969), 966);
        assert_eq!(total_fuel_mass(100_756), 50346);
    }
}
