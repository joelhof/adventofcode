#![allow(non_snake_case)]

use crate::core::*;

pub struct PartOne {
    onesMask: u64,
    zerosMask: u64,
    memoryArray: Vec<u64> 
}

impl PartOne {
    fn init(input: &str) -> PartOne {
        return PartOne {
            onesMask: 0,
            zerosMask: 0,
            memoryArray: Vec::new()
        }
    }
}

impl AdventOfCodeSolver for PartOne {
    fn day(&self) -> &str {
        return "Fourteen";
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
}