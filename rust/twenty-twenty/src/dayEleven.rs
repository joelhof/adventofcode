#![allow(non_snake_case)]

use crate::core::*;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
enum Layout {
    Floor(String), 
    Seat(String),
}

pub struct Day {
    seats: Vec<Vec<Layout>>,
    input: Vec<Vec<Layout>>
}

impl Day {
    fn test(input: &str) -> Day {
        return Day {
            seats: parseInput(input),
            input: parseInput(input)
        }
    }

    pub fn new() -> Day {
        return Day {
            seats: parseInput(&loadInput("Eleven")),
            input: parseInput(&loadInput("Eleven"))
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
        // let sum: u64 = self.seats[..].into_iter()
        //     .map(|seatRow| seatRow.iter().cloned()
        //                 .filter(|seat| *seat == Layout::Seat("L".to_string()))
        //                 .count() as u64
        //     ).sum();
        //println!("Nr of empty seats {}", sum);
        let mut changed = true;
        while changed {
            let res = self.nextGeneration();
            self.seats = res.1;
            changed = res.0;
        }
        
        //&self.seats[..].iter().for_each(|seat| println!("{:?}", seat));

        return self.seats[..].iter().cloned()
            .map(|seatRow| seatRow.into_iter()
                            .filter(|seat| *seat == Layout::Seat("#".to_string()))
                            .count() as u64
                ).sum();
    }

    pub fn partTwo(&self) -> u64 {
        let init: Vec<Vec<State>> = self.input.iter()
            .map(move |seats| seats.into_iter()
                .map(move |layout| match layout {
                    Layout::Floor(_) => State::Floor,
                    Layout::Seat(occupied) if occupied == "#" => State::Occupied,
                    Layout::Seat(_) => State::Unoccupied
                }).collect()
            ).collect();
        let mut conwayGrid = PartTwo {
            grid: init
        };
        let mut changed = true;
        let mut iteration = 0;
        // println!("{}", conwayGrid);
        // println!("--------------------------------------");
        while changed {
            let (res, grid) = conwayGrid.nextGeneration();
            conwayGrid.grid = grid;
            changed = res;
            iteration = iteration + 1;
            //println!("iteration {} ", iteration);
            //println!("{}", conwayGrid);
            //println!("--------------------------------------");
        }

        return conwayGrid.getGrid().iter()
                .map(|seatRow| seatRow.into_iter()
                    .filter(|seat| **seat == State::Occupied)
                    .count() as u64
                ).sum();
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Floor = 0,
    Unoccupied = 1,
    Occupied = 2
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            State::Floor => ".",
            State::Occupied => "#",
            State::Unoccupied => "L"
        })
    }
}

struct PartTwo {
    grid: Vec<Vec<State>>
}

impl fmt::Display for PartTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.getGrid() {
            for state in row {
                write!(f, "{}", state);
            }
            writeln!(f, "");
        };
        return write!(f, "");
    }
}


impl PartTwo {
    fn getGrid(&self) -> &Vec<Vec<State>> {
        return &self.grid;
    }

