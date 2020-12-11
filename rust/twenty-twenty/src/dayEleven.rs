#![allow(non_snake_case)]

use crate::core::*;

#[derive(Debug, PartialEq, Clone)]
enum Layout {
    Floor(String), 
    Seat(String),
}

struct Day {
    seats: Vec<Vec<Layout>>,
}

impl Day {
    fn test(input: &str) -> Day {
        return Day {
            seats: parseInput(input)
        }
    }

    fn nextGeneration(&self) -> Vec<Vec<Layout>> {
        let mut next: Vec<Vec<Layout>> = vec![vec![Layout::Floor(".".to_string()); self.seats[0].len()]; self.seats.len()];
        
        for i in 0..self.seats.len() {
            for j in 0..self.seats[0].len() {
                //println!("{:?}", self.seats[i][j]);
                match &self.seats[i][j] {
                    Layout::Floor(_) => (),
                    Layout::Seat(occupied) if occupied == "L" && self.countOccupiedAdjecent(i, j) == 0 => next[i][j] = Layout::Seat("#".to_string()),
                    Layout::Seat(occupied) if occupied == "#" => (),
                    Layout::Seat(_) => ()
                }
            }
        }

        // self.seats.iter()
        //     .enumerate()
        //     .map(|(i, row)| row.iter()
        //                         .enumerate()
        //                         .map(|(j, seat)| nextSeatState()))
        return next;
    }

    fn countOccupiedAdjecent(&self, i: usize, j: usize) -> u32 {
        return 0;
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Eleven.txt";
    }

    fn partOne(&self) -> u64 {
        let sum: u64 = self.seats[..].into_iter()
            .map(|seatRow| seatRow.iter().cloned()
                        .filter(|seat| *seat == Layout::Seat("L".to_string()))
                        .count() as u64
            ).sum();
        println!("Nr of empty seats {}", sum);
        let nextGen = self.nextGeneration();
        nextGen.iter().for_each(|seat| println!("{:?}", seat));

        return nextGen.into_iter()
            .map(|seatRow| seatRow.into_iter()
                            .filter(|seat| *seat == Layout::Seat("#".to_string()))
                            .count() as u64
                ).sum();
    }
}

fn parseInput(input: &str) -> Vec<Vec<Layout>> {
    let grid: Vec<Vec<Layout>> = input.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars()
                .map(|c| match c {
                    '.' => Layout::Floor(".".to_string()),
                    'L' => Layout::Seat("L".to_string()),
                    '#' => Layout::Seat("#".to_string()),
                    _x => panic!("Unsupported character encountered")
                })
                .collect()
            )
        .collect();
    return grid;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn elevenPartOneExampleTest() {
        const INPUT: &str = "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        let result = Day::test(INPUT).partOne();
        assert_eq!(result, 71);
    }
}