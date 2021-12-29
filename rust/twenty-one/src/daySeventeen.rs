use regex::Regex;
use std::str::FromStr;
use std::ops::RangeInclusive;
use std::f32;

#[derive(Debug)]
struct Trajectory {
    target_area: (RangeInclusive<i32>, RangeInclusive<i32>),
    v_init: Velocity,
    y_max: i32
}

#[derive(Debug, Clone, Copy)]
struct Velocity(i32, i32);

impl Velocity {
    fn next(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        } else if self.0 < 0 {
            self.0 += 1;
        }
        self.1 -= 1;
    }

    fn yMax(&self) -> i32 {
        let Velocity(_x_0, y_0) = self;
        if y_0 > &0 {
            return y_0 * (1 + y_0) / 2;
        } else {
            return 0;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Coordinate(i32, i32);

impl Coordinate {
    fn next(&mut self, v: Velocity) {
        self.0 = self.0 + v.0;
        self.1 = self.1 + v.1;
    }
}

impl FromStr for Trajectory {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let TARGET_BOX_PATTERN: Regex = Regex::new(r"(-*\d+)").unwrap();
        let caps = TARGET_BOX_PATTERN.captures_iter(input);
        let numbers: Result<Vec<i32>, _> = caps.filter_map(|m| m.get(0))
            .map(|nr| nr.as_str())
            .map(|nr| nr.parse::<i32>())
            .collect();
        let area = match numbers {
            Ok(nrs) => (nrs[0]..=nrs[1], nrs[2]..=nrs[3]),
            _ => (0..=0, 0..=0)
        };
        return Ok(Trajectory { target_area: area, v_init: Velocity(0,0), y_max: 0 });
    }
}

impl Trajectory {

    fn simulate(&mut self) -> Option<Coordinate> {
        let mut pos = Coordinate(0,0);
        loop {
            //println!("{:?}", pos);
            if self.is_hit(&pos) {
                //println!("{:?} is a HIT ", pos);
                return Some(pos);
            }
            if self.is_miss(&pos) {
                //println!("{:?} is a MISS ", pos);
                return None;
            }
            pos.next(self.v_init);
            if pos.1 > self.y_max {
                //println!("new y_max {:?} old y_max {}", pos, self.y_max);
                self.y_max = pos.1;
            }
            self.v_init.next();
        }
    }

    fn is_hit(&self, pos: &Coordinate) -> bool {
        let Coordinate(x, y) = pos;
        return self.target_area.0.contains(x) && self.target_area.1.contains(y);
    }

    fn is_miss(&self, Coordinate(x, y): &Coordinate) -> bool {
        let x_overshot = x > self.target_area.0.end();
        let y_overshot = if *self.target_area.1.end() < 0 && *y > 0 {
            false
        } else if *self.target_area.1.end() < 0 {
            y < self.target_area.1.start()
        } else {
            y < self.target_area.1.end()
        };
        return x_overshot || y_overshot; 
    }
}

/*
The probe's x position increases by its x velocity.
The probe's y position increases by its y velocity.
Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
Due to gravity, the probe's y velocity decreases by 1.

if V_y0 > 0 {
    T_rise = V_y0
    yMax = T_rise * (1 + V_y0) / 2 = 2 * 3 / 2 = 3
    yMax = T_rise * (1 + V_y0) / 2 = 3 * (1 + 3) / 2 = 3 * 4 / 2 = 6
}

T_tot = T_rise + 1 + T_drop
T_drop * (1 + y_hit) / 2 >= y_target_min
T_drop * (1 + y_hit) / 2 <= y_target_max

T_tot * (1 + v_x0) / 2 <= x_target_max
T_tot * (1 + v_x0) / 2 >= x_target_min

T_x_max = v_x0 * (1 + v_x0) / 2 >= x_target_min
v_x0 + v_x0^2 >= 2 * x_target_min

v_x0_min^2 + v_x0_min - 40 = 0
v_x0_min = -1 +- sqrt (1 + 4 * 2 * x_target_min) / 2
v_x0_min --> T_tot = 
*/

pub fn partOne(input: &str) -> i32 {
    let mut trajectory: Trajectory = input.parse().unwrap();
    let x_0 = (((1f32 + 8f32 * *trajectory.target_area.0.start() as f32).sqrt() - 1f32) / 2f32).ceil() as i32;
    let mut y_max = 0;
    let mut v_y = 0;
    trajectory.v_init = Velocity(x_0, v_y);
    for _i in 0..200 {
        let possible = trajectory.v_init.yMax();
        let res = trajectory.simulate();
        match res {
            Some(c) if possible > y_max => {
                y_max = possible
            },
            _ => ()
        }
        v_y += 1;
        trajectory.v_init = Velocity(x_0, v_y);
    }
    return y_max;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let INPUT = "target area: x=20..30, y=-10..-5";
        let result = partOne(INPUT);
        assert_eq!(45, result);
    }

    #[test]
    fn simulateTest() {
        const INPUT: &str = "target area: x=20..30, y=-10..-5";
        let mut trajectory: Trajectory = INPUT.parse().unwrap();
        trajectory.v_init = Velocity(7,2);
        
        let res = trajectory.simulate();
        let (x, y) = match res {
            Some(Coordinate(x,y)) => (x,y),
            None => (0,0)
        };
        assert_eq!(28, x);
        assert_eq!(-7, y);

        trajectory.v_init = Velocity(6,3);
        
        let res = trajectory.simulate();
        let (x, y) = match res {
            Some(Coordinate(x,y)) => (x,y),
            None => (0,0)
        };
        assert_eq!(21, x);
        assert_eq!(-9, y);

        trajectory.v_init = Velocity(9,0);
        
        let res = trajectory.simulate();
        let (x, y) = match res {
            Some(Coordinate(x,y)) => (x,y),
            None => (0,0)
        };
        assert_eq!(-6, y);
        assert_eq!(30, x);

        trajectory.v_init = Velocity(17,-4);
        
        let res = trajectory.simulate();
        let (x, y) = match res {
            Some(Coordinate(x,y)) => (x,y),
            None => (0,0)
        };
        assert_eq!(0, y);
        assert_eq!(0, x);
    }
}