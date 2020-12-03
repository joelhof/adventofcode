#![allow(non_snake_case)]
use twentytwenty::dayOne;
use twentytwenty::dayTwo;
use twentytwenty::*;
use std::path::PathBuf;
use std::fs;

fn main() {
    dayOne();
    dayTwo();
    dayThree();
}

fn dayOne() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources");
    d.push("dayOne.txt");
    let day_one_input = fs::read_to_string(d).unwrap();
    let result = dayOne::day_one(&day_one_input[..]);
    println!("Day One, part 1: {:?}", result);

    let result = dayOne::dayOnePartTwo(&day_one_input[..]);
    println!("Day One, part 2: {:?}", result);
}

fn dayTwo() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources");
    d.push("dayTwo.txt");
    let day_two_input = fs::read_to_string(d).unwrap();
    let result = dayTwo::solve(&day_two_input[..]);
    println!("Day Two, part 1: {:?}", result);

    let result = dayTwo::partTwo(&day_two_input[..]);
    println!("Day Two, part 2: {:?}", result);
}

fn dayThree() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources");
    d.push("dayThree.txt");
    let input = fs::read_to_string(d).unwrap();
    let result = dayThree::partOne(&input[..]);
    println!("Day Three, part 1: {:?}", result);
}