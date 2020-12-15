#![allow(non_snake_case)]

use crate::core::*;
use std::collections::HashMap;

pub struct Day {
    input: String
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Fifteen";
    }

    fn partOne(&self) -> u64 {
        let mut spoken: HashMap<usize, Vec<usize>> = self.input.split(",")
            .filter_map(|c| Some(c.parse::<usize>()))
            .map(|r| r.unwrap())
            .enumerate()
            .map(|(i, r)| {
                let mut v = Vec::new();
                v.push(i);
                (r, v)
            })
            .collect();
        let mut previous = self.input.split(",")
            .filter_map(|c| Some(c.parse::<usize>()))
            .map(|r| r.unwrap())
            .last()
            .unwrap();
        for i in spoken.len()..=2019 {
            //println!("-------turn {} last spoken nr {}", i, previous);
            let next = match isSpoken(&spoken, &previous, &i) {
                false => 0,
                true => diff(spoken.get(&previous))
            };
            let mut order = match spoken.get(&next) {
                None => Vec::new(),
                Some(v) => v.to_vec()
            };
            //println!("next number is {}", next);
            order.push(i);
            //println!("turns {} was spoken: {:?}", next, order);
            spoken.insert(next, order);
            previous = next;
            //println!("---------------");
        }
        //println!("{}", previous);
        return previous as u64;
    }
 }

fn isSpoken(spoken: &HashMap<usize, Vec<usize>>, word: &usize, currentTurn: &usize) -> bool {
    //println!("{:?} {}", spoken, word);
    return match spoken.get(word) {
        Some(v) => { v.len() > 1 || *v.last().unwrap() != (currentTurn - 1)},
        None => false
    }
}

fn diff(order: Option<&Vec<usize>>) -> usize {
    //println!("in diff: {:?}, {}", order, order.unwrap().len());
    let delta =  match order {
        Some(o) => {
            let lastTwo: Vec<&usize> = o.iter().rev().take(2).collect();
            //println!("last two {:?}", lastTwo);
            return lastTwo[0] - lastTwo[1];
        },
        None => 0
    };
    //println!("{:?}, diff:  {}", order ,delta);
    return delta;
}

 impl Day {
    fn init(input: &str) -> Day {
        return Day {
            input: input.to_string()
        };
    }

    pub fn new() -> Day {
        return Day::init("0,5,4,1,10,14,7");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn fifteenPartOneExampleTest() {
        const INPUT: &str = "0,3,6";
        let result = Day::init(INPUT).partOne();
        assert_eq!(result, 436);
    }

    #[test]
    fn fifteenPartOneExampleTest2() {
        const INPUT: &str = "1,3,2";
        let result = Day::init(INPUT).partOne();
        assert_eq!(result, 1);
    }

    #[test]
    fn fifteenPartOneExampleTest3() {
        const INPUT: &str = "3,1,2";
        let result = Day::init(INPUT).partOne();
        assert_eq!(result, 1836);
    }
}