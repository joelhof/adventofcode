#![allow(non_snake_case)]

use std::path::PathBuf;
use std::fs;

pub trait AdventOfCodeProblem {
    fn partOne(&self) -> u32;
    fn partTwo(&self) -> u32;
}

pub fn loadInput(day: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources");
    d.push(format!("day{}.txt", day));
    return fs::read_to_string(d).unwrap();
}