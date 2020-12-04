use std::{num::ParseIntError, ops::RangeInclusive, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref BYR_RE: Regex = Regex::new(r"byr:(\d+)\b").unwrap();
    pub static ref IYR_RE: Regex = Regex::new(r"iyr:(\d+)\b").unwrap();
    pub static ref EYR_RE: Regex = Regex::new(r"eyr:(\d+)\b").unwrap();
    pub static ref HGT_RE: Regex = Regex::new(r"hgt:(\d+)((?:cm)|(?:in))\b").unwrap();
    pub static ref HCL_RE: Regex = Regex::new(r"hcl:#([0-9a-f]{6})\b").unwrap();
    pub static ref ECL_RE: Regex =
        Regex::new(r"ecl:((?:amb)|(?:blu)|(?:brn)|(?:gry)|(?:grn)|(?:hzl)|(?:oth))\b").unwrap();
    pub static ref PID_RE: Regex = Regex::new(r"pid:(\d{9})\b").unwrap();
}
#[derive(Debug)]
enum Height {
    Inches(u32),
    Centimeters(u32),
}
#[derive(Debug)]
#[allow(dead_code)]
pub struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: Height,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}
#[derive(Debug)]
pub enum ParseError {
    Parse(ParseIntError),
    NoMatchError,
    InvalidFormatError,
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::Parse(e)
    }
}

fn extract_ranged_field(
    s: &str,
    range: RangeInclusive<u32>,
    regex: &Regex,
) -> Result<u32, ParseError> {
    let value: &u32 = &regex.captures(s).ok_or_else(|| ParseError::NoMatchError)?[1].parse()?;

    match value {
        value if range.contains(value) => Ok(*value),
        _ => Err(ParseError::InvalidFormatError),
    }
}

fn extract_height(s: &str) -> Result<Height, ParseError> {
    let captures = HGT_RE.captures(s).ok_or_else(|| ParseError::NoMatchError)?;
    let hgt_value: &u32 = &captures[1].parse()?;

    match &captures[2] {
        x if x == "cm" && (150..=193).contains(hgt_value) => Ok(Height::Centimeters(*hgt_value)),
        x if x == "in" && (59..=76).contains(hgt_value) => Ok(Height::Inches(*hgt_value)),
        _ => Err(ParseError::InvalidFormatError),
    }
}

fn extract_string(s: &str, regex: &Regex) -> Result<String, ParseError> {
    Ok(regex.captures(s).ok_or_else(|| ParseError::NoMatchError)?[1].into())
}

impl FromStr for Passport {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Passport {
            byr: extract_ranged_field(s, 1920..=2002, &(*BYR_RE))?,
            iyr: extract_ranged_field(s, 2010..=2020, &(*IYR_RE))?,
            eyr: extract_ranged_field(s, 2020..=2030, &(*EYR_RE))?,
            hgt: extract_height(s)?,
            hcl: extract_string(s, &(*HCL_RE))?,
            ecl: extract_string(s, &(*ECL_RE))?,
            pid: extract_string(s, &(*PID_RE))?,
            cid: String::from("TBD"),
        })
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

        assert!(s1.parse::<Passport>().is_ok());
        assert!(s2.parse::<Passport>().is_ok());
        assert!(s3.parse::<Passport>().is_ok());
        assert!(s4.parse::<Passport>().is_ok());
        assert!(s5.parse::<Passport>().is_err());
        assert!(s6.parse::<Passport>().is_err());
        assert!(s7.parse::<Passport>().is_err());
        assert!(s8.parse::<Passport>().is_err());
    }
}
