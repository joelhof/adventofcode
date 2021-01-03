#![allow(non_snake_case)]

use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;
use std::hash::Hash;
use itertools::Itertools;

pub trait AdventOfCodeSolver {
    fn partOne(&self) -> u64 {
        return 0;
    }
    fn partTwo(&self) -> u64 {
        return 0;
    }
    fn day(&self) -> &str;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coordinate(pub isize, pub isize);

pub fn loadInput(day: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources");
    d.push(format!("day{}.txt", day));
    return fs::read_to_string(d).unwrap();
}

pub trait GameOfLife<T> where T: Sized + Eq + Clone + Copy + Hash {
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