#![allow(non_snake_case)]

use crate::core::*;
use std::str::FromStr;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::error::Error;
use lazy_static::lazy_static;


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
        
        return passports.iter().filter(|passport| passport.is_ok()).count() as u64;
    }

    fn partTwo(&self) -> u64 {
        return self.input.iter()
            .filter(|passport| !passport.is_empty())
            .map(|passport| passport.parse::<Passport>())
            .filter_map(|passport| passport.ok())
            .filter_map(|passport| passport.validate().ok())
            .count() as u64;
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

lazy_static! {
    static ref FIELD_VALUE_RE: Regex = Regex::new(r"(?P<field>\w+):(?P<value>\w+|\#\w+)").unwrap();
}

impl FromStr for Passport {
    type Err = Box<dyn Error>;
    
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        
        let map: HashMap<String, String> = FIELD_VALUE_RE.captures_iter(input)
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

lazy_static! {
    static ref HEIGTH_RE: Regex = Regex::new(r"(([0-9]{2,3})(in|cm)\b)").unwrap();
    static ref HAIR_COLOR_RE: Regex = Regex::new(r"(#[0-9a-f]{6}\b)").unwrap();
    static ref EYE_COLOR_RE: Regex = Regex::new(r"(amb\b|blu\b|brn\b|gry\b|grn\b|hzl\b|oth\b)").unwrap();
    static ref PASSPORT_ID_RE: Regex = Regex::new(r"(^[0-9]{9}$)").unwrap();
}
type Err = Box<dyn Error>;

#[derive(Debug, Clone)]
struct InvalidFieldValue(String);

impl fmt::Display for InvalidFieldValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for InvalidFieldValue {}
impl Passport {

    pub fn validate(&self) -> Result<bool, Vec<Err>> {
        let mut errors: Vec<Err> = Vec::new();

        if !Self::isValidNumber(&self.birthYear, 1920, 2002) {
            errors.push(Box::new(InvalidFieldValue(format!("Invalid 'BirthYear' {}", self.birthYear).to_string())));
        }
        if !Self::isValidNumber(&self.issueYear, 2010, 2020) {
            errors.push(Box::new(InvalidFieldValue(format!("Invalid 'IssueYear' {}", self.issueYear).to_string())));
        }
        if !Self::isValidNumber(&self.expirationYear, 2020, 2030) {
            errors.push(Box::new(InvalidFieldValue(format!("Invalid 'ExpirationYear' {}", self.expirationYear).to_string())))
        }
        if !self.isValidHeight() {
            errors.push(Box::new(InvalidFieldValue(format!("Invalid 'Height' {}", self.height).to_string())))
        }
        if !self.isValidEyeColor() {
            errors.push(Box::new(InvalidFieldValue(format!("Invalid 'EyeColor' {}", self.eyeColor).to_string())))
        }
        if !self.isValidHairColor() {
            errors.push(Box::new(InvalidFieldValue(format!("Invalid 'HairColor' {}", self.hairColor).to_string())))
        }
        if !self.isValidId() {
            errors.push(Box::new(InvalidFieldValue(format!("Invalid 'Id' {}", self.passportID).to_string())))
        };
        
        return if errors.len() > 0 { Err(errors) } else { Ok(true) };
    }

    fn isValidNumber(nr: &str, min: u32, max: u32) -> bool {
        return match nr.parse::<u32>() {
            Ok(nr) if nr >= min && nr <= max => true,
            _ => false
        };
    }

    fn isValidHeight(&self) -> bool {
        return match HEIGTH_RE.captures(&self.height) {
            Some(cap) => {
                let h = cap.get(2).map_or("", |m| m.as_str());
                cap.get(3).map_or(false, |m| match m.as_str() { 
                    "in" => Self::isValidNumber(h, 59, 76),
                    "cm" => Self::isValidNumber(h, 150, 193),
                    _ => false
                 })
            },
            None => false  
        };
    }

    fn isValidEyeColor(&self) -> bool {
        return match EYE_COLOR_RE.captures(&self.eyeColor) {
            Some(_cap) => true,
            None => false
         }
    }

    fn isValidHairColor(&self) -> bool {
        return match HAIR_COLOR_RE.captures(&self.hairColor) {
            Some(_cap) => true,
            None => false
         }
    }

    fn isValidId(&self) -> bool {
        return match PASSPORT_ID_RE.captures(&self.passportID) {
            Some(_cap) => true,
            None => false
         }
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

    #[test]
    fn fourPartTwoInvalidExampleTest() {
        const INPUT: &str = "eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
        
        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946
        
        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
        
        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007";
        let result = INPUT.parse::<Day>().unwrap().partTwo();
        assert_eq!(result, 0);
    }

    #[test]
    fn fourPartTwoValidExampleTest() {
        const INPUT: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f
        
        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:7912057436 hcl:#a97842 hgt:165cm
        
        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022
        
        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let result = INPUT.parse::<Day>().unwrap().partTwo();
        assert_eq!(result, 3);
    }

    fn testPassport() -> Passport {
        return Passport {
            birthYear: "1920".to_string(),
            issueYear: "2012".to_string(),
            expirationYear: "2030".to_string(),
            height: "hgt:158cm".to_string(),
            hairColor: "#623a2f".to_string(),
            eyeColor: "grn".to_string(),
            passportID: "087499704".to_string(),
            countryID: None
        };
    }

    #[test]
    fn fieldValidationExampleTest() {
        let mut passport = testPassport();
        passport.birthYear = "1919".to_string();
        assert_eq!(passport.validate().is_err(), true);
        passport.birthYear = "2003".to_string();
        assert_eq!(passport.validate().is_err(), true);
        
        // hgt valid:   60in
        testHeight("60in", true);
        // hgt valid:   190cm
        testHeight("190cm", true);
        // hgt invalid: 190in
        testHeight("190in", false);
        // hgt invalid: 190
        testHeight("190", false);
        // hcl valid:   #123abc
        testHairColor("#123abc", true);
        // hcl invalid: #123abz
        testHairColor("#123abz", false);
        // hcl invalid: 123abc
        testHairColor("123abc", false);
        // ecl valid:   brn
        testEyeColor("brn", true);
        // ecl invalid: wat
        testEyeColor("wat", false);

        // pid valid:   000000001
        // pid invalid: 0123456789

    }

    fn testHeight(input: &str, expected: bool) {
        let mut passport = testPassport();
        passport.height = input.to_string();
        if expected {
            assert_eq!(passport.validate().is_err(), false);
            assert_eq!(passport.validate().ok(), Some(true));
        } else {
            assert_eq!(passport.validate().is_ok(), false);
            assert_eq!(passport.validate().is_err(), true);
        }
    }

    fn testHairColor(input: &str, expected: bool) {
        let mut passport = testPassport();
        passport.hairColor = input.to_string();
        let actual = passport.validate();
        if expected {
            assert_eq!(actual.is_err(), false);
            assert_eq!(actual.ok(), Some(true));
        } else {
            assert_eq!(actual.is_ok(), false);
            assert_eq!(actual.is_err(), true);
        }
    }

    fn testEyeColor(input: &str, expected: bool) {
        let mut passport = testPassport();
        passport.eyeColor = input.to_string();
        let actual = passport.validate();
        if expected {
            assert_eq!(actual.is_err(), false);
            assert_eq!(actual.ok(), Some(true));
        } else {
            assert_eq!(actual.is_ok(), false);
            assert_eq!(actual.is_err(), true);
        }
    }
 }