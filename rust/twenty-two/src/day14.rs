use std::collections::HashSet;
use crate::core::{Day};
use lazy_static::lazy_static;
use regex::Regex;

pub struct DayFourteen {
    input: String
}

impl From<String> for DayFourteen {
    fn from(input: String) -> Self {
       DayFourteen { input }
    }
}

lazy_static! {
    static ref COORDINATE_RE: Regex = Regex::new(r"([0-9]+,[0-9]+)").unwrap();
}

struct SandCave {
    rock: HashSet<Coordinate>,
    left_boundary: i32,
    right_boundary: i32,
    bottom_boundary: i32,
}

impl From<&str> for SandCave {
    fn from(input: &str) -> Self {
        let rocks: HashSet<Coordinate> = input.lines()
              .map(|l| DayFourteen::parsePoint(l))
              .flat_map(|p| p.windows(2)
                  .flat_map(|pair| pair[0].on_path(&pair[1]))
                  .collect::<HashSet<Coordinate>>()
              ).collect();

        let right_boundary = rocks.iter().map(|pos| pos.0).max().unwrap();
        let left_boundary = rocks.iter().map(|pos| pos.0).min().unwrap();
        let bottom_boundary = rocks.iter().map(|pos| pos.1).max().unwrap();
        SandCave { rock: rocks, left_boundary, right_boundary, bottom_boundary }
    }
}

impl Day for DayFourteen {
    type R = u32;

    fn day() -> String where Self: Sized {
        "14".to_string()
    }

    fn part_one(&self) -> Self::R {
        let cave = SandCave::from(self.input.as_str());
        let mut occupied = cave.rock;
        let mut grain_count= 0;
        let mut sand_resting = true;
        while sand_resting {
            let mut grain_pos =Coordinate(500, 0);
            // depth first search
            let mut grain_falling = true;
            while grain_falling {
                if grain_pos.0 < cave.left_boundary || grain_pos.0 > cave.right_boundary || grain_pos.1 > cave.bottom_boundary {
                    grain_falling = false;
                    sand_resting = false;
                    break
                }
                grain_pos = match grain_pos {
                    Coordinate(x, y) if !occupied.contains(&Coordinate(x, y + 1)) => {
                        let p = Coordinate(x, y + 1);
                        p
                    },
                    Coordinate(x, y) if !occupied.contains(&Coordinate(x - 1, y + 1)) => {
                        let p = Coordinate(x - 1, y + 1);
                        p
                    },
                    Coordinate(x, y) if !occupied.contains(&Coordinate(x + 1, y + 1)) => {
                        let p = Coordinate(x + 1, y + 1);
                        p
                    },
                    Coordinate(x,y) => {
                        grain_count = grain_count + 1;
                        grain_falling = false;
                        occupied.insert(Coordinate(x,y));
                        grain_pos
                    }
                };
            };
        }
        grain_count
    }

    fn part_two(&self) -> Self::R {
        let cave = SandCave::from(self.input.as_str());
        let mut occupied = cave.rock;
        let mut grain_count= 0;
        let mut sand_resting = true;
        while sand_resting {
            let mut grain_pos = Coordinate(500, 0);
            // depth first search
            if occupied.contains(&grain_pos) {
                break;
            }
            let mut grain_falling = true;
            while grain_falling {
                if grain_pos.1 == cave.bottom_boundary+1 {
                    occupied.insert(grain_pos);
                    grain_count = grain_count + 1;
                    grain_falling = false;
                    break;
                }
                grain_pos = match grain_pos {
                    Coordinate(x, y) if !occupied.contains(&Coordinate(x, y + 1)) => {
                        let p = Coordinate(x, y + 1);
                        //println!("pos {:?} straight below is available", p);
                        p
                    },
                    Coordinate(x, y) if !occupied.contains(&Coordinate(x - 1, y + 1)) => {
                        let p = Coordinate(x - 1, y + 1);
                        //println!("pos {:?} below to the left is available", p);
                        p
                    },
                    Coordinate(x, y) if !occupied.contains(&Coordinate(x + 1, y + 1)) => {
                        let p = Coordinate(x + 1, y + 1);
                        //println!("pos {:?} below to the right is available", p);
                        p
                    },
                    Coordinate(x,y) => {
                        //println!("all options occupied, grain rests at {:?}", grain_pos);
                        grain_count = grain_count + 1;
                        grain_falling = false;
                        occupied.insert(Coordinate(x,y));
                        break;
                    }
                };
            };
        }
        grain_count
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
struct Coordinate(i32,i32);

impl Coordinate {
    fn on_path(&self, end: &Coordinate) -> Vec<Coordinate> {
        let mut res = vec![];

        let horizontal = end.0 - self.0;
        let vertical = end.1 - self.1;

        if horizontal == 0 {
            let start = if vertical > 0 {self.1} else {self.1+vertical};
            let end = if vertical > 0 {self.1+vertical} else {self.1};
            for y in start..=end {
                res.push(Coordinate(self.0, y));
            }
        }
        else if vertical == 0 {
            let start = if horizontal > 0 {self.0} else {self.0+horizontal};
            let end = if horizontal > 0 {self.0+horizontal} else {self.0};
            for x in start..=end {
                res.push(Coordinate(x, self.1));
            }
        }
        else {
            panic!("Path is neither horizontal nor vertical! start {:?} end {:?}", self, end);
        }
        res
    }
}

impl DayFourteen {
    fn parsePoint(l: &str) -> Vec<Coordinate> {
        COORDINATE_RE.captures_iter(l).map(|point| {
            let coord: Vec<i32> = point[0].split(",")
                .map(|a| a.parse().unwrap())
                .collect();
            Coordinate{ 0: coord[0], 1: coord[1] }
        }
        ).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9 ";
        let actual_res = DayFourteen::from(input.to_string()).part_one();
        assert_eq!(24, actual_res);
    }

    #[test]
    fn partTwoExampleTest() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9 ";
        let actual_res = DayFourteen::from(input.to_string()).part_two();
        assert_eq!(93, actual_res);
    }
}
