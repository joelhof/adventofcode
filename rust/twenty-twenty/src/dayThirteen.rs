#![allow(non_snake_case)]

use crate::core::*;

pub struct Day {
    arrival: u64,
    schedule: Vec<u64>
}

impl Day {
    fn init(input: &str) -> Day {
        let mut lines = input.split("\n")
            .map(|line| line.trim());
        let a: u64 = lines.next().unwrap().parse().unwrap();
        let schedule: Vec<u64> = lines.next().unwrap()
            .replace("x", "")
            .split(",")
            .filter_map(|s| Some(s.parse()))
            .filter_map(Result::ok)
            .collect();
            
        return Day {
            arrival: a,
            schedule: schedule
        };
    }

    pub fn new() -> Day {
        return Day::init(&loadInput("Thirteen"));
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Thirteen";
    }

    fn partOne(&self) -> u64 {
        //println!("{:?}", self.schedule);
        //println!("{:?}", self.arrival);
        let mut ratios: Vec<(u64, u64)> = self.schedule[..].into_iter()
            .map(|freq| (freq, self.arrival / freq, self.arrival % freq))
            .map(|(f, n, m)| (multiple((*f, n, m)) - self.arrival, *f))
            .collect();
        ratios.sort_by(|(wait1, _id1), (wait2, _id2)| wait1.cmp(&wait2));
        //println!("{:?}", ratios);
        return match ratios.first() {
            Some((departure, id)) => departure * id,
            None => 0
        };
    }
 }

fn multiple(input: (u64, u64, u64)) -> u64 {
    let (f, n, m) = input;
    if m > 0 {
        return f * (n + 1);
    } else {
        return f * n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn thirteenPartOneExampleTest() {
        const INPUT: &str = "939
        7,13,x,x,59,x,31,19";
        let result = Day::init(INPUT).partOne();
        assert_eq!(result, 295);
    }
}