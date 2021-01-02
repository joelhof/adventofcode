#![allow(non_snake_case)]

use crate::core::*;
use std::str::FromStr;
use std::collections::HashMap;
use itertools::Itertools;
use std::hash::Hash;

pub struct Day {
    cubeGrid: HashMap<[isize; 3], u8>,
}

impl Day {
    pub fn new() -> Day {
        return loadInput("Seventeen").parse().unwrap();
    }
}

trait GameOfLife<T> where T: Sized + Eq + Clone + Copy + Hash {
    fn getGrid(&self) -> &HashMap<T, u8>;
    fn getNeighbours(coordinate: &T) -> Vec<T>;
    
    fn nextState(&self) -> HashMap<T, u8> {
        // loop over each point that is neighbour to an active cube + all active cubes
        // loop over all active cubes
        let neighbours: Vec<T> = self.getGrid().keys()
            .flat_map(|coordinate| Self::getNeighbours(coordinate))
            .unique()
            .collect();
        //println!("{:?} size {}", neighbours, neighbours.len());
        let result: HashMap<T, u8> = neighbours.into_iter()
        // for each such cube set new state according to rule
            .map(|cube| (cube, self.rule(&cube)))
            .filter(|(_coordinate, state)| *state > 0)
            .collect();
        //println!("--------------");
        //println!("{:?}, grid size {}", result, result.len());
        return result;
    }

    fn rule(&self, coordinate: &T) -> u8 {
        let neighbours = Self::getNeighbours(coordinate);
        let activeNeighbours: usize = neighbours.into_iter()
            .filter(|cube| cube != coordinate)
            .filter(|cube| self.getGrid().contains_key(cube))
            .count();
        if self.getGrid().contains_key(coordinate) {
            return if activeNeighbours == 2 || activeNeighbours == 3 { 1 } else { 0 };
        } else {
            return if activeNeighbours == 3 { 1 } else { 0 };
        }
    }
}

struct PartOne {
    cubeGrid: HashMap<[isize; 3], u8>
}

impl GameOfLife<[isize; 3]> for PartOne {
    fn getGrid(&self) -> &HashMap<[isize; 3], u8> {
        return &self.cubeGrid;
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
}

struct PartTwo {
    cubeGrid:  HashMap<[isize; 4], u8>
}

impl GameOfLife<[isize; 4]> for PartTwo {
    fn getGrid(&self) -> &HashMap<[isize; 4], u8> {
        return &self.cubeGrid;
    }

    fn getNeighbours(coordinate: &[isize; 4]) -> Vec<[isize; 4]> {
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
        let mut conwayGrid = PartOne {
            cubeGrid: self.cubeGrid.clone()
        };
        for _i in 0..6 {
            conwayGrid.cubeGrid = conwayGrid.nextState();
        }
        return conwayGrid.getGrid().values().map(|x| *x as u64).sum::<u64>();
    }

    fn partTwo(&self) -> u64 {
        let mut conwayCubes4d = PartTwo {
            cubeGrid : self.cubeGrid.clone().iter()
                .map(|([x,y,z], v)| ([*x,*y,*z,0isize], *v))
                .collect()
        };
        for _i in 0..6 {
            conwayCubes4d.cubeGrid = conwayCubes4d.nextState();
        }
        return conwayCubes4d.getGrid().values().map(|x| *x as u64).sum::<u64>();
    }
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
        let result = PartOne::getNeighbours(&[1, 2, 3]);
        assert_eq!(27, result.len());
        let shouldContain222 = result.into_iter().any(|[x,y,z]| x == 2 && y == 2 && z == 2);
        assert_eq!(true, shouldContain222);
        let shouldContain023 = PartOne::getNeighbours(&[1, 2, 3]).into_iter().any(|[x,y,z]| x == 0 && y == 2 && z == 3);
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