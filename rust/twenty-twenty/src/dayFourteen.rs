#![allow(non_snake_case)]

use crate::core::*;
use std::collections::HashMap;
use std::collections::HashSet;

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

struct Version2Decoder {
    onesMask: u64,
    wildcardMask: String
}

impl Version2Decoder {
    fn fromString(input: &str) -> Version2Decoder {
        let mask = if input.len() > 0 { parseMask(input, "0") } else { 0 };
        let wildcard = match input.split("=").nth(1) {
            Some(m) => m.trim().to_string(),
            None => "".to_string()
        };
        return Version2Decoder {
            wildcardMask: wildcard,
            onesMask: mask,
        }
    }
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

    fn partTwo(&self) -> u64 {
        let mut decoder: Version2Decoder = Version2Decoder::fromString("");
        let mut memoryArray: HashMap<u64, u64> = HashMap::new();
        let base: u64 = 2;
        for line in self.input.split("\n") {
            //println!("line {}", line);
            if line.trim().starts_with("mask") {
                decoder = Version2Decoder::fromString(line);
            } else if line.trim().starts_with("mem") {
                let instruction = parseMemoryInstruction(line);
                let mappedValue = decoder.onesMask | instruction.0;
                let mut possibleValues: HashSet<u64> = HashSet::new();
                possibleValues.insert(mappedValue);
                decoder.wildcardMask.chars().rev()
                    .enumerate()
                    .for_each(|(i, mask)| {
                        if mask == 'X' {
                            let newValues: HashSet<u64> = possibleValues.iter()
                                .map(|v| base.pow(i as u32) ^ v)
                                .collect();
                            possibleValues.extend(newValues);
                        }
                    });
                for addr in possibleValues.into_iter() {
                    memoryArray.insert(addr, instruction.1);
                }
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

    #[test]
    fn fourTeenPartTwoExampleTest() {
        const INPUT: &str = "mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1";
        let result = PartOne::init(INPUT).partTwo();
        assert_eq!(result, 208);
    }
}