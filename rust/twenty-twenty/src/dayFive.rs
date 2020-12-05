#![allow(non_snake_case)]

trait AdventOfCodeProblem {
    fn partOne(&self) -> u32;
    fn partTwo(&self) -> u32;
    
}

struct DayFive {
    input: String
}

impl DayFive {
    pub fn new(input: &str) -> DayFive {
        return DayFive {
            input: input.into(),
        }
    }
}

impl AdventOfCodeProblem for DayFive {

    fn partOne(&self) -> u32 {
        return match self.input.split("\n")
            .map(|line| line.trim())
            .map(|seat| seat.chars().count())
            .max() {
                Some(x) => x as u32,
                None => 0
            }
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
        const INPUT: &str = "BFFFBBFRRR
        FFFBBBFRRR
        BBFFBBFRLL" ;
        let result = DayFive::new(INPUT).partOne();
        assert_eq!(result, 820);
    }

    #[test]
    fn partTwoexampleTest() {
        const INPUT: &str = "BFFFBBFRRR
        FFFBBBFRRR
        BBFFBBFRLL" ;
        let result = DayFive::new(INPUT).partTwo();
        assert_eq!(result, 0);
    }
}