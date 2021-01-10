#![allow(non_snake_case)]

use crate::core::*;
use std::collections::HashSet;

pub struct Day {
    adapters: Vec<u64>
}

impl Day {
    pub fn new() -> Day {
        return Day {
            adapters: loadInput("Ten")
                .split("\n")
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.trim().parse().unwrap())
                .collect()
        };
    }

    pub fn test(input: &str) -> Day {
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

    fn partTwo(&self) -> u64 {
        let mut adapterChain: Vec<u64> = self.adapters.iter().cloned().collect();
        adapterChain.push(0);
        adapterChain.sort();
        let max = *adapterChain.last().unwrap() as usize;
        let adjacencyMatrix = createAdjacencyMatrix(&adapterChain[..]);
        let mut res = vec![0; max+1];
        res[max] = 1;
        for node in adapterChain.into_iter().rev().skip(1) {
            let neighbours: u64 = adjacencyMatrix[node as usize]
                .iter()
                .enumerate()
                .filter(|(_i, n)| **n > 0)
                .map(|(i, _n)| {
                    res[i as usize]
                })
                .sum();
            res[node as usize] = neighbours;
        }
        //println!("After loop {:?}", res);
        return res[0];
    }
}

fn createAdjacencyMatrix(adapters: &[u64]) -> Vec<Vec<u8>> {
    // each adapter can connect to adapters with 1,2 or 3 jolt higher rating
    // for each adapter, create entries in adjacency matrix for a +1, a+2, a+3 if 
    // those adapters exist in the list of valid adapters.
    let valid: HashSet<&u64> = adapters.iter().collect();
    let max = *adapters.last().unwrap() as usize;
    let length = max + 1;
    let mut matrix = vec![vec![0; length]; length];
    adapters.iter().for_each(|adapter| {
        let neighbours: Vec<usize> = vec![1,2,3].iter()
            .map(|n| adapter + n)
            .filter(|edge| valid.contains(edge))
            .map(|edge| edge as usize)
            .collect();
        //println!("{} -> {:?}", adapter, neighbours);
        neighbours.into_iter().for_each(|edge| matrix[*adapter as usize][edge] = 1);
    });
    //println!("{:#?}", matrix);
    return matrix;
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

    #[test]
    fn tenPartOneLargeExampleTest() {
        const INPUT: &str = " 28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";
        let result = Day::test(INPUT).partOne();
        assert_eq!(result, 22 * 10);
    }

    #[test]
    fn tenPartTwoSmallExampleTest() {
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
        let result = Day::test(INPUT).partTwo();
        assert_eq!(result, 8);
    }

    #[test]
    fn tenPartTwoLargeExampleTest() {
        const INPUT: &str = "28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";
        let result = Day::test(INPUT).partTwo();
        assert_eq!(result, 19208);
    }
}
