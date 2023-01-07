use std::collections::{HashMap, VecDeque};
use crate::core::{Day};
use regex::Regex;
use lazy_static::lazy_static;

pub struct DayFive {
    input: String
}

impl From<String> for DayFive {
    fn from(input: String) -> Self {
       DayFive { input }
    }
}

lazy_static! {
    static ref BLOCK_SPLIT_RE: Regex = Regex::new(r"\n\n").unwrap();
    static ref MOVE_RE: Regex = Regex::new(r"(\d+)").unwrap();

}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    qty: usize
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let c: Vec<usize> = MOVE_RE.captures_iter(input)
            .filter_map(|c| (&c[0]).parse().ok())
            .collect();
        Move { from: c[1], to: c[2], qty: c[0] }
    }
}

impl Day for DayFive {
    type R = String;

    fn day() -> String where Self: Sized {
       "5".to_string()
    }

    fn part_one(&self) -> Self::R {
        let (mut stacks, moves) = self.parse_stacks_and_moves();

        for m in moves.iter() {
            let from = stacks.get_mut(&m.from).unwrap();
            let moved: Vec<char> = from.drain(0..m.qty).collect();
            let to = stacks.get_mut(&m.to).unwrap();
            //println!("move {:?} to {:?}", moved, to);
            for ccrate in moved.iter() {
                to.push_front(*ccrate);
            }
        }
        //println!("Stacks: {:?}", stacks);
        Self::get_top_crates(&mut stacks)
    }

    fn part_two(&self) -> Self::R {
        let (mut stacks, moves) = self.parse_stacks_and_moves();

        for m in moves.iter() {
            //println!("Stacks: {:?}", stacks);
            let from = stacks.get_mut(&m.from).unwrap();
            let mut moved: Vec<char> = from.drain(0..m.qty).collect();
            moved.reverse();
            let to = stacks.get_mut(&m.to).unwrap();
            //println!("move {:?} to {:?}", moved, to);
            for ccrate in moved.iter() {
                to.push_front(*ccrate);
            }
        }
        //println!("Stacks: {:?}", stacks);
        Self::get_top_crates(&mut stacks)
    }
}

impl DayFive {
    fn parse_stacks_and_moves(&self) -> (HashMap<usize, VecDeque<char>>, Vec<Move>) {
        let mut s = BLOCK_SPLIT_RE.split(&self.input);

        let initial_config = s.next().unwrap();
        let footer = initial_config.lines().last().unwrap();
        let stack_idx: HashMap<usize, usize> = footer.chars().enumerate()
            .filter(|(_i, n)| n.is_numeric())
            .map(|(i, n)| (i, n.to_digit(10).unwrap() as usize)).collect();
        let mut stacks: HashMap<usize, VecDeque<char>> = footer.chars().enumerate()
            .filter(|(_i, n)| n.is_numeric())
            .map(|(i, _n)| (*stack_idx.get(&i).expect("Every index should have a stack id mapping"), VecDeque::new()))
            .collect();

        for l in initial_config.lines() {
            for (j, c) in l.chars().enumerate() {
                if c.is_ascii_uppercase() {
                    let idx = stack_idx.get(&j).unwrap();
                    let stack = stacks.get_mut(idx).unwrap();
                    stack.push_back(c);
                }
            }
        }
        //println!("Stacks: {:?}", stacks);

        let moves: Vec<Move> = s.next().map(|m|
            m.lines().map(|l| Move::from(l)).collect()
        ).unwrap();
        (stacks, moves)
    }

    fn get_top_crates(stacks: &mut HashMap<usize, VecDeque<char>>) -> String {
        let mut v: Vec<(&usize, &VecDeque<char>)> = stacks.iter().collect();
        v.sort_by(|(i, _s1), (j, _s2)| i.cmp(j));
        v.iter().filter_map(|(_i, stack)| stack.front()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let actual_res = DayFive::from(input.to_string())
            .part_one();
        assert_eq!("CMZ", actual_res);
    }

    #[test]
    fn partTwoExampleTest() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let actual_res = DayFive::from(input.to_string())
            .part_two();
        assert_eq!("MCD", actual_res);
    }
}
