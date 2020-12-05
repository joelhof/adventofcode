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

    fn findSeat(&self, seat: &str) -> (u32, u32) {
        return (1,1);
    }
}

impl AdventOfCodeProblem for DayFive {

    fn partOne(&self) -> u32 {
        return match self.input.split("\n")
            .map(|line| line.trim())
            .map(|seat| self.findSeat(seat))
            .map(|seatNr| {
                let (row, column) = seatNr;
                return 44 * row + column;
            })
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