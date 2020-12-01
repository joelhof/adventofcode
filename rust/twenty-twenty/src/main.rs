use twentytwenty::dayOne;
use std::path::PathBuf;
use std::fs;

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources");
    d.push("dayOne.txt");
    let day_one_input = fs::read_to_string(d);
    let result = dayOne::day_one(&day_one_input.unwrap()[..]);
    println!("Day One, part 1: {:?}", result);
}