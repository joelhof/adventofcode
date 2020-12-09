#![allow(non_snake_case)]
extern crate itertools;

use crate::core::*;
use itertools::Itertools;
use std::collections::HashSet;

pub struct DayNine {
    preamble: usize,
    seq: Vec<u64>
}

impl DayNine {
    fn test(input: &str, preamble: usize) -> DayNine {
        return DayNine {
            preamble: preamble,
            seq: input.split("\n")
                .map(|line| line.trim().parse::<u64>().unwrap())
                .collect()
        }
    }
    pub fn new() -> DayNine {
        return DayNine {
            preamble: 25,
            seq: loadInput("Nine").split("\n")
                .filter(|line| !line.trim().is_empty())
                .map(|line| match line.trim().parse::<u64>() {Ok(i) => i, Err(_) => handleError(line) })
                .collect()
            };
    }

    fn isInValid(&self, number: &u64, index: &usize) -> bool {
        let start: usize = match index.checked_sub(self.preamble) { Some(i) => i, None => 0};
        let preambleSeq = &self.seq[start..*index];
        //println!("index {}, start {} {:?}", index, start, preambleSeq);
        let sums: HashSet<u64> = preambleSeq.iter()
            .combinations(2)
            .map(|pair| pair.into_iter().sum())
            .collect();
        //println!("{:?}", sums);
        return !sums.contains(number);
    }

    pub fn day(&self) -> &str {
        return "Nine";
    }

    pub fn partOne(&self) -> u64 {
        //println!("{:?}", &self.seq);
        return match self.seq.iter()
            .enumerate()
            .skip(self.preamble)
            .find(|(i, x)| self.isInValid(x, i)) {
                Some((_, invalidNumber)) => *invalidNumber,
                None => 0
            };
    }
}

fn handleError(err: &str) -> u64 {
    println!("error parsing line {}", err);
    return 0;
}

//impl AdventOfCodeSolver for DayNine {
  
//}

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