    fn nextGeneration(&self) -> (bool, Vec<Vec<State>>) {
        let mut next: Vec<Vec<State>> = vec![vec![State::Floor; self.grid[0].len()]; self.grid.len()];
        let mut changed = false;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                //println!("{},{}: {:?}", i, j, self.grid[i][j]);
                match &self.grid[i][j] {
                    State::Floor => (),
                    State::Unoccupied if self.countVisible(&[i,j]) == 0 => {
                        changed = true;
                        next[i][j] = State::Occupied;
                    },
                    State::Occupied if self.countVisible(&[i,j]) >= 5 => {
                        changed = true;
                        next[i][j] = State::Unoccupied;
                    },
                    state => next[i][j] = *state
                }
            }
        }
        return (changed, next);
    }

    fn getNeighbours(&self, coordinate: &[usize; 2]) -> Vec<[usize; 2]> {
        let mut n: Vec<[usize; 2]> = Vec::new();
        let [x, y] = coordinate;
        for deltaX in (-1 as i8)..=1 {
            let newX;
            if deltaX > 0 {
                newX = x.checked_add(deltaX as usize);
            } else {
                newX = x.checked_sub(deltaX.abs() as usize);
            }
            for deltaY in (-1 as i64)..=1 {
                let newY;
                if deltaY > 0 {
                    newY = y.checked_add(deltaY as usize);
                } else {
                    newY = y.checked_sub(deltaY.abs() as usize);
                }
                if newX.is_some() && newY.is_some() {
                    n.push([newX.unwrap(), newY.unwrap()]);
                }
            }
        }

        return n.into_iter()
            .filter(|[i1, j1]| !(x == i1 && y == j1)
                             && *i1 < self.grid.len()
                             && *j1 < self.grid[0].len()
            ).collect();
    }

    fn countVisible(&self, coordinate: &[usize; 2]) -> usize {
        let [x1, y1] = coordinate;
        //println!("counting visible {:?}", coordinate);
        let directions: Vec<[isize; 2]> = self.getNeighbours(coordinate).into_iter()
                .map(|[x2, y2]| [
                    x2 as isize - *x1 as isize,
                    y2 as isize - *y1 as isize
                    ]
                )
                .collect();
        //let nr = directions.len();
        //if nr == 3 {
        //println!("neighbours {:?}", self.getNeighbours(coordinate));
        //println!("directions {:?}", directions);
        //}
        // let mut seats = self.getGrid().into_iter()
        //     .enumerate()
        //     .flat_map(|(row, seatRow)| seatRow.into_iter()
        //         .enumerate()
        //         .filter(|(_col, state)| **state == State::Occupied || **state == State::Unoccupied)
        //         .map(move |(col, _state)| [row, col])
        //     )
        //     .collect::<Vec<[usize; 2]>>();
        
        // find first seat in each direction.
        // The set of first seat in each direction can be pre-computed and stored in a map
        let res = directions.into_iter()
                .filter(|dir| {
                    let first = self.findFirst(dir, coordinate);
                    //println!("First hit {:?}", first);
                    return match first {
                        None => false,
                        Some([x,y]) => self.grid[x][y] == State::Occupied
                    };
                }
                )
                .count();
        // if nr == 3 {
        //     println!("nr of visible {}", res);
        // }
        return res;
    }

    fn findFirst(&self, direction: &[isize; 2], origin: &[usize; 2]) -> Option<[usize; 2]> {
        // loop until either a seat is found, or an edge is reached.
        let mut candidate: Vec<usize> = origin.iter().cloned().collect();
        loop {
            candidate = candidate.iter().zip(direction).map(|(a, b)| (*a as isize + *b) as usize).collect();
            match self.getGrid().get(candidate[0]).and_then(|row| row.get(candidate[1])) {
                Some(State::Floor) => continue,
                Some(State::Unoccupied) => return Some([candidate[0], candidate[1]]),
                Some(State::Occupied) => return Some([candidate[0], candidate[1]]),
                None => return None
            }
        };
    }
}

fn onLine(direction: &[isize; 2], point: &[usize; 2], origin: &[usize; 2]) -> bool {
    //println!("is point {:?} on line {:?} with origin {:?}?", point, direction, origin);
    let [deltaX, deltaY] = direction;
    let [x, y] = *point;
    let [x0, y0] = *origin;
    
    let crossProduct = deltaX * (y as isize - y0 as isize) - deltaY * (x as isize - x0 as isize);
    //println!("p: {:?} is on line {:?}, {}", point, direction, crossProduct);
    if crossProduct == 0 {
        let sameX = deltaX.signum() == (x as isize - x0 as isize).signum();
        let sameY = deltaY.signum() == (y as isize - y0 as isize).signum();
        return sameX && sameY;
    }
    return false;
}

