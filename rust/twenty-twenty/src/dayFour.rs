#![allow(non_snake_case)]

use crate::core::*;
use std::str::FromStr;

pub struct Day {
    input: Vec<String>
}

impl FromStr for Day {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        return Ok(Day {
            input: input.split("\n")
                        .map(|line| line.trim())
                        .filter(|line| !line.is_empty())
                        .map(|line| line.to_string())
                        .collect()
        });
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Four";
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn fourPartOneExampleTest() {
        const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm
        
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929
        
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm
        
        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";
        let result = INPUT.parse::<Day>().unwrap().partOne();
        assert_eq!(result, 2);
    }
}