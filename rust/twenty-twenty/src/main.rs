#![allow(non_snake_case)]
use twentytwenty::dayOne;
use twentytwenty::dayTwo;
use twentytwenty::*;
use twentytwenty::dayFive::*;
use std::path::PathBuf;
use std::fs;

fn main() {
    dayOne();
    dayTwo();
    dayThree();
    dayFive();
}

fn dayOne() {
    let day_one_input = loadInput("One");
    let result = dayOne::day_one(&day_one_input[..]);
    println!("Day One, part 1: {:?}", result);

    let result = dayOne::dayOnePartTwo(&day_one_input[..]);
    println!("Day One, part 2: {:?}", result);
}

fn dayTwo() {
    let day_two_input = loadInput("Two");
    let result = dayTwo::solve(&day_two_input[..]);
    println!("Day Two, part 1: {:?}", result);

    let result = dayTwo::partTwo(&day_two_input[..]);
    println!("Day Two, part 2: {:?}", result);
}

fn dayThree() {
    let input = loadInput("Three");
    let result = dayThree::partOne(&input[..]);
    println!("Day Three, part 1: {:?}", result);

    let result = dayThree::partTwo(&input[..]);
    println!("Day Three, part 2: {:?}", result);
}

fn dayFive() {
    let input = loadInput("Five");
    let dayFive = dayFive::DayFive::new(&input[..]);
    let result = dayFive.partOne();
    println!("Day Five, part 1: {:?}", result);

    let result = dayFive.partTwo();
    println!("Day Five, part 2: {:?}", result);
}

fn loadInput(day: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources");
    d.push(format!("day{}.txt", day));
    let input = fs::read_to_string(d).unwrap();
    return input;
}