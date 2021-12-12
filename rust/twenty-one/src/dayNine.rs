use std::str::FromStr;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug)]
struct HeightMap {
    map: Vec<Vec<u32>>,
    rows: usize,
    cols: usize
}

impl HeightMap {
    fn orthogonal_neighbours(&self, x: &usize, y: &usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        if *x >= 1 {
            neighbours.push((x - 1, *y));
        }
        if x + 1 < self.rows {
            neighbours.push((x + 1, *y));
        }
        if *y >= 1 {
            neighbours.push((*x, y - 1));
        }
        if y + 1 < self.cols {
            neighbours.push((*x, y + 1));
        } 
        
        return neighbours;
    }

    fn low_points(&self) -> Vec<(usize, usize)> {
        let mut low_points = Vec::new();
        for x in 0..self.map.len() {
            for y in 0..self.map[0].len() {
                let smallestNeighbour = match self.orthogonal_neighbours(&x, &y).iter()
                    .map(|(row, col)| self.map[*row][*col])
                    .min() {
                        Some(risk) => risk,
                        None => u32::MAX
                    };
                if self.map[x][y] < smallestNeighbour {
                    low_points.push((x,y));
                }
            }
        }
        return low_points;
    }

    fn total_risk(&self) -> u32 {
        return self.low_points().iter()
            .map(|(x,y)| self.map[*x][*y] + 1)
            .sum();
    }

    fn adjacent(&self, x: &usize, y: &usize) -> Vec<(usize, usize)> {
        return self.orthogonal_neighbours(x, y)
            .into_iter()
            .filter(|(row, col)| self.map[*row][*col] < 9)
            .collect();
    }

    fn basin(&self, x: &usize, y: &usize) -> Vec<(usize, usize)> {

        // breadth first search to find all positions connected to (x,y)
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new(); 
        queue.push_back((*x, *y));
        visited.insert((*x, *y));
        while let Some(v) = queue.pop_front() {
            for n in self.adjacent(&v.0, &v.1) {
                if !visited.contains(&n) {
                    visited.insert(n);
                    queue.push_back(n);
                }
            }
        };

        return Vec::from_iter(visited);
    }

    fn basins(&self) -> Vec<Vec<(usize, usize)>> {
        return self.low_points().iter()
            .map(|(x,y)| self.basin(x,y))
            .collect();
    }
}

impl FromStr for HeightMap {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let heights: Vec<Vec<u32>> = input.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.chars()
                .map(|c| match c.to_digit(10) { Some(x) => x, None => 999})
                .collect()
            ).collect();
        return Ok(HeightMap { map: heights, rows: input.lines().count(), cols: input.lines().next().unwrap().chars().count() });
    }
}
pub fn partOne(input: &str) -> u32 {
    let heightMap: HeightMap = input.parse().unwrap();
    return heightMap.total_risk();
}

pub fn partTwo(input: &str) -> u32 {
    let heightMap: HeightMap = input.parse().unwrap();
    let mut basins = heightMap.basins();
        basins.sort_by(|a,b| b.len().cmp(&a.len()));
    return basins.iter().take(3).map(|b| b.len() as u32).product();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExample() {
        let input = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678";
        let res = partOne(input);
        assert_eq!(15, res);
    }

    #[test]
    fn partTwoExample() {
        let input = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678";
        let res = partTwo(input);
        assert_eq!(1134, res);
    }
}