fn cartesianDistance(p1: &[usize; 2], p2: &[usize; 2]) -> isize {
    //println!("distance between {:?} and {:?}", p1, p2);
    return p1.into_iter()
        .zip(p2.into_iter())
        .map(|(p, q)| (*p as isize - *q as isize) * (*p as isize - *q as isize))
        .sum();
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

    #[test]
    fn elevenPartTwoExampleTest() {
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
        let result = Day::test(INPUT).partTwo();
        assert_eq!(result, 26);
    }
    
    #[test]
    fn elevenCountVisible1ExampleTest() {
        const INPUT: &str = ".......#.
        ...#.....
        .#.......
        .........
        ..#L....#
        ....#....
        .........
        #........
        ...#.....";
        let init: Vec<Vec<State>> = Day::test(INPUT).seats.iter()
            .map(move |seats| seats.into_iter()
                .map(move |layout| match layout {
                    Layout::Floor(_) => State::Floor,
                    Layout::Seat(occupied) if occupied == "#" => State::Occupied,
                    Layout::Seat(_) => State::Unoccupied
                }).collect()
            ).collect();
        let conwayGrid = PartTwo {
            grid: init
        };
        let result = conwayGrid.countVisible(&[4, 3]);
        assert_eq!(result, 8);
    }

    #[test]
    fn elevenCountVisible2ExampleTest() {
        const INPUT: &str = ".##.##.
        #.#.#.#
        ##...##
        ...L...
        ##...##
        #.#.#.#
        .##.##.";
        let init: Vec<Vec<State>> = Day::test(INPUT).seats.iter()
            .map(move |seats| seats.into_iter()
                .map(move |layout| match layout {
                    Layout::Floor(_) => State::Floor,
                    Layout::Seat(occupied) if occupied == "#" => State::Occupied,
                    Layout::Seat(_) => State::Unoccupied
                }).collect()
            ).collect();
        let conwayGrid = PartTwo {
            grid: init
        };
        let result = conwayGrid.countVisible(&[3, 3]);
        assert_eq!(result, 0);
    }

    #[test]
    fn elevenCountVisible3ExampleTest() {
        const INPUT: &str = "#.##.##.##
        #######.##
        #.#.#..#..
        ####.##.##
        #.##.##.##
        #.#####.##
        ..#.#.....
        ##########
        #.######.#
        #.#####.##";
        let init: Vec<Vec<State>> = Day::test(INPUT).seats.iter()
            .map(move |seats| seats.into_iter()
                .map(move |layout| match layout {
                    Layout::Floor(_) => State::Floor,
                    Layout::Seat(occupied) if occupied == "#" => State::Occupied,
                    Layout::Seat(_) => State::Unoccupied
                }).collect()
            ).collect();
        let conwayGrid = PartTwo {
            grid: init
        };
        let result = conwayGrid.countVisible(&[1, 0]);
        assert_eq!(result, 4);
    }

    #[test]
    fn elevenCountVisible4ExampleTest() {
        const INPUT: &str = "#.LL.LL.L#
        #LLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLL#
        #.LLLLLL.L
        #.LLLLL.L#";
        let init: Vec<Vec<State>> = Day::test(INPUT).seats.iter()
            .map(move |seats| seats.into_iter()
                .map(move |layout| match layout {
                    Layout::Floor(_) => State::Floor,
                    Layout::Seat(occupied) if occupied == "#" => State::Occupied,
                    Layout::Seat(_) => State::Unoccupied
                }).collect()
            ).collect();
        let conwayGrid = PartTwo {
            grid: init
        };
        let result = conwayGrid.countVisible(&[0, 3]);
        assert_eq!(result, 0);
    }
    
    #[test]
    fn elevenCountVisible5ExampleTest() {
        const INPUT: &str = "#.LL.LL.L#
        #LLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLL#
        #.LLLLLL.L
        #.LLLLL.L#";
        let init: Vec<Vec<State>> = Day::test(INPUT).seats.iter()
            .map(move |seats| seats.into_iter()
                .map(move |layout| match layout {
                    Layout::Floor(_) => State::Floor,
                    Layout::Seat(occupied) if occupied == "#" => State::Occupied,
                    Layout::Seat(_) => State::Unoccupied
                }).collect()
            ).collect();
        let conwayGrid = PartTwo {
            grid: init
        };
        let result = conwayGrid.countVisible(&[0, 2]);
        assert_eq!(result, 1);
    }

    #[test]
    fn elevenCountVisible6ExampleTest() {
        const INPUT: &str = ".##.##.
        #.#.#.#
        ##...##
        #L.L.L#
        ##...##
        #.#.#.#
        .##.##.";
        let init: Vec<Vec<State>> = Day::test(INPUT).seats.iter()
            .map(move |seats| seats.into_iter()
                .map(move |layout| match layout {
                    Layout::Floor(_) => State::Floor,
                    Layout::Seat(occupied) if occupied == "#" => State::Occupied,
                    Layout::Seat(_) => State::Unoccupied
                }).collect()
            ).collect();
        let conwayGrid = PartTwo {
            grid: init
        };
        let result = conwayGrid.countVisible(&[3, 3]);
        assert_eq!(result, 0);
    }
}