use std::collections::HashSet;
use crate::core::{Day};

pub struct DayThree {
    input: String
}

impl From<String> for DayThree {
    fn from(input: String) -> Self {
        DayThree { input }
    }
}

#[derive(Debug)]
struct Rucksack(Vec<char>, Vec<char>);

impl From<&str> for Rucksack {
    fn from(input: &str) -> Self {
        let half = input.len() / 2;
        let halves = input.split_at(half);
        assert_eq!(halves.0.len(), halves.1.len(), "Rucksack item lists must be equal");
        return Rucksack(
            halves.0.chars().collect(),
            halves.1.chars().collect()
        );
    }
}

impl Rucksack {
    fn find_shared_items(&self) -> HashSet<char> {
        self.0.iter()
            .filter(|c| self.1.contains(c))
            .map(|c| c.clone())
            .collect()
    }
}

impl Day for DayThree {
    type R = u32;

    fn day() -> String where Self: Sized {
        "3".to_string()
    }

    fn part_one(&self) -> Self::R {
        self.input.lines()
            .map(|l| Rucksack::from(l))
            .map(|rucksack| {
                rucksack.find_shared_items()
                    .iter()
                    .map(|item|
                        if item.is_ascii_lowercase() { *item as u32 -96 }
                        else if item.is_ascii_uppercase() { *item as u32 - 38 }
                        else {
                            panic!("Unsupported char: {}", item);
                        }
                    )
                    .sum::<u32>()
            })
            .sum()
    }

    fn part_two(&self) -> Self::R {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let actual_res = DayThree::from(input.to_string())
            .part_one();
        assert_eq!(157, actual_res);
    }
}