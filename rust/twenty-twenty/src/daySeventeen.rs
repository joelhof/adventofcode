#![allow(non_snake_case)]

use crate::core::*;
use std::str::FromStr;
use std::collections::HashMap;
use itertools::Itertools;

pub struct Day {
    cubeGrid: HashMap<[isize; 3], u8>,
}

impl Day {
    pub fn new() -> Day {
        return loadInput("Seventeen").parse().unwrap();
    }
}

trait GameOfLife {
    type Coordinate;

    fn nextState(&self) -> HashMap<Self::Coordinate, u8>;

    fn getNeighbours(&self, coordinate: &Self::Coordinate) -> Vec<Self::Coordinate>;

    fn rule(&self, coordinate: &Self::Coordinate) -> u8;
}

impl FromStr for Day {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let xSize = input.split("\n").count();
        let ySize = input.split("\n").next().unwrap().len();
        
        let init: HashMap<[isize; 3], u8> = input.split("\n")
            .map(|line| line.trim())
            .enumerate()
            .filter(|(_i, line)| !line.is_empty())
            .flat_map(|(row, line)| line.chars()
                    .enumerate()
                    .filter(|(_col, c)| '#' == *c)
                    .map(move |(col, _c)| ([row as isize, col  as isize, 0isize], 1u8))
            )
            .collect();
        return Ok(Day {
            cubeGrid: init
        });
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Seventeen";
    }

    fn partOne(&self) -> u64 {
        //println!("{:?}", self.cubeGrid);
        let mut cubeGrid = self.cubeGrid.clone();
        for _i in 0..6 {
            cubeGrid = nextState(&cubeGrid, &partOneRule);
        }
        return cubeGrid.values().map(|x| *x as u64).sum::<u64>();
    }

    fn partTwo(&self) -> u64 {
        let mut cubeGrid: HashMap<[isize; 4], u8> = self.cubeGrid.clone().iter()
            .map(|([x,y,z], v)| ([*x,*y,*z,0isize], *v))
            .collect();
        for _i in 0..6 {
            let neighbours: Vec<[isize; 4]> = cubeGrid.keys()
                .flat_map(|coordinate| getNeighbours4d(coordinate))
                .unique()
                .collect();
    //println!("{:?} size {}", neighbours, neighbours.len());
            cubeGrid = neighbours.into_iter()
    // for each such cube set new state according to rule
                .map(|cube| (cube, partTwoRule(&cube, &cubeGrid)))
                .filter(|(_coordinate, state)| *state > 0)
                .collect();
        }
        return cubeGrid.values().map(|x| *x as u64).sum::<u64>();
    }
}

fn nextState(
    grid: &HashMap<[isize; 3], u8>,
    rule: &dyn Fn(&[isize; 3], &HashMap<[isize; 3], u8>) -> u8)
 -> HashMap<[isize; 3], u8> {
    // loop over each point that is neighbour to an active cube + all active cubes
    // loop over all active cubes
    let neighbours: Vec<[isize; 3]> = grid.keys()
        .flat_map(|coordinate| getNeighbours(coordinate))
        .unique()
        .collect();
    //println!("{:?} size {}", neighbours, neighbours.len());
    let result: HashMap<[isize; 3], u8> = neighbours.into_iter()
    // for each such cube set new state according to rule
        .map(|cube| (cube, rule(&cube, grid)))
        .filter(|(_coordinate, state)| *state > 0)
        .collect();
    //println!("--------------");
    //println!("{:?}, grid size {}", result, result.len());
    
    return result;
}

fn partOneRule(coordinate: &[isize; 3], grid: &HashMap<[isize; 3], u8>) -> u8 {
    //let current = (coordinate[0], coordinate[1], coordinate[2]);
    let neighbours = getNeighbours(&coordinate);
    let activeNeighbours: usize = neighbours.into_iter()
        .filter(|cube| cube != coordinate)
        .filter(|cube| grid.contains_key(cube))
        .count();
    //println!("current {:?}, active {}, active neighbours {}", coordinate, grid.contains_key(&current), activeNeighbours);
    if grid.contains_key(coordinate) {
        return if activeNeighbours == 2 || activeNeighbours == 3 { 1 } else { 0 };
    } else {
        return if activeNeighbours == 3 { 1 } else { 0 };
    }
}

fn getNeighbours(coordinate: &[isize; 3]) -> Vec<[isize; 3]> {
    let [x,y,z] = coordinate;
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

fn partTwoRule(coordinate: &[isize; 4], grid: &HashMap<[isize; 4], u8>) -> u8 {
    //let current = (coordinate[0], coordinate[1], coordinate[2]);
    let neighbours = getNeighbours4d(&coordinate);
    let activeNeighbours: usize = neighbours.into_iter()
        .filter(|cube| cube != coordinate)
        .filter(|cube| grid.contains_key(cube))
        .count();
    //println!("current {:?}, active {}, active neighbours {}", coordinate, grid.contains_key(&current), activeNeighbours);
    if grid.contains_key(coordinate) {
        return if activeNeighbours == 2 || activeNeighbours == 3 { 1 } else { 0 };
    } else {
        return if activeNeighbours == 3 { 1 } else { 0 };
    }
}

fn getNeighbours4d(coordinate: &[isize; 4]) -> Vec<[isize; 4]> {
    let [x,y,z,w] = coordinate;
    let r = -1isize..2;
    let mut result: Vec<[isize; 4]> = Vec::new();
    for deltaX in r.clone() {
        for deltaY in r.clone() {
            for deltaZ in r.clone() {
                for deltaW in r.clone() {
                    let n = [(x + deltaX), (y + deltaY), (z + deltaZ), (w + deltaW)];
                    result.push(n);
                }
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
        let result = getNeighbours(&[1, 2, 3]);
        assert_eq!(27, result.len());
        let shouldContain222 = result.into_iter().any(|[x,y,z]| x == 2 && y == 2 && z == 2);
        assert_eq!(true, shouldContain222);
        let shouldContain023 = getNeighbours(&[1, 2, 3]).into_iter().any(|[x,y,z]| x == 0 && y == 2 && z == 3);
        assert_eq!(true, shouldContain023);
    }

    #[test]
    fn seventeenPartTwoExampleTest() {
        const INPUT: &str = ".#.
        ..#
        ###";
        let result = INPUT.parse::<Day>().unwrap().partTwo();
        assert_eq!(result, 848);
    }
}