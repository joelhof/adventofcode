use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

pub fn load_input(day: &str) -> std::io::Result<String> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resource");
    d.push(format!("day{}.txt", day));
    return fs::read_to_string(d);
}

pub trait Day {
    type R: Display;

    fn day() -> String where Self: Sized;
    fn part_one(&self) -> Self::R;
    fn part_two(&self) -> Self::R;
    fn from(input: String) -> Box<dyn Day<R=Self::R>> where Self: Sized;
    fn solve() where Self: Sized {
        let input = load_input(&Self::day());
        let day = match input {
            Ok(problem) => Self::from(problem),
            Err(_) => {
                println!("Failed to read input for day {}", Self::day());
                panic!()
            }
        };
        println!("Day {}, part 1: {}", Self::day(), day.part_one());
        println!("Day {}, part 2: {}", Self::day(), day.part_two());
    }

}