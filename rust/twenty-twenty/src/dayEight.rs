#![allow(non_snake_case)]

use crate::core::*;
use std::collections::HashSet;

pub struct DayEight {
    program: Vec<Instruction>
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Instruction {
    opCode: String,
    arg: i32,
    id: usize
}

impl DayEight {
    fn test(input: &str) -> DayEight {
        return DayEight {
            program: parseInput(input)
        };
    }

    pub fn new() -> DayEight {
        return DayEight {
            program: parseInput(&loadInput("Eight"))
        }
    }
}

impl AdventOfCodeSolver for DayEight {
    fn day(&self) -> &str {
        return "Eight";
    }

    fn partOne(&self) -> u32 {
        let mut executed: HashSet<&Instruction> = HashSet::new();
        let mut index: usize = 0;
        let mut instruction: &Instruction = self.program.get(index).unwrap();
        let mut acc: i32 = 0;
        while !executed.contains(&instruction) {
            index = match &instruction.opCode[..] {
                "nop" => index + 1,
                "acc" => index + 1,
                "jmp" => (index as i32 + instruction.arg) as usize,
                _ => index 
            };
            if instruction.opCode == "acc" {
                acc += instruction.arg;
            };
            executed.insert(instruction);
            instruction = match self.program.get(index) { Some(inst) => inst, None => instruction };
        }
        return acc as u32;
    }
}

impl Instruction {
    fn from(line: &str, id: usize) -> Instruction {
        let mut tmp = line.trim().splitn(2, " ");
        return Instruction {
            opCode: tmp.next().unwrap().to_string(),
            arg: tmp.next().unwrap().replace("+", "").parse().unwrap(),
            id: id
        }
    }
}

fn parseInput(input: &str) -> Vec<Instruction> {
    return input.split("\n")
        .enumerate()
        .filter(|(_i, line)| !line.trim().is_empty())
        .map(|(i, line)| Instruction::from(line, i))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        const INPUT: &str = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";
        let result = DayEight::test(INPUT).partOne();
        assert_eq!(result, 5);
    }
}