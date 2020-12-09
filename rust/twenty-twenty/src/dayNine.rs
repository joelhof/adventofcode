#![allow(non_snake_case)]

use crate::core::*;

pub struct DayNine {
    preamble: usize,
    seq: Vec<u32>
}

impl DayNine {
    fn test(input: &str, preamble: usize) -> DayNine {
        return DayNine {
            preamble: preamble,
            seq: input.split("\n")
                .map(|line| line.trim().parse::<u32>().unwrap())
                .collect()
        }
    }

    fn isValid(&self, number: &u32) -> bool {
        return false;
    }
}

impl AdventOfCodeSolver for DayNine {
    fn day(&self) -> &str {
        return "Nine";
    }

    fn partOne(&self) -> u32 {
        println!("{:?}", self.seq);
        return match self.seq[self.preamble..].iter()
            .find(|x| self.isValid(x)) {
                Some(invalidNumber) => *invalidNumber,
                None => 0
            };
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