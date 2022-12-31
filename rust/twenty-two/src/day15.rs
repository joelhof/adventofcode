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
    type R = i64;

    fn day() -> String where Self: Sized {
        "15".to_string()
    }

    fn part_one(&self) -> Self::R {
        let sensorMap = SensorMap::from(self.input.as_str());
        let min = sensorMap.sensors.iter().map(|s| s.pos.0 - s.range as i32).min().unwrap();
        let max = sensorMap.sensors.iter().map(|s| s.pos.0 + s.range as i32).max().unwrap();
        self.count_coverage(&sensorMap, 2000000, min, max).count as i64
    }

    fn part_two(&self) -> Self::R {
        let distress_pos = self.find_distress_beacon(0, 4000000);
        DayFifteen::tuning_frequency(distress_pos) as i64
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

    fn to_row_sensor(&self, row: i32) -> Option<(i32, i32)> {
        // if row is out of range, return None

        let max_row_reached = self.pos.1 + self.range as i32;
        let min_row_reached = self.pos.1 - self.range as i32;
        //println!("max row: {}, min row: {}, target row {}", max_row_reached, min_row_reached, row);
        if row > max_row_reached || row <= min_row_reached {
            //println!("sensor: {:?} does not cover row: {}", self, row);
            return None;
        }
        //println!("sensor: {:?} covers row: {}", self, row);
        // else, calculate this sensors coverage of the row
        // row sensor start x and row sensor stop x
        let r = self.range as i32 - (row - self.pos.1).abs();
        let start_x = self.pos.0 - r;
        let end_x = self.pos.0 + r;
        Some((start_x, end_x))
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

struct SensorMap {
    sensors: Vec<Sensor>,
    beacons: HashSet<(i32, i32)>
}

impl From<&str> for SensorMap {
    fn from(input: &str) -> Self {
        let sensors: Vec<Sensor> = input.lines()
            .map(|l| l.trim())
            .map(|l| Sensor::from(l))
            .collect();
        let beacons: HashSet<(i32, i32)> = sensors.iter()
            .map(|s| s.beacon.clone())
            .collect();
        SensorMap { sensors, beacons }
    }
}

struct CoverageResult {
    count: u32,
    pos: Option<(i32, i32)>
}

impl DayFifteen {
    fn count_coverage(&self, sensorMap: &SensorMap, row: i32, min: i32, max: i32) -> CoverageResult {
        //println!("Sensors: {:?}", sensors);
        //println!("Min y: {:?}, max y {:?}", min, max);
        let mut count = 0;
        let mut pos = None;
        for x in min..=max {
            let c = (x, row);
            let covered = sensorMap.sensors.iter()
                .find(|s| s.in_range(c));
            match covered {
                Some(_s) => {
                    count = count + 1;
                    //println!("pos: {:?} is covered by sensor: {:?}", c, _s);
                },
                None => {
                    pos = Some(c.clone())
                }
            };
        }
        let coverage_count = count - (sensorMap.beacons.iter()
            .filter(|b| (*b).1 == row).count() as u32);
        return CoverageResult { count:coverage_count, pos };
    }

    fn find_distress_beacon(&self, min: i32, max: i32) -> (i32, i32) {
        // start at row min
        let sensorMap = SensorMap::from(self.input.as_str());
        let mut distress_beacon_row = None;
        let mut current_x_pos = min;
        for row in min..=max {
            if distress_beacon_row.is_some() {
                break;
            }
            let mut rowSensors: Vec<(i32, i32)> = sensorMap.sensors.iter()
                .filter_map(|sensor| sensor.to_row_sensor(row))
                .map(|s| match s {
                    (start,end) if start < min => (min,end),
                    (start,end) if end > max => (start,max),
                    (start,end) => (start,end)
                })
                .collect();
            rowSensors.sort_by(|a,b| a.0.cmp(&b.0));
            // do DFS to find if there is a path from min to max
            current_x_pos = min;
            while !rowSensors.is_empty() && current_x_pos < max {
                // find next sensor
                let nextSensor = rowSensors.iter()
                    .position(|s| current_x_pos >= s.0 && current_x_pos <= s.1);
                if nextSensor.is_none() {
                    //println!("Distress beacon is at row: {} and col: {}", row, current_x_pos);
                    distress_beacon_row = Some(row);
                    break;
                } else {
                    let rowSensor = rowSensors.swap_remove(nextSensor.unwrap());
                    current_x_pos = rowSensor.1 + 1;
                }
            }
        }
        return (current_x_pos, distress_beacon_row.unwrap());
    }

    fn tuning_frequency(pos: (i32, i32)) -> i64 {
        (pos.0 as i64).checked_mul( 4000000i64).and_then(|v| Some(v+(pos.1 as i64))).unwrap()
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
        let sensor_map = SensorMap::from(input);
        let min = sensor_map.sensors.iter().map(|s| s.pos.0 - s.range as i32).min().unwrap();
        let max = sensor_map.sensors.iter().map(|s| s.pos.0 + s.range as i32).max().unwrap();
        let actual_res = DayFifteen::from(input.to_string())
            .count_coverage(&sensor_map, searchAtRow, min, max).count;
        assert_eq!(26, actual_res);
    }

    #[test]
    fn partTwoExampleTest() {
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
        let actual_pos = DayFifteen::from(input.to_string())
            .find_distress_beacon(0, 20);
        assert_eq!((14, 11), actual_pos);
        assert_eq!(56000011, DayFifteen::tuning_frequency(actual_pos));
    }
}
