#![allow(non_snake_case)]

use std::collections::HashMap;

pub fn solve(input: &str) -> u32 {
    return input.split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| validPassword(line))
            .sum();
}

pub fn partTwo(input: &str) -> u32 {
    return input.split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| officialTobogganCorporatePassword(line))
            .sum();
}

fn validPassword(input: &str) -> u32 {
    let tmp: Vec<&str> = input.split(":").collect();
    let policy = Policy::new(tmp[0]);
    let password = tmp[1].trim();
    let mut frequencies = HashMap::new();
    for character in password.chars() {
        let count = frequencies.entry(character).or_insert(0);
        *count += 1;
    }
    return match frequencies.get(&policy.key) {
        None => 0,
        Some(x) if x >= &policy.min && x <= &policy.max => 1,
        Some(_) => 0
    };
}

fn officialTobogganCorporatePassword(input: &str) -> u32 {
    let tmp: Vec<&str> = input.split(":").collect();
    let policy = Policy::new(tmp[0]);
    let password = tmp[1].trim();
    let firstChar = password.chars().nth((policy.min-1) as usize).unwrap();
    let secondChar = password.chars().nth((policy.max-1) as usize).unwrap();
    if firstChar == policy.key && secondChar != policy.key {
        return 1;
    } else if secondChar == policy.key && firstChar != policy.key {
        return 1;
    } else {
        return 0;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Policy {
    min: u32,
    max: u32,
    key: char
}

impl Policy {
    pub fn new(input: &str) -> Policy {
        let tmp: Vec<_> = input.split(" ").collect();
        let tmp2: Result<Vec<u32>, _> = tmp[0].split("-")
                .map(|s| s.parse())
                .collect();
        let minMax = tmp2.unwrap();
        return Policy {
            min: minMax[0],
            max: minMax[1],
            key: tmp[1].trim().chars().nth(0).unwrap()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exampleTest() {
        const INPUT: &str = "1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc" ;
        let result = solve(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn multidigitPolicytidsTest() {
        const INPUT: &str = "11-311 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc" ;
        let result = solve(INPUT);
        assert_eq!(result, 1);
    }

    #[test]
    fn partTwoExample() {
        const INPUT: &str = "1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc" ;
        let result = partTwo(INPUT);
        assert_eq!(result, 1);
    }
}