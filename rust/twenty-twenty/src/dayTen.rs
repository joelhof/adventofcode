#![allow(non_snake_case)]

use crate::core::*;

pub struct Day {
    adapters: Vec<u64>
}

impl Day {
    fn test(input: &str) -> Day {
        return Day {
            adapters: input.split("\n")
                .map(|line| line.trim().parse().unwrap())
                .collect()
        }
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Ten";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tenPartOneSmallExampleTest() {
        const INPUT: &str = "16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4";
        let result = Day::test(INPUT).partOne();
        assert_eq!(result, 7 * 5);
    }
}


