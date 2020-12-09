#![allow(non_snake_case)]
extern crate itertools;

use crate::core::*;
use itertools::Itertools;
use std::collections::HashSet;

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

    fn isInValid(&self, number: &u32, index: &usize) -> bool {
        let start: usize = match index.checked_sub(self.preamble) { Some(i) => i, None => 0};
        let preambleSeq = &self.seq[start..*index];
        println!("index {}, start {} {:?}", index, start, preambleSeq);
        let sums: HashSet<u32> = preambleSeq.iter()
            .combinations(2)
            .map(|pair| pair.into_iter().sum())
            .collect();
        println!("{:?}", sums);
        return !sums.contains(number);
    }
}

impl AdventOfCodeSolver for DayNine {
    fn day(&self) -> &str {
        return "Nine";
    }

    fn partOne(&self) -> u32 {
        println!("{:?}", &self.seq);
        return match self.seq.iter()
            .enumerate()
            .skip(self.preamble)
            .find(|(i, x)| self.isInValid(x, i)) {
                Some((_, invalidNumber)) => *invalidNumber,
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