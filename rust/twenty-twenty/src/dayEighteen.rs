#![allow(non_snake_case)]

use crate::core::*;
use std::str::FromStr;

struct Day {
    input: Vec<String>
}

impl FromStr for Day {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        return Ok(Day {
            input:  input.split("\n")
                        .map(|line| line.trim())
                        .filter(|line| !line.is_empty())
                        .map(|line| line.to_string())
                        .collect()
        });
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Eighteen";
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn eighteenPartOneSimpleExampleTest() {
        const INPUT: &str = "1 + 2 * 3 + 4 * 5 + 6";
        let result = INPUT.parse::<Day>().unwrap().partOne();
        assert_eq!(result, 71);
    }
}