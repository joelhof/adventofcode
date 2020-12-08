#![allow(non_snake_case)]

use crate::core::*;

pub struct DayEight {
    program: Vec<Instruction>
}

struct Instruction {
    opCode: String,
    arg: u32
}

impl DayEight {
    fn test(input: &str) -> DayEight {
        return DayEight {
            program: Vec::new()
        };
    }
}

impl AdventOfCodeSolver for DayEight {
    fn day(&self) -> &str {
        return "Eight";
    }

    fn partOne(&self) -> u32 {
        return 0;
    }
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