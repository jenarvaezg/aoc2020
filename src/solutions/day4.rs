use crate::solver::Solver;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, BufReader, Read};

lazy_static! {
    static ref BYR_RE: Regex = Regex::new(r"byr:(\d+)\b").unwrap();
    static ref IYR_RE: Regex = Regex::new(r"iyr:(\d+)\b").unwrap();
    static ref EYR_RE: Regex = Regex::new(r"eyr:(\d+)\b").unwrap();
    static ref HGT_RE: Regex = Regex::new(r"hgt:(\d+)((?:cm)|(?:in))\b").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"hcl:#([0-9a-f]{6})\b").unwrap();
    static ref ECL_RE: Regex =
        Regex::new(r"ecl:((?:amb)|(?:blu)|(?:brn)|(?:gry)|(?:grn)|(?:hzl)|(?:oth))\b").unwrap();
    static ref PID_RE: Regex = Regex::new(r"pid:(\d{9})\b").unwrap();
}
#[allow(dead_code)]
pub struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}
pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output = usize;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        let mut buf_r = BufReader::new(r);
        let mut buffer = String::new();
        buf_r.read_to_string(&mut buffer).unwrap();

        buffer.split("\n\n").map(|s| String::from(s)).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        input
            .into_iter()
            .filter(|&pass| passport_has_fields(pass))
            .count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        input
            .into_iter()
            .filter(|&pass| passport_has_values(pass))
            .count()
    }
}

fn passport_has_fields(pass: &str) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_fields
        .iter()
        .filter(|&field| pass.contains(field))
        .count()
        == required_fields.len()
}

fn passport_has_values(pass: &str) -> bool {
    validate_byr(pass)
        && validate_iyr(pass)
        && validate_eyr(pass)
        && validate_hgt(pass)
        && HCL_RE.is_match(pass)
        && ECL_RE.is_match(pass)
        && PID_RE.is_match(pass)
}

fn validate_byr(pass: &str) -> bool {
    BYR_RE.is_match(pass)
        && (1920..=2002).contains(&BYR_RE.captures(pass).unwrap()[1].parse().unwrap())
}

fn validate_iyr(pass: &str) -> bool {
    IYR_RE.is_match(pass)
        && (2010..=2020).contains(&IYR_RE.captures(pass).unwrap()[1].parse().unwrap())
}

fn validate_eyr(pass: &str) -> bool {
    EYR_RE.is_match(pass)
        && (2020..=2030).contains(&EYR_RE.captures(pass).unwrap()[1].parse().unwrap())
}

fn validate_hgt(pass: &str) -> bool {
    if !HGT_RE.is_match(pass) {
        return false;
    }

    let hgt_captures = HGT_RE.captures(pass).unwrap();
    let hgt_value: &i32 = &hgt_captures[1].parse().unwrap();

    match &hgt_captures[2] {
        "cm" => (150..=193).contains(hgt_value),
        "in" => (59..=76).contains(hgt_value),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passport_has_values() {
        let s1 = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f";
        let s2 = "eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm";
        let s3 = "hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022";
        let s4 = "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let s5 = "eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";
        let s6 = "iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946";
        let s7 = "hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277";
        let s8 = "hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007";

        assert!(passport_has_values(&s1));
        assert!(passport_has_values(&s2));
        assert!(passport_has_values(&s3));
        assert!(passport_has_values(&s4));
        assert!(!passport_has_values(&s5));
        assert!(!passport_has_values(&s6));
        assert!(!passport_has_values(&s7));
        assert!(!passport_has_values(&s8));
    }
}
