#![allow(non_snake_case)]
use twentytwenty::dayOne;
use twentytwenty::dayTwo;
use twentytwenty::*;
use twentytwenty::core::*;
use std::path::PathBuf;
use std::fs;

fn main() {
    dayOne();
    dayTwo();
    dayThree();

    dayFive();
    daySix();
    daySeven();
    dayEight();
    dayNine();
    dayTen();
    dayEleven();
    dayTwelve();
    dayThirteen();
    dayFourteen();
    //dayFifteen();
    daySixteen();
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
    let dayThree = dayThree::DayThree::new(&input[..]);
    adventOfCodeProblem(&dayThree);
}

fn dayFive() {
    let input = loadInput("Five");
    let dayFive = dayFive::DayFive::new(&input[..]);
    adventOfCodeProblem(&dayFive);
}

fn daySix() {
    let daySix = daySix::DaySix::new();
    adventOfCodeProblem(&daySix);
}

fn daySeven() {
    let daySeven = daySeven::DaySeven::new();
    adventOfCodeProblem(&daySeven);
}

fn dayEight() {
    adventOfCodeProblem(&dayEight::DayEight::new());
}

fn dayNine() {
    adventOfCodeProblem(&dayNine::DayNine::new());
}

fn dayTen() {
    adventOfCodeProblem(&dayTen::Day::new());
}

fn dayEleven() {
    let mut day = dayEleven::Day::new();
    let result = day.partOne();
    println!("-----------------------------------");
    println!("Day {}, part 1: {:?}", day.day(), result);
    //println!("Day {}, part 2: {:?}", day.day(), day.partTwo());
}

fn dayTwelve() {
    adventOfCodeProblem(&dayTwelve::Day::new());
}

fn dayThirteen() {
    adventOfCodeProblem(&dayThirteen::PartOne::new());
}

fn dayFourteen() {
    adventOfCodeProblem(&dayFourteen::PartOne::new());
}

fn dayFifteen() {
    adventOfCodeProblem(&dayFifteen::Day::new());
}

fn daySixteen() {
    adventOfCodeProblem(&daySixteen::Day::new());
}

fn loadInput(day: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources");
    d.push(format!("day{}.txt", day));
    return fs::read_to_string(d).unwrap();
}

fn adventOfCodeProblem<>(day: &dyn AdventOfCodeSolver) {
    println!("-----------------------------------");
    println!("Day {}, part 1: {:?}", day.day(), day.partOne());
    println!("Day {}, part 2: {:?}", day.day(), day.partTwo());
}