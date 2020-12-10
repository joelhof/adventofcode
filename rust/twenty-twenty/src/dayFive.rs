#![allow(non_snake_case)]
extern crate itertools;

use std::collections::HashSet;
use itertools::Itertools;
use crate::core::*;

pub struct DayFive {
    input: String,
    rowRange: [u32; 128],
    columnRange: [u32; 8]
}

impl DayFive {
    pub fn new(input: &str) -> DayFive {
        let mut seatRange =  [0; 128];
        for i in 0..128 {
            seatRange[i] = i as u32;
        }
        let mut columnRange =  [0; 8];
        for i in 0..8 {
            columnRange[i] = i as u32;
        }
        return DayFive {
            input: input.into(),
            rowRange: seatRange,
            columnRange: columnRange
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

impl AdventOfCodeSolver for DayFive {
    fn day(&self) -> &str {
        return "Five";
    }

    fn partOne(&self) -> u64 {
        return match self.input.split("\n")
            .map(|line| line.trim())
            .map(|seat| self.findSeat(seat, &self.rowRange, &self.columnRange))
            .map(|seatNr| {
                let (row, column) = seatNr;
                return 8 * row + column;
            })
            .max() {
                Some(x) => x as u64,
                None => 0
            }
    }

    fn partTwo(&self) -> u64 {
        let seats: Vec<(u32, u32)> = self.input.split("\n")
            .map(|line| line.trim())
            .map(|seat| self.findSeat(seat, &self.rowRange, &self.columnRange))
            .collect();
        let takenSeatIds: HashSet<u32> = seats.iter().map(|seatNr| {
            let (row, column) = seatNr;
            return 8 * row + column;
        }).collect();
        let mut takenSeats = HashSet::new();
        for s in seats.iter() {
            takenSeats.insert((&s.0, &s.1));
        }
        let allSeats: HashSet<_> = self.rowRange[1..127].iter()
            .cartesian_product(self.columnRange.iter())
            .collect();
        let freeSeats: Vec<_> = allSeats.difference(&takenSeats).collect();
        let freeSeatIds: Option<(u32, u32, u32)> = freeSeats.iter()
            .map(|seatNr| {
                let (row, column) = seatNr;
                return 8 * *row + *column;
            })
            .map(|seatId| (seatId, seatId + 1, seatId - 1))
            .find(|seatId| takenSeatIds.contains(&seatId.1) && takenSeatIds.contains(&seatId.2));
        return match freeSeatIds {
            Some((id, _, _)) => id as u64,
            None => 0
        };
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
}