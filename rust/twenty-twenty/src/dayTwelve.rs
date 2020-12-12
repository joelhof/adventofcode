#![allow(non_snake_case)]

use crate::core::*;

pub struct Day {

}

impl Day {
    fn test(input: &str) -> Day {
        return Day {

        };
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Twelve";
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn twelvePartOneExampleTest() {
        const INPUT: &str = "F10
            N3
            F7
            R90
            F11";
        let result = Day::test(INPUT).partOne();
        assert_eq!(result, 25);
    }
}