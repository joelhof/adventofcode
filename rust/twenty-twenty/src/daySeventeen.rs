#![allow(non_snake_case)]

use crate::core::*;
use std::str::FromStr;

pub struct Day {
    input: String
}

impl FromStr for Day {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        return Ok(Day {
            input: "".to_string()
        });
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Seventeen";
    }

    fn partOne(&self) -> u64 {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn seventeenPartOneExampleTest() {
        const INPUT: &str = ".#.
        ..#
        ###";
        let result = INPUT.parse::<Day>().unwrap().partOne();
        assert_eq!(result, 112);
    }

}