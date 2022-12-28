use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;
use crate::core::{Day};

pub struct DayFifteen {
    input: String
}

impl From<String> for DayFifteen {
    fn from(input: String) -> Self {
       DayFifteen { input }
    }
}

impl Day for DayFifteen {
    type R = u32;

    fn day() -> String where Self: Sized {
        "15".to_string()
    }

    fn part_one(&self) -> Self::R {
        self.count_coverage(2000000)
    }

    fn part_two(&self) -> Self::R {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    range: u32
}

impl Sensor {
    fn in_range(&self, pos: (i32, i32)) -> bool {
        let dist = ((pos.0 - self.pos.0).abs() + (pos.1 - self.pos.1).abs()) as u32;
        //println!("pos: {:?} is in range from {:?}: {}", pos, self, dist <= self.range);
        dist <= self.range
    }
}

lazy_static! {
    static ref COORDINATE_RE: Regex = Regex::new(r"(-?[0-9]+)").unwrap();
}

impl From<&str> for Sensor {
    fn from(input: &str) -> Self {
          let nrs: Vec<i32> = COORDINATE_RE.captures_iter(input)
              .filter_map(|cap| cap.get(0).map(|m| m.as_str()))
              .map(|x| x.parse().unwrap())
              .collect();
        Sensor {
            pos: (nrs[0],nrs[1]),
            beacon: (nrs[2],nrs[3]),
            range: ((nrs[2] - nrs[0]).abs() + (nrs[3] - nrs[1]).abs()) as u32
        }
    }
}

impl DayFifteen {
    fn count_coverage(&self, row: i32) -> u32 {
        let sensors: Vec<Sensor> = self.input.lines()
            .map(|l| l.trim())
            .map(|l| Sensor::from(l))
            .collect();
        let beacons: HashSet<(i32, i32)> = sensors.iter().map(|s| s.beacon.clone()).collect();
        //println!("Sensors: {:?}", sensors);
        let min = sensors.iter().map(|s| s.pos.0 - s.range as i32).min().unwrap();
        let max = sensors.iter().map(|s| s.pos.0 + s.range as i32).max().unwrap();
        //println!("Min y: {:?}, max y {:?}", min, max);
        let mut count = 0;
        for x in min..=max {
            let c = (x, row);
            let covered = sensors.iter().find(|s| s.in_range(c));
            match covered {
                Some(_s) => {
                    count = count + 1;
                    //println!("pos: {:?} is covered by sensor: {:?}", c, _s);
                },
                None => ()
            };
        }
        count - (beacons.iter().filter(|b| (*b).1 == row).count() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let searchAtRow = 10;
        let actual_res = DayFifteen::from(input.to_string())
            .count_coverage(searchAtRow);
        assert_eq!(26, actual_res);
    }
}
