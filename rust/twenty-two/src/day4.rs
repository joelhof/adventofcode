use std::iter::Map;
use std::ops::RangeInclusive;
use std::str::Lines;
use crate::core::{Day};

pub struct DayFour {
    input: String
}

impl From<String> for DayFour {
    fn from(input: String) -> Self {
        DayFour { input }
    }
}

impl Day for DayFour {
    type R = usize;

    fn day() -> String where Self: Sized {
        "4".to_string()
    }

    fn part_one(&self) -> Self::R {
        self.create_elf_assignments()
            .filter(|(e1, e2)|
                              (e1.contains(&e2.start()) && e1.contains(&e2.end()))
                                  || (e2.contains(&e1.start()) && e2.contains(&e1.end())))
            .count()
    }

    fn part_two(&self) -> Self::R {
        self.create_elf_assignments()
            .filter(|(e1, e2)|
                e1.contains(e2.start())
                || e1.contains(e2.end())
                || e2.contains(e1.start())
                || e2.contains(e1.end())
            )
            .count()
    }
}

impl DayFour {

    fn create_range(input: &str) -> RangeInclusive<u32> {
        let mut s = input.split("-");
        let start = s.next().map(|v| v.parse().ok()).unwrap().expect("A range needs a start value!");
        let end = s.next().map(|v| v.parse().ok()).unwrap().expect("A range needs a start value!");
        start..=end
    }

    fn create_elf_assignments(&self) -> Map<Lines, fn(&str) -> (RangeInclusive<u32>, RangeInclusive<u32>)> {
        self.input.lines().map(|l| {
            let mut s = l.split(",");
            let elf1 = s.next()
                .map(|r| DayFour::create_range(r)).expect("Failed to construct elf 1");
            let elf2 = s.next()
                .map(|r| DayFour::create_range(r)).expect("Failed to construct elf 2");
            (elf1, elf2)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let actual_res = DayFour::from(input.to_string())
            .part_one();
        assert_eq!(2, actual_res);
    }

    #[test]
    fn partTwoExampleTest() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let actual_res = DayFour::from(input.to_string())
            .part_two();
        assert_eq!(4, actual_res);
    }
}
