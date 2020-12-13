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
    fn init(input: &str) -> DayEight {
        return DayEight {
            program: parseInput(input)
        };
    }

    pub fn new() -> DayEight {
        return DayEight::init(&loadInput("Eight"));
    }

    fn replace(&self, instruction: Instruction) -> Vec<Instruction> {
        let mut copy = self.program.clone();
        copy.remove(instruction.id);
        copy.insert(instruction.id, instruction);
        return copy;
    }
}

impl AdventOfCodeSolver for DayEight {
    fn day(&self) -> &str {
        return "Eight";
    }

    fn partOne(&self) -> u64 {
        return match execute(&self.program) {
            Ok(res) => res as u64,
            Err(res) => res as u64
        };
    }

    fn partTwo(&self) -> u64 {
        return self.program.iter()
            .enumerate()
            .filter(|(_i, inst)| inst.opCode == "nop" || inst.opCode == "jmp")
            .map(|(i, inst)| if inst.opCode == "nop" {
                Instruction {
                    id: i,
                    opCode: "jmp".to_string(),
                    arg: inst.arg
                }
                } else {
                    Instruction {
                        id: i,
                        opCode: "nop".to_string(),
                        arg: inst.arg
                    }
                }
            )
            .map(|instr| execute(&self.replace(instr)))
            .find_map(|res| res.ok())
            .unwrap() as u64;
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

fn execute(program: &Vec<Instruction>) -> Result<i32, i32> {
    let mut executed: HashSet<&Instruction> = HashSet::new();
    let mut index: usize = 0;
    let mut instruction: &Instruction = program.get(index).unwrap();
    let mut acc: i32 = 0;
    while !executed.contains(&instruction) {
        if instruction.opCode == "acc" {
            acc += instruction.arg;
        };
        index = match &instruction.opCode[..] {
            "nop" => index + 1,
            "acc" => index + 1,
            "jmp" => (index as i32 + instruction.arg) as usize,
            _ => index 
        };
        executed.insert(instruction);
        if index >= program.len() {
            //println!("Exit program successfully");
            return Ok(acc);
        }
        instruction = match program.get(index) { Some(inst) => inst, None => instruction };
    }
    return Err(acc);
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
        let result = DayEight::init(INPUT).partOne();
        assert_eq!(result, 5);
    }

    #[test]
    fn partTwoExampleTest() {
        const INPUT: &str = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";
        let result = DayEight::init(INPUT).partTwo();
        assert_eq!(result, 8);
    }
}