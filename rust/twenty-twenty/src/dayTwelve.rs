#![allow(non_snake_case)]

use crate::core::*;
use std::cell::Cell;

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

#[derive(Debug)]
pub struct Day {
    instructions: Vec<Instruction>,
    ship: Cell<Ship>
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ship(pub Coordinate, pub Instruction);

impl Day {
    fn test(input: &str) -> Day {
        let instructions: Vec<Instruction> = input.split("\n")
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
        println!("{:?}", instructions);
        return Day {
            instructions: instructions,
            ship: Cell::new(Ship(Coordinate(0, 0), Instruction::East(0))),
        };
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Twelve";
    }

    fn partOne(&self) -> u64 {
        println!("{:?}", self);
        for instruction in self.instructions.iter() {
            self.ship.set(self.ship.get().execute(instruction));
        }

        return self.ship.get().manhattanDistance();
    }
}

impl Ship {
    fn execute(&self, instruction: &Instruction) -> Ship {
        return Ship(Coordinate(0,0), Instruction::East(0));
    }

    fn manhattanDistance(&self) -> u64 {
        return 0;
    }
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
        let result = Day::test(INPUT).partOne();
        assert_eq!(result, 25);
    }
}