#![allow(non_snake_case)]

use std::collections::HashMap;
use crate::core::*;

pub struct DayThree {
    treeMap: Vec<HashMap<usize, u32>>,
    length: usize
}

impl DayThree {
    pub fn new(input: &str) -> DayThree {
        return DayThree {
            treeMap: input.split("\n")
                        .map(|line| line.trim())
                        .map(|line| parseLine(line))
                        .collect(),
            length: input.split("\n").next().unwrap().chars().count()
        }
    }

    fn countTrees(&self, right: usize, down: usize) -> u32 {
        let mut pos: usize = 0;
        let mut sum: u32 = 0;
        for row in self.treeMap.iter().step_by(down) {
            sum = sum + match row.get(&pos) {
                Some(tree) => tree,
                None => &0
            };
            if pos + right >= self.length {
                pos = (pos + right) - self.length;
            } else {
                pos = pos + right;
            }
            //println!("pos {}", pos);
        }
        return sum;
    }
}

impl AdventOfCodeSolver for DayThree {
    fn day(&self) -> &str {
        return "Three";
    }

    fn partOne(&self) -> u32 {
        return self.countTrees(3, 1);
    }

    fn partTwo(&self) -> u32 {
        let directions = [[1,1],[3,1],[5,1],[7,1],[1,2]];
        return directions.iter()
            .map(|direction| self.countTrees(direction[0], direction[1]))
            .fold(1, |acc, x| acc * x);
    }
}

fn parseLine(line: &str) -> HashMap<usize, u32> {
    let mut trees = HashMap::new();
    for (i, c) in line.chars().enumerate() {
        if c == '#' {
            trees.insert(i, 1);
        }
    }
    return trees;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exampleTest() {
        const INPUT: &str = 
        "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#" ;
        let result = DayThree::new(INPUT).partOne();
        assert_eq!(result, 7);
    }

    #[test]
    fn partTwoexampleTest() {
        const INPUT: &str = 
        "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#" ;
        let result = DayThree::new(INPUT).partTwo();
        assert_eq!(result, 336);
    }
}