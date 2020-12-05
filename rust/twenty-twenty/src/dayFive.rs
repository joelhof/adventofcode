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

    fn findSeat(&self, seat: &str, seatRange: &[u32], columnRange: &[u32]) -> (u32, u32) {
        let half = if seatRange.len() > 1 { seatRange.len() / 2 } else { columnRange.len() / 2 };
        if seat.is_empty() {
            return (seatRange[0], columnRange[0]);
        }
        let pos = &seat[0..1];
        return match pos {
            "F" => self.findSeat(&seat[1..], &seatRange[0..half], columnRange),
            "B" => self.findSeat(&seat[1..], &seatRange[half..], columnRange),
            "L" => self.findSeat(&seat[1..], &seatRange, &columnRange[0..half]),
            "R" => self.findSeat(&seat[1..], &seatRange, &columnRange[half..]),
            _ => (0,0)
        };
    }
}

impl AdventOfCodeProblem for DayFive {

    fn partOne(&self) -> u32 {
        let mut seatRange =  [0; 128];
        for i in 0..128 {
            seatRange[i] = i as u32;
        }
        let mut columnRange =  [0; 8];
        for i in 0..8 {
            columnRange[i] = i as u32;
        }
        return match self.input.split("\n")
            .map(|line| line.trim())
            .map(|seat| self.findSeat(seat, &seatRange, &columnRange))
            .map(|seatNr| {
                let (row, column) = seatNr;
                return 8 * row + column;
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
        const INPUT: &str = "FBFBBFFRLR
        BFFFBBFRRR
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