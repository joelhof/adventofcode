#![allow(non_snake_case)]

use std::path::PathBuf;
use std::fs;
use twentyone::*;

fn main() {
    dayOne();
    {
        let res = dayTwo::partOne(&loadInput("Two"));
        println!("Day 2, part 1: {}", res);
    }
    {
        let res = dayTwo::partTwo(&loadInput("Two"));
        println!("Day 2, part 2: {}", res);
    }
    {
        let res = dayThree::partOne(&loadInput("Three"));
        println!("Day 3, part 1: {}", res);
    }
    {
        let res = dayThree::partTwo(&loadInput("Three"));
        println!("Day 3, part 2: {}", res);
    }
    {
        let res = dayFour::partOne(&loadInput("Four"));
        println!("Day 4, part 1: {}", res);
    }
    {
        let res = dayFour::partTwo(&loadInput("Four"));
        println!("Day 4, part 2: {}", res);
    }
    {
        let res = dayFive::partOne(&loadInput("Five"));
        println!("Day 5, part 1: {}", res);
    }
    {
        let res = dayFive::partTwo(&loadInput("Five"));
        println!("Day 5, part 2: {}", res);
    }
    {
        let res = daySix::partOne(&loadInput("Six"));
        println!("Day 6, part 1: {}", res);
    }
    {
        let res = daySix::partTwo(&loadInput("Six"));
        println!("Day 6, part 2: {}", res);
    }
    {
        let res = daySeven::partOne(&loadInput("Seven"));
        println!("Day 7, part 1: {}", res);
    }
    {
        let res = daySeven::partTwo(&loadInput("Seven"));
        println!("Day 7, part 2: {}", res);
    }
    {
        let res = dayEight::partOne(&loadInput("Eight"));
        println!("Day 8, part 1: {}", res);
    }
    {
        let res = dayNine::partOne(&loadInput("Nine"));
        println!("Day 9, part 1: {}", res);
    }
    {
        let res = dayNine::partTwo(&loadInput("Nine"));
        println!("Day 9, part 2: {}", res);
    }
    {
        let res = dayTen::partOne(&loadInput("Ten"));
        println!("Day 10, part 1: {}", res);
    }
    {
        let res = dayTen::partTwo(&loadInput("Ten"));
        println!("Day 10, part 2: {}", res);
    }
    {
        let res = dayEleven::partOne(&loadInput("Eleven"));
        println!("Day 11, part 1: {}", res);
    }
    {
        let res = dayEleven::partTwo(&loadInput("Eleven"));
        println!("Day 11, part 2: {}", res);
    }
    {
        let res = dayTwelve::partOne(&loadInput("Twelve"));
        println!("Day 12, part 1: {}", res);
    }
    // {
    //     let res = dayTwelve::partTwo(&loadInput("Twelve"));
    //     println!("Day 12, part 2: {}", res);
    // }
    {
        let res = dayThirteen::partOne(&loadInput("Thirteen"));
        println!("Day 13, part 1: {}", res);
    }
    {
        let res = dayThirteen::partTwo(&loadInput("Thirteen"));
        println!("Day 13, part 2: {}", res);
    }
    {
        let res = dayFourteen::partOne(&loadInput("Fourteen"));
        println!("Day 14, part 1: {}", res);
    }
    {
        let res = dayFourteen::partTwo(&loadInput("Fourteen"));
        println!("Day 14, part 2: {}", res);
    }
    {
        let res = dayFifteen::partOne(&loadInput("Fifteen"));
        println!("Day 15, part 1: {}", res);
    }
    {
        let res = dayFifteen::partTwo(&loadInput("Fifteen"));
        println!("Day 15, part 2: {}", res);
    }
    {
        let res = daySixteen::partOne(&loadInput("Sixteen"));
        println!("Day 16, part 1: {}", res);
    }
    {
        let res = daySixteen::partTwo(&loadInput("Sixteen"));
        println!("Day 16, part 2: {}", res);
    }
    {
        let res = daySeventeen::partOne(&loadInput("Seventeen"));
        println!("Day 17, part 1: {}", res);
    }
    {
        let res = daySeventeen::partTwo(&loadInput("Seventeen"));
        println!("Day 17, part 2: {}", res);
    }
    {
        let res = dayEighteen::partOne(&loadInput("Eighteen"));
        println!("Day 18, part 1: {}", res);
    }
    {
        let res = dayEighteen::partTwo(&loadInput("Eighteen"));
        println!("Day 18, part 2: {}", res);
    }
    {
        let res = dayNineteen::partOne(&loadInput("Nineteen"));
        println!("Day 19, part 1: {}", res);
    }
    {
        let res = dayNineteen::partTwo(&loadInput("Nineteen"));
        println!("Day 19, part 2: {}", res);
    }
    {
        let res = dayTwenty::partOne(&loadInput("Twenty"));
        println!("Day 20, part 1: {}", res);
    }
    {
        let res = dayTwenty::partTwo(&loadInput("Twenty"));
        println!("Day 20, part 2: {}", res);
    }
    {
        let res = dayTwentyOne::partOne(&loadInput("TwentyOne"));
        println!("Day 21, part 1: {}", res);
    }
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