#![allow(non_snake_case)]

use crate::core::*;

pub struct DayEight {
    program: Vec<Instruction>
}

#[derive(Debug, Clone, PartialEq)]
struct Instruction {
    opCode: String,
    arg: i32
}

impl DayEight {
    fn test(input: &str) -> DayEight {
        return DayEight {
            program: parseInput(input)
        };
    }
}

impl AdventOfCodeSolver for DayEight {
    fn day(&self) -> &str {
        return "Eight";
    }

    fn partOne(&self) -> u32 {
        self.program.iter().for_each(|instruction| println!("{:?}", instruction));
        return 0;
    }
}

impl Instruction {
    fn from(line: &str) -> Instruction {
        let mut tmp = line.trim().splitn(2, " ");
        return Instruction {
            opCode: tmp.next().unwrap().to_string(),
            arg: tmp.next().unwrap().replace("+", "").parse().unwrap()
        }
    }
}

fn parseInput(input: &str) -> Vec<Instruction> {
    return input.split("\n")
        .map(|line| Instruction::from(line))
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