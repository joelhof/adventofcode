use std::str::FromStr;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
struct DumboOctopus {
    energylevel: u32
}

impl DumboOctopus {
    fn flash(&mut self) -> bool {
        if self.energylevel > 9 {
            self.energylevel = 0;
            return true;
        }
        return false;
    }
}


struct Grid {
    octupuses: [[DumboOctopus; 10]; 10],
    flash_count: u32
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid: [[DumboOctopus; 10]; 10] = [[DumboOctopus { energylevel: 0};10];10];
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.trim().chars().enumerate() {
                let energy = c.to_digit(10);
                if energy.is_some() {
                    grid[row][col] = DumboOctopus { energylevel: energy.unwrap() };
                } else if energy.is_none() {
                    return Err("Failed to parse octupus grid due to unknown energylevel");
                }
            }
        }
        return Ok(Grid { octupuses: grid, flash_count: 0 });
    }
}

impl Grid {

    fn neighbours(row: usize, col:usize) -> Vec<(usize, usize)> {
        let start_x = match row.checked_sub(1) {
            Some(x) => x,
            None => row
        };
        let start_y = match col.checked_sub(1) {
            Some(x) => x,
            None => col
        };
        let end_x = if row == 9 { row } else { row + 1 };
        let end_y = if col == 9 { col } else { col + 1 };
        let mut points = Vec::new();
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                if !(x == row && y == col) {
                    points.push((x,y));
                }
            }
        }
        return points;
    }

    fn flash(&mut self, neighbourhood: Vec<(usize, usize)>, flashed: &mut HashSet<(usize, usize)>) {
        for (row,col) in neighbourhood.iter() {
            if self.octupuses[*row][*col].energylevel > 9 && !flashed.contains(&(*row,*col)) {
                flashed.insert((*row, *col));
                let neighbours = Grid::neighbours(*row, *col);
                neighbours.iter().for_each(|(x,y)| self.octupuses[*x][*y].energylevel += 1 );
                self.flash(neighbours, flashed);
            }
        }
    } 

    fn next(& mut self) -> u32 {
        self.octupuses.iter_mut()
            .for_each(|row| row.iter_mut()
                .for_each(|octopus| octopus.energylevel += 1 )
            );
        
        self.flash(self.octupuses.iter().enumerate()
            .map(|(x, row)| row.iter().enumerate().map(move |(y, _o)| (x,y))).flatten().collect(),
            &mut HashSet::new()
        );    

        return self.octupuses.iter_mut()
            .map(|row| row.iter_mut()
                .map(|octopus| octopus.flash())
                .filter(|flashed| *flashed)
            )
            .flatten()
            .count() as u32;
    }
}

pub fn partOne(input: &str) -> u32 {
    let mut grid: Grid = input.parse().unwrap();
    //println!("Iteration: {}, flashed: {} grid: {:?}", 0, grid.flash_count, grid.octupuses);
    return (0..100).map(|_i| grid.next()).sum();
}

pub fn partTwo(input: &str) -> u32 {
    let mut grid: Grid = input.parse().unwrap();
    let mut count = 1;
    while grid.next() != 100 {
        count += 1;
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partOneExample() {
        let input = "5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
        ";
        let res = partOne(input);
        assert_eq!(1656, res);
    }

    #[test]
    fn partTwoExample() {
        let input = "5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
        ";
        let res = partTwo(input);
        assert_eq!(195, res);
    }
}