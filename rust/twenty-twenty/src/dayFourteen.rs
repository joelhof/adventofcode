#![allow(non_snake_case)]

use crate::core::*;
use std::collections::HashMap;

pub struct PartOne {
   input: String
}

impl PartOne {
    fn init(input: &str) -> PartOne {
        return PartOne {
            input: input.to_string()
        }
    }

    pub fn new() -> PartOne {
        return PartOne::init(&loadInput("Fourteen"));
    }
}

fn parseMask(mask: &str, placeholder: &str) -> u64 {
    let binary = mask.split("=").nth(1).unwrap().trim();
    return u64::from_str_radix(&binary.replace("X", placeholder), 2).unwrap()
}

fn parseMemoryInstruction(line: &str) -> (u64, u64) {
    let l = line.replace("mem", "").replace("[", "").replace("]", "");
    let mut nrs = l.split("=");
    let addr = nrs.next().unwrap();
    let value = nrs.next().unwrap();
    return (
            addr.trim().to_string().parse().unwrap(),
            value.trim().to_string().parse().unwrap()
        );
}

fn applyMask(value: &u64, zerosMask: &u64, onesMask: &u64) -> u64 {
    return zerosMask & (onesMask | value);
}
 
impl AdventOfCodeSolver for PartOne {
    fn day(&self) -> &str {
        return "Fourteen";
    }

    fn partOne(&self) -> u64 {
        let mut onesMask: u64 = 0;
        let mut zerosMask: u64 = 0;
        let mut memoryArray: HashMap<u64, u64> = HashMap::new();
        for line in self.input.split("\n") {
            if line.starts_with("mask") {
                onesMask = parseMask(line, "0");
                zerosMask = parseMask(line, "1");
            } else if line.trim().starts_with("mem") {
                let instruction = parseMemoryInstruction(line);
                memoryArray.insert(instruction.0, applyMask(&instruction.1, &zerosMask, &onesMask));
            }
        }
        return memoryArray.iter().map(|(_k, v)| v).sum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn fourTeenPartOneExampleTest() {
        const INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0";
        let result = PartOne::init(INPUT).partOne();
        assert_eq!(result, 165);
    }

    #[test]
    fn fourTeenPartOneMultipleMasksExampleTest() {
        const INPUT: &str = "mask = 110X1XX01011X100XX001X00100100X11X10
        mem[36932] = 186083
        mem[61779] = 1736
        mem[8438] = 233922
        mem[14437] = 52044
        mask = 111010XX11110X001110010XXXX10X110010
        mem[13582] = 24353
        mem[1496] = 392102652
        mem[57760] = 2161095";
        let result = PartOne::init(INPUT).partOne();
        assert_eq!(result, 377131941922);
    }
}