use std::collections::HashSet;
use crate::core::{Day};
use crate::day9::RopeMove::{DOWN, LEFT, RIGHT, UP};

pub struct DayNine {
    input: String
}

impl From<String> for DayNine {
    fn from(input: String) -> Self {
        DayNine { input }
    }
}

impl Day for DayNine {
    type R = usize;

    fn day() -> String where Self: Sized {
        "9".to_string()
    }

    fn part_one(&self) -> Self::R {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut state = State { head_pos: (0,0), tail_pos: (0,0) };
        for rope_move in self.input.lines().map(|l| l.trim()) {
            let m = RopeMove::from(rope_move);
            state.nextState(&mut visited, m);

        }
        visited.len()
    }

    fn part_two(&self) -> Self::R {
        todo!()
    }
}

struct State {
    head_pos: (i32, i32),
    tail_pos: (i32, i32)
}

impl State {

    fn nextState(&mut self, visited: &mut HashSet<(i32,i32)>, m: RopeMove) {
        for _step in 0..m.getMoves() {
            let head_delta = match m {
                UP(_) => (0, 1),
                DOWN(_) => (0, -1),
                LEFT(_) => (-1, 0),
                RIGHT(_) => (1, 0),
            };
            self.head_pos = (self.head_pos.0 + head_delta.0, self.head_pos.1 + head_delta.1);
            let diff = (
                self.head_pos.0 - self.tail_pos.0,
                self.head_pos.1 - self.tail_pos.1
            );
            let tail_delta = match diff {
                (x,y) if x.abs() == 2 && y == 0 => head_delta,
                (x,y) if y.abs() == 2 && x == 0 => head_delta,
                (2,y) => (1, y),
                (x,2) => (x, 1),
                (-2,y) => (-1, y),
                (x,-2) => (x, -1),
                (_,_) => (0,0)
            };
            self.tail_pos = (self.tail_pos.0 + tail_delta.0, self.tail_pos.1 + tail_delta.1);
            visited.insert(self.tail_pos);
        }
    }
}

#[derive(Debug)]
enum RopeMove {
    UP(usize),
    DOWN(usize),
    LEFT(usize),
    RIGHT(usize)
}

impl RopeMove {
    fn getMoves(&self) -> usize {
        match self {
            UP(x) => *x,
            DOWN(x) => *x,
            LEFT(x) => *x,
            RIGHT(x) => *x
        }
    }
}

impl From<&str> for RopeMove {
    fn from(input: &str) -> Self {
       let mut split = input.split(" ");
        match split.next() {
            Some("U") => UP(split.next().map_or(0, |d| d.parse().unwrap())),
            Some("D") => DOWN(split.next().map_or(0, |d| d.parse().unwrap())),
            Some("L") => LEFT(split.next().map_or(0, |d| d.parse().unwrap())),
            Some("R") => RIGHT(split.next().map_or(0, |d| d.parse().unwrap())),
            _ => panic!("Unable to parse &str {} into RopeMove", input)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2";
        let actual_res = DayNine::from(input.to_string()).part_one();
        assert_eq!(13, actual_res);
    }
}