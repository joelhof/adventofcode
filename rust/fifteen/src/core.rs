use std::fs;
use std::path::PathBuf;

pub fn load_input(day: &str) -> std::io::Result<String> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resource");
    d.push(format!("day{}.txt", day));
    return fs::read_to_string(d);
}

pub trait Day {
    type R;

    fn part_one() -> Self::R;
    fn part_two() -> Self::R;
}