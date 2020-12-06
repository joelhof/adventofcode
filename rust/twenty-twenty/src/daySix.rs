#![allow(non_snake_case)]

use crate::core::AdventOfCodeProblem;

pub struct DaySix {
    input: String
}

impl DaySix {
    pub fn new(input: &str) -> DaySix {
        println!("{}", input);
        return DaySix {
            input: input.to_string()
        }
    }
}

impl AdventOfCodeProblem for DaySix {
    fn partOne(&self) -> u32 {
        return 0;
    }

    fn partTwo(&self) -> u32 {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        const INPUT: &str = "abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b" ;
        let result = DaySix::new(INPUT).partOne();
        assert_eq!(result, 11);
    }
}