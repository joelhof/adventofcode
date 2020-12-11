#![allow(non_snake_case)]

use crate::core::*;

#[derive(Debug, PartialEq)]
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
        let mut next: Vec<Vec<Layout>> = Vec::new();
        let mut row: Vec<Layout> = Vec::new();
        row.push(Layout::Seat("#".to_string()));

        next.push(row);
        return next;
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Eleven.txt";
    }

    fn partOne(&self) -> u64 {
        self.seats.iter()
            .for_each(|seat| println!("{:?}", seat));

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
        assert_eq!(result, 37);
    }
}