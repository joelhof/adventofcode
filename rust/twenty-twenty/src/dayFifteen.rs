#![allow(non_snake_case)]

use crate::core::*;

pub struct Day {
    input: String
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Fifteen";
    }
 }

 impl Day {
    fn init(input: &str) -> Day {
        return Day {
            input: input.to_string()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn fifteenPartOneExampleTest() {
        const INPUT: &str = "0,3,6";
        let result = Day::init(INPUT).partOne();
        assert_eq!(result, 436);
    }
}