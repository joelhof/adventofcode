#![allow(non_snake_case)]

use crate::core::*;

pub struct Day {
    input: String
}

impl Day {
    fn init(input: &str) -> Day {
        return Day {
            input: input.to_string()
        }
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Sixteen";
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sixteenPartOneExampleTest() {
        const INPUT: &str = "class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50
        
        your ticket:
        7,1,14
        
        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12";
        let result = Day::init(INPUT).partOne();
        assert_eq!(result, 71);
    }
}