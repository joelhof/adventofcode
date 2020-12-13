#![allow(non_snake_case)]

use crate::core::*;

pub struct Day {
    arrival: u64,
    schedule: Vec<u64>
}

impl Day {
    fn init(input: &str) -> Day {
        return Day {
            arrival: 0,
            schedule: Vec::new()
        };
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Thirteen";
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn thirteenPartOneExampleTest() {
        const INPUT: &str = "939
        7,13,x,x,59,x,31,19";
        let result = Day::init(INPUT).partOne();
        assert_eq!(result, 295);
    }
}