#![allow(non_snake_case)]

use crate::core::*;
use std::cell::Cell;
use std::convert::TryInto;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize)
}

impl Instruction {
    fn getHeading(&self) -> isize {
        return match self {
            Instruction::North(_) => 0,
            Instruction::East(_) => 90,
            Instruction::South(_) => 180,
            Instruction::West(_) => 270,
            _ => 0
        };
    }
}

#[derive(Debug)]
pub struct Day {
    instructions: Vec<Instruction>,
    ship: Cell<Ship>
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ship(pub Coordinate, pub Instruction);

impl Day {

    pub fn new() -> Day {
        return Day {
            instructions: parseInput(&loadInput("Twelve")),
            ship: Cell::new(Ship(Coordinate(0, 0), Instruction::East(0))),
        };
    }
}

impl FromStr for Day {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        return Ok(Day {
            instructions: parseInput(input),
            ship: Cell::new(Ship(Coordinate(0, 0), Instruction::East(0)))
        });
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Twelve";
    }

    fn partOne(&self) -> u64 {
        for instruction in self.instructions.iter() {
            self.ship.set(self.ship.get().execute(instruction));
        }
        return self.ship.get().manhattanDistance();
    }

    fn partTwo(&self) -> u64 {
        // Set ship facing to follow waypoint, i.e update ship facing when way point is moved.
        // way point coordinates are relative to the ship.

        // loop over instructions, if F instruction update ship heading, then move ship.
        // if other instruction, move waypoint
        let waypointShip = Waypoint {
            ship: Coordinate(0,0),
            waypoint: Coordinate(10,1)
        };
        let result = self.instructions.iter().fold(waypointShip, |mut acc, instruction| {
            //println!("{:?}, next instruction {:?}", acc, instruction);
            acc.execute(instruction);
            return acc;
        });
        //println!("{:?}", result);
        return result.ship.manhattan(Coordinate(0,0)) as u64;
    }
}

impl Ship {
    fn execute(&self, instruction: &Instruction) -> Ship {
        return match instruction {
            Instruction::Forward(x) => self.forward(*x),
            Instruction::Left(x) => self.turn(0isize.checked_sub((*x).try_into().unwrap()).unwrap()),
            Instruction::Right(x) => self.turn((*x).try_into().unwrap()),
            heading => self.heading(*heading)
        }
    }

    fn heading(&self, heading: Instruction) -> Ship {
        let Coordinate(x,y) = self.0;
        let newPosition = match heading {
            Instruction::South(d) => Coordinate(x - (d as isize), y),
            Instruction::North(d) => Coordinate(x + (d as isize), y),
            Instruction::East(d) => Coordinate(x, y + (d as isize)),
            Instruction::West(d) => Coordinate(x, y - (d as isize)),
            h => panic!("Unknown heading {:?}", h),
        };
        return Ship(newPosition, self.1);
    }

    fn turn(&self, degrees: isize) -> Ship {
        let mut h = self.1.getHeading() + degrees;
        if h >= 360 {
            h = h - 360;
        } else if h < 0 {
            h += 360;
        }
        let newHeading = match h {
            0 => Instruction::North(0),
            90 => Instruction::East(0),
            180 => Instruction::South(0),
            270 => Instruction::West(0),
            _ => panic!("Heading is wrong! {}", h)
        };
        return Ship(self.0, newHeading);
    }

    fn forward(&self, forward: usize) -> Ship {
        let Coordinate(x,y) = self.0;
        let newPosition = match self.1 {
            Instruction::South(_) => Coordinate(x - (forward as isize), y),
            Instruction::North(_) => Coordinate(x + (forward as isize), y),
            Instruction::East(_) => Coordinate(x, y + (forward as isize)),
            Instruction::West(_) => Coordinate(x, y - (forward as isize)),
            h => panic!("Unknown heading {:?}", h),
        };
        return Ship(newPosition, self.1);
    }

    fn manhattanDistance(&self) -> u64 {
        return self.0.manhattan(Coordinate(0,0)) as u64;
    }
}

#[derive(Debug)]
struct Waypoint {
    ship: Coordinate,
    waypoint: Coordinate
}

impl Waypoint {
    fn execute(&mut self, instruction: &Instruction) {
        return match instruction {
            Instruction::Forward(x) => self.forward(*x),
            Instruction::Left(x) => self.rotation((*x).try_into().unwrap()),
            Instruction::Right(x) => self.rotation(0isize.checked_sub((*x).try_into().unwrap()).unwrap()),
            heading => self.translation(*heading)
        }
    }

    fn translation(&mut self, heading: Instruction) {
        let Coordinate(x,y) = self.waypoint;
        self.waypoint = match heading {
            Instruction::West(d) => Coordinate(x - (d as isize), y),
            Instruction::East(d) => Coordinate(x + (d as isize), y),
            Instruction::North(d) => Coordinate(x, y + (d as isize)),
            Instruction::South(d) => Coordinate(x, y - (d as isize)),
            h => panic!("Unknown heading {:?}", h),
        };
    }

    fn rotation(&mut self, rotation: isize) {
        let Coordinate(x,y) = self.waypoint;
        self.waypoint = match rotation {
            r if r == 270 || r == -90 => Coordinate(y, -x),
            r if r == 90 || r == -270 => Coordinate(-y, x),
            r if r == 180 || r == -180 => Coordinate(-x, -y),
            r if r.abs() == 360 => Coordinate(x,y),
            r => panic!("Unsupported rotation {}", r)
        };
    }

    fn forward(&mut self, forward: usize) {
        let Coordinate(x,y) = self.waypoint;
        let Coordinate(x1, y1) = self.ship;
        self.ship = Coordinate(x1 + (forward as isize * x), y1 + (forward as isize * y));
    }
}

fn parseInput(input: &str) -> Vec<Instruction> {
    return input.split("\n")
    .map(|line| line.trim())
    .filter(|line| !line.is_empty())
    .filter_map(|line| match line.chars().nth(0) {
        Some('S') => Some(Instruction::South(line[1..].parse::<usize>().unwrap())),
        Some('N') => Some(Instruction::North(line[1..].parse::<usize>().unwrap())),
        Some('E') => Some(Instruction::East(line[1..].parse::<usize>().unwrap())),
        Some('W') => Some(Instruction::West(line[1..].parse::<usize>().unwrap())),
        Some('F') => Some(Instruction::Forward(line[1..].parse::<usize>().unwrap())),
        Some('L') => Some(Instruction::Left(line[1..].parse::<usize>().unwrap())),
        Some('R') => Some(Instruction::Right(line[1..].parse::<usize>().unwrap())),
        Some(_i) => panic!("Unexpected instruction: {}", line),
        None => panic!("Unexpected instruction: {}", line)
    })
    .collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn twelvePartOneExampleTest() {
        const INPUT: &str = "F10
            N3
            F7
            R90
            F11";
        let result = INPUT.parse::<Day>().unwrap().partOne();
        assert_eq!(result, 25);
    }

    #[test]
    fn twelvePartTwoExampleTest() {
        const INPUT: &str = "F10
            N3
            F7
            R90
            F11";
        let result = INPUT.parse::<Day>().unwrap().partTwo();
        assert_eq!(result, 286);
    }
}