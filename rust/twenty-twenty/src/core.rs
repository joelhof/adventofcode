#![allow(non_snake_case)]

use std::path::PathBuf;
use std::fs;

pub trait AdventOfCodeSolver {
    fn partOne(&self) -> u64 {
        return 0;
    }
    fn partTwo(&self) -> u64 {
        return 0;
    }
    fn day(&self) -> &str;
}

pub fn loadInput(day: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources");
    d.push(format!("day{}.txt", day));
    return fs::read_to_string(d).unwrap();
}