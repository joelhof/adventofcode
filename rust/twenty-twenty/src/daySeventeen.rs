#![allow(non_snake_case)]

use crate::core::*;
use std::str::FromStr;
use std::collections::HashMap;
use itertools::Itertools;

pub struct Day {
    input: String,
    cubeGrid: HashMap<(isize, isize, isize), u8>
}

impl FromStr for Day {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let xSize = input.split("\n").count();
        let ySize = input.split("\n").next().unwrap().len();
        let init: HashMap<(isize, isize, isize), u8> = input.split("\n")
            .map(|line| line.trim())
            .enumerate()
            .filter(|(_i, line)| !line.is_empty())
            .flat_map(|(row, line)| line.chars()
                    .enumerate()
                    .filter(|(_col, c)| '#' == *c)
                    .map(move |(col, _c)| ((row as isize, col  as isize, 0isize), 1u8))
            )
            .collect();
        return Ok(Day {
            input: input.to_string(),
            cubeGrid: init
        });
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Seventeen";
    }

    fn partOne(&self) -> u64 {
        println!("{:?}", self.cubeGrid);
        let mut cubeGrid = self.cubeGrid.clone();
        for _i in 0..6 {
            cubeGrid = nextState(&cubeGrid, &partOneRule);
        }
        return cubeGrid.values().map(|x| *x as u64).sum::<u64>();
    }
}

fn nextState(
    grid: &HashMap<(isize, isize, isize), u8>,
    rule: &dyn Fn(&[isize; 3], &HashMap<(isize, isize, isize), u8>) -> u8)
 -> HashMap<(isize, isize, isize), u8> {
    // loop over each point that is neighbour to an active cube + all active cubes
    // loop over all active cubes
    let neighbours: Vec<[isize; 3]> = grid.keys()
        .flat_map(|coordinate| getNeighbours(coordinate))
        .unique()
        .collect();
    //println!("{:?} size {}", neighbours, neighbours.len());
    let result: HashMap<(isize, isize, isize), u8> = neighbours.into_iter()
    // for each such cube set new state according to rule
        .map(|[x,y,z]| ((x,y,z), rule(&[x,y,z], grid)))
        .filter(|(_coordinate, state)| *state > 0)
        .collect();
    println!("--------------");
    println!("{:?}, grid size {}", result, result.len());
    
    return result;
}

fn partOneRule(coordinate: &[isize; 3], grid: &HashMap<(isize, isize, isize), u8>) -> u8 {
    let current = (coordinate[0], coordinate[1], coordinate[2]);
    let neighbours = getNeighbours(&current);
    let activeNeighbours: usize = neighbours.into_iter()
        .filter(|cube| cube != coordinate)
        .filter(|[x, y, z]| grid.contains_key(&(*x, *y, *z)))
        .count();
    println!("current {:?}, active {}, active neighbours {}", coordinate, grid.contains_key(&current), activeNeighbours);
    if grid.contains_key(&current) {
        return if activeNeighbours == 2 || activeNeighbours == 3 { 1 } else { 0 };
    } else {
        return if activeNeighbours == 3 { 1 } else { 0 };
    }
}

fn getNeighbours(coordinate: &(isize, isize, isize)) -> Vec<[isize; 3]> {
    let (x,y,z) = coordinate;
    let r = -1isize..2;
    let mut result: Vec<[isize; 3]> = Vec::new();
    for deltaX in r.clone() {
        for deltaY in r.clone() {
            for deltaZ in r.clone() {
                let n = [(x + deltaX), (y + deltaY), (z + deltaZ)];
                result.push(n);
            }
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn seventeenPartOneExampleTest() {
        const INPUT: &str = ".#.
        ..#
        ###";
        let result = INPUT.parse::<Day>().unwrap().partOne();
        assert_eq!(result, 112);
    }

    #[test]
    fn getNeighboursTest() {
        let result = getNeighbours(&(1, 2, 3));
        assert_eq!(27, result.len());
        let shouldContain222 = result.into_iter().any(|[x,y,z]| x == 2 && y == 2 && z == 2);
        assert_eq!(true, shouldContain222);
        let shouldContain023 = getNeighbours(&(1, 2, 3)).into_iter().any(|[x,y,z]| x == 0 && y == 2 && z == 3);
        assert_eq!(true, shouldContain023);
    }
}