#![allow(non_snake_case)]

use crate::core::*;

#[derive(Debug, PartialEq, Clone)]
enum Layout {
    Floor(String), 
    Seat(String),
}

pub struct Day {
    seats: Vec<Vec<Layout>>,
}

impl Day {
    fn test(input: &str) -> Day {
        return Day {
            seats: parseInput(input)
        }
    }

    pub fn new() -> Day {
        return Day {
            seats: parseInput(&loadInput("Eleven"))
        }
    }

    fn nextGeneration(&self) -> (bool, Vec<Vec<Layout>>) {
        let mut next: Vec<Vec<Layout>> = vec![vec![Layout::Floor(".".to_string()); self.seats[0].len()]; self.seats.len()];
        let mut changed = false;
        for i in 0..self.seats.len() {
            for j in 0..self.seats[0].len() {
                //println!("{:?}", self.seats[i][j]);
                match &self.seats[i][j] {
                    Layout::Floor(_) => (),
                    Layout::Seat(occupied) if occupied == "L" && self.countOccupiedAdjecent(i, j) == 0 => {
                        changed = true;
                        next[i][j] = Layout::Seat("#".to_string())
                    },
                    Layout::Seat(occupied) if occupied == "#" && self.countOccupiedAdjecent(i, j) >= 4 => {
                        changed = true;
                        next[i][j] = Layout::Seat("L".to_string())
                    },
                    Layout::Seat(_) => next[i][j] = self.seats[i][j].clone()
                }
            }
        }
        return (changed, next);
    }

    fn countOccupiedAdjecent(&self, i: usize, j: usize) -> usize {
        let mut n: Vec<(Option<usize>, Option<usize>)> = Vec::new();
        for deltaI in (-1 as i64)..=1 {
            let newI;
            if deltaI > 0 {
                newI = i.checked_add(deltaI as usize);
            } else {
                newI = i.checked_sub(deltaI.abs() as usize);
            }
            for deltaJ in (-1 as i64)..=1 {
                let newJ;
                if deltaJ > 0 {
                    newJ = j.checked_add(deltaJ as usize);
                } else {
                    newJ = j.checked_sub(deltaJ.abs() as usize);
                }
                n.push((newI, newJ));
            }
        }
        let neighbours: usize = n.into_iter()
            .filter(|(i,j)| i.is_some() && j.is_some())
            .map(|(i,j)| (i.unwrap(), j.unwrap()))
            .filter(|(i1, j1)| !(i == *i1 && j == *j1)
                             && *i1 < self.seats.len()
                             && *j1 < self.seats[0].len()
            )
            .map(|(x,y)| {
                //println!("{:?}", &self.seats[x][y]);
                match &self.seats[x][y] {
                    Layout::Floor(_) => 0,
                    Layout::Seat(occupied) if occupied == "#" => 1,
                    Layout::Seat(_) => 0
                }
            }
            )
            .sum();
            //.collect();
            //println!("{}", neighbours);
        //println!("{}  new coordinates {:?}", self.seats.len(), neighbours);
        return neighbours;
    }

    pub fn day(&self) -> &str {
        return "Eleven.txt";
    }

    pub fn partOne(&mut self) -> u64 {
        let sum: u64 = self.seats[..].into_iter()
            .map(|seatRow| seatRow.iter().cloned()
                        .filter(|seat| *seat == Layout::Seat("L".to_string()))
                        .count() as u64
            ).sum();
        println!("Nr of empty seats {}", sum);
        let mut changed = true;
        //let mut nextGen;
        while changed {
            let res = self.nextGeneration();
            self.seats = res.1;
            changed = res.0;
        }
        

        
        &self.seats[..].iter().for_each(|seat| println!("{:?}", seat));

        return self.seats[..].iter().cloned()
            .map(|seatRow| seatRow.into_iter()
                            .filter(|seat| *seat == Layout::Seat("#".to_string()))
                            .count() as u64
                ).sum();
    }
}

//impl AdventOfCodeSolver for Day {
 
//}

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