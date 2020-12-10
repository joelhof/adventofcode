#![allow(non_snake_case)]
extern crate itertools;

use crate::core::*;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
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
                .map(|line| match line.trim().parse::<u64>() { Ok(i) => i, Err(_) => handleError(line) })
                .collect()
            };
    }

    fn isInValid(&self, number: &u64, index: &usize) -> bool {
        let start: usize = match index.checked_sub(self.preamble) { Some(i) => i, None => 0};
        let preambleSeq = &self.seq[start..*index];
        //println!("index {}, start {} {:?}", index, start, preambleSeq);
        let sums: HashSet<u64> = self.sums(preambleSeq, 2);
        
        //println!("{:?}", sums);
        return !sums.contains(number);
    }

    fn sums(&self, seq: &[u64], length: usize) -> HashSet<u64> {
       return seq.iter()
            .combinations(length)
            .map(|pair| pair.into_iter().sum())
            .collect();
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

    pub fn partTwo(&self) -> u64 {
        let invalidNumber = self.partOne();
        println!("invalid nr {}", invalidNumber);
        /* let sum: &Vec<&u64> = self.seq.iter()
            .fold_while(&Vec::new(), |mut acc, nr| {
                acc.push(nr);
                let currentSum: u64 = acc.into_iter().map(|x| *x).sum();
                if currentSum == invalidNumber {
                    return Done(acc);
                 } 
                else if currentSum > invalidNumber { Done(&Vec::new()) }
                else { Continue(acc) }
            }
        ).into_inner(); */
        let mut partialSeq: Vec<u64> = Vec::new();
        for (i, _nr) in self.seq.iter().enumerate() {
            let res: Option<Vec<u64>> = sumWhile(&self.seq[i..], invalidNumber);
            match res {
                Some(seq) => { partialSeq = seq; break },
                None => continue
            }
        }
        
        println!("{:?}", partialSeq);
        let sum: u64 = partialSeq[..].into_iter().sum();

        let min = match partialSeq[..].into_iter().min() {
            Some(min) => min,
            None => &0
       };
        let max = match partialSeq[..].into_iter().max() {
             Some(max) => max,
             None => &0
        };
        println!("sum {:?} max {:?} min {:?}", sum, min, max);
        return min + max;
    }
}

fn sumWhile(seq: &[u64], target: u64) -> Option<Vec<u64>> {
    let mut result: Vec<u64> = Vec::new();
    //let mut sum: u64 = 0;
    for nr in seq {
        println!("{}", nr);
        result.push(*nr);
        let sum: u64 = result.iter().sum();
        if sum == target {
            return Some(result);
        } else if sum > target {
            return None;
        }
    }
    return None;
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

    #[test]
    fn partTwoExampleTest() {
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
        let result = DayNine::test(INPUT, 5).partTwo();
        assert_eq!(result, 62);
    }
}