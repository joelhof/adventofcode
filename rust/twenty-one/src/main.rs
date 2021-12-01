#![allow(non_snake_case)]

use std::path::PathBuf;
use std::fs;

fn main() {
    dayOne();
}

fn dayOne() {
    let input: String = loadInput("One");

    let res = dayOnePartOne(&input);
    println!("{}", res);
    println!("{}", dayOnePartTwo(&input));
}

fn dayOnePartOne(input: &str) -> i32 {
    let lines: Result<Vec<i32>, _> = input.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse())
        .collect();
    
    let res: i32 = lines.unwrap().windows(2)
        .map(|chunk| 
            match chunk.len() {
                2 => if chunk[1] > chunk[0] { return 1; } else { return 0; }, 
                _ => 0
            }
        ).sum();
    return res;
}

fn dayOnePartTwo(input: &str) -> i32 {
    let lines: Result<Vec<i32>, _> = input.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse())
        .collect();
    
    let res: i32 = lines.unwrap().windows(3)
        .map(|chunk| chunk.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|chunk| 
            match chunk.len() {
                2 => if chunk[1] > chunk[0] { return 1; } else { return 0; }, 
                _ => 0
            }
        ).sum();
    return res;
}

pub fn loadInput(day: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources");
    d.push(format!("day{}.txt", day));
    return fs::read_to_string(d).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExample() {
        let input = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263
        ";
        let res = dayOnePartOne(input);
        assert_eq!(7, res);
    }

    #[test]
    fn part2Example() {
        let input = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263
        ";
        let res = dayOnePartTwo(input);
        assert_eq!(5, res);
    }
}