use std::iter::Map;
use std::str::Split;
use lazy_static::lazy_static;
use regex::Regex;
use crate::core::{Day};

pub struct DayOne {
    input: String
}

impl From<String> for DayOne {
    fn from(input: String) -> Self {
       DayOne { input }
    }
}

struct Elf {
    calories: Vec<u32>
}

impl Day for DayOne {
    type R = u32;

    fn day() -> String where Self: Sized {
        "1".to_string()
    }

    fn part_one(&self) -> Self::R {
        self.elfs_iter()
            .map(|elf| elf.calories.iter().sum())
            .max()
            .unwrap_or(0u32)
    }

    fn part_two(&self) -> Self::R {
       let mut elf_calories: Vec<u32> = self.elfs_iter()
           .map(|elf| elf.calories.iter().sum())
           .collect();
        elf_calories.sort();
        elf_calories.reverse();
        return elf_calories.iter().take(3).sum();
    }
}

impl DayOne {
    fn elfs_iter(&self) -> Map<Split<&str>, fn(&str) -> Elf> {
        self.input.split("\n\n")
            .map(|elf|
                Elf {
                    calories: elf.lines()
                        .map(|l| l.trim())
                        .filter_map(|l| l.parse().ok())
                        .collect()
                }
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "1000
                           2000
                           3000

                           4000

                           5000
                           6000

                           7000
                           8000
                           9000

                           10000";
        let actual_res = DayOne::from(input.to_string())
            .part_one();
        assert_eq!(24000, actual_res);
    }

    #[test]
    fn partTwoExampleTest() {
        let input = "1000
                           2000
                           3000

                           4000

                           5000
                           6000

                           7000
                           8000
                           9000

                           10000";
        let actual_res = DayOne::from(input.to_string())
            .part_two();
        assert_eq!(45000, actual_res);
    }
}
