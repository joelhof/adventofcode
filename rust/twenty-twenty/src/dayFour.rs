#![allow(non_snake_case)]

use crate::core::*;
use std::str::FromStr;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

pub struct Day {
    input: Vec<String>
}

impl Day {
    pub fn new() -> Day {
        return loadInput("Four").parse().unwrap();
    }
}

impl FromStr for Day {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        return Ok(Day {
            input: input.split("\n")
                        .map(|line| line.trim())
                        .map(|line| line.to_string())
                        .fold(Vec::new(), |mut acc, line| {
                            if line.is_empty() {
                                acc.push("".to_string());
                            } else {
                                let group = match acc.pop() {
                                    Some(g) => g,
                                    None => "".to_string(),
                                };
                                let newGroup = format!("{}{}\n", group, line);
                                acc.push(newGroup);    
                            }
                            return acc;
                    })
        });
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Four";
    }

    fn partOne(&self) -> u64 {
        let passports: Vec<_> = self.input.iter()
            .filter(|passport| !passport.is_empty())
            .map(|passport| passport.parse::<Passport>())
            .collect();
        //println!("{:?}", passports);
        
        return passports.iter().filter(|passport| passport.is_ok()).count() as u64;
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Passport {
    
    birthYear: String,
    issueYear: String,
    expirationYear: String,
    height: String,
    hairColor: String,
    eyeColor: String,
    passportID: String,
    countryID: Option<u32>
}

impl FromStr for Passport {
    type Err = Box<dyn Error>;
    
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rangePattern = Regex::new(r"(?P<field>\w+):(?P<value>\w+|\#\w+)").unwrap();
        //let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
        let map: HashMap<String, String> = rangePattern.captures_iter(input)
            .map(|captures|{ 
                let f = &captures["field"];
                let v =  &captures["value"];
                (f.to_string(), v.to_string())
            }).collect();
        
        return Ok(Passport {
            birthYear: map.get("byr").ok_or_else(|| "Required field 'byr' is missing")?.to_string(),
            issueYear: map.get("iyr").ok_or_else(|| "Required field 'iyr' is missing")?.to_string(),
            expirationYear: map.get("eyr").ok_or_else(|| "Required field 'eyr' is missing")?.to_string(),
            height: map.get("hgt").ok_or_else(|| "Required field 'hgt' is missing")?.to_string(),
            hairColor: map.get("hcl").ok_or_else(|| "Required field 'hcl' is missing")?.to_string(),
            eyeColor: map.get("ecl").ok_or_else(|| "Required field 'ecl' is missing")?.to_string(),
            passportID: map.get("pid").ok_or_else(|| "Required field 'pid' is missing")?.to_string(),
            countryID: map.get("cid").and_then(|s| Some(s.parse().unwrap()))
        });
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
        assert_eq!(result, 4);
    }
}