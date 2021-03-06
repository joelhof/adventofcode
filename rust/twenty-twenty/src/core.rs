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

impl Coordinate {
    pub fn manhattan(&self, other: Coordinate) -> usize {
        return ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize;
    }
}

pub fn loadInput(day: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources");
    d.push(format!("day{}.txt", day));
    return fs::read_to_string(d).unwrap();
}
