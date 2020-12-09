#![allow(non_snake_case)]

use crate::core::*;

pub struct DayNine {
    preamble: usize,
    seq: Vec<u32>
}

impl DayNine {
    fn test(input: &str, preamble: usize) -> DayNine {
        let tmp: Vec<u32> = input.split("\n")
            .map(|line| line.trim().parse::<u32>().unwrap())
            .collect();
        return DayNine {
            preamble: preamble,
            seq: tmp
        }
    }
}

impl AdventOfCodeSolver for DayNine {
    fn day(&self) -> &str {
        return "Nine";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        const INPUT: &str = "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";
        let result = DayNine::test(INPUT, 5).partOne();
        assert_eq!(result, 127);
    }
}