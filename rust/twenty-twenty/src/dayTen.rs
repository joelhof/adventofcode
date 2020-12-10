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

    fn partOne(&self) -> u64 {
        let mut adapterChain: Vec<u64> = self.adapters.iter().cloned().collect();
        adapterChain.sort();
        let maxJoltage = adapterChain.last();
        let mut reference = Vec::new();
        reference.push(0);
        reference.push(*maxJoltage.unwrap() + 3);
        reference.extend(&adapterChain[..]);
        reference.sort();
        let joltageDiff: Vec<(u64, u64)> = adapterChain[..].into_iter()
            .zip(reference)
            .map(|(adapter, jolt)| (*adapter, adapter - jolt))
            .collect();
        joltageDiff.iter().for_each(|(adapter, diff)|
            println!("adapter joltage {} joltage diff {}", adapter, diff));
        let oneJoltDiffs: usize = joltageDiff.iter()
            .filter(|(_adapter, diff)| *diff == 1)
            .map(|(_adapter, diff)| diff)
            .count();
        let threeJoltDiffs: usize = joltageDiff.iter()
            .filter(|(_adapter, diff)| *diff == 3)
            .map(|(_adapter, diff)| diff)
            .count();
        
        return (oneJoltDiffs * (threeJoltDiffs + 1)) as u64;
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


