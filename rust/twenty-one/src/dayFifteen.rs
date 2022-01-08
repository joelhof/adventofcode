use std::str::FromStr;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct Grid {
    chitons: Vec<Vec<u32>>,
    rows: usize,
    cols: usize
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new(); 
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.trim().chars() {
                let energy = c.to_digit(10);
                if energy.is_some() {
                    row.push(energy.unwrap());
                } else if energy.is_none() {
                    return Err("Failed to parse chiton grid due to unknown risk level");
                }
            }
            grid.push(row);
        }
        let y = grid.len();
        let x = grid[0].len();
        return Ok(Grid { chitons: grid, rows: x, cols: y });
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Node {
    n: (usize, usize),
    risklevel: u32
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.risklevel.cmp(&self.risklevel);
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Grid {
    fn manhattan_neighbours(&self, x: &usize, y: &usize) -> Vec<(usize, usize)> {
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

    /*

    function Dijkstra(Graph, source):
2      dist[source] ← 0                           // Initialization
3
4      create vertex priority queue Q
5
6      for each vertex v in Graph:          
7          if v ≠ source
8              dist[v] ← INFINITY                 // Unknown distance from source to v
9              prev[v] ← UNDEFINED                // Predecessor of v
10
11         Q.add_with_priority(v, dist[v])
12
13
14     while Q is not empty:                      // The main loop
15         u ← Q.extract_min()                    // Remove and return best vertex
16         for each neighbor v of u:              // only v that are still in Q
17             alt ← dist[u] + length(u, v)
18             if alt < dist[v]
19                 dist[v] ← alt
20                 prev[v] ← u
21                 Q.decrease_priority(v, alt)
22
23     return dist, prev

    */

  

    fn dijkstra_lowest_risk(&self, start: (usize, usize)) -> Vec<Vec<u32>> {
        let mut priority_queue: BinaryHeap<Node> = BinaryHeap::with_capacity(self.rows * self.cols);
        let mut costs = vec![vec![u32::MAX; self.cols]; self.rows];
        let mut previous: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let (row_s, col_s) = start;
        costs[row_s][col_s] = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {
                priority_queue.push(Node { n: (row, col), risklevel: costs[row][col] });
            }
        }

        while !priority_queue.is_empty() {
            let Node { n: (row, col), .. } = priority_queue.pop().unwrap();
            for v in self.manhattan_neighbours(&row, &col) {
                let new_risk = costs[row][col] + self.chitons[v.0][v.1];
                if new_risk < costs[v.0][v.1] {
                    costs[v.0][v.1] = new_risk;
                    let prev = previous.entry(v).or_insert((0,0));
                    *prev = (row, col);
                    priority_queue.push(Node { n: v, risklevel: new_risk });
                }
            }
        }

        return costs;
    }

    

    fn expand(&mut self, rows: usize, cols: usize) {
        let mut expandedGrid: Vec<Vec<u32>> = vec![vec![0; cols]; rows];

        for row in 0..rows {
            let expandedRow = match row.checked_sub(self.rows) { None => row, Some(c) => c};
            for col in 0..cols {
                if row < self.rows && col < self.cols {
                    expandedGrid[row][col] = self.chitons[row][col];
                } else if row >= self.rows && col >= self.cols {
                    let expandedCol = match col.checked_sub(self.cols) { None => col, Some(c) => c};
                    expandedGrid[row][col] = wrap_around(expandedGrid[row][expandedCol] + 1 as u32);
                } else {
                    let expandedCol = match col.checked_sub(self.cols) { None => col, Some(c) => c};
                    expandedGrid[row][col] = wrap_around(expandedGrid[expandedRow][expandedCol] + 1 as u32);
                }
            }
        }
        self.chitons = expandedGrid;
        self.rows = rows;
        self.cols = cols;
    }
}

fn wrap_around(value: u32) -> u32 {
    if value > 9 {
        return 1;
    } else {
        return value;
    }
}

pub fn partOne(input: &str) -> u32 {
    let map: Grid = input.parse().unwrap();
    //map.chitons.iter().for_each(|r| println!("{:?}", r));
    let target = (map.rows-1, map.cols-1);
    let risks = map.dijkstra_lowest_risk((0,0));
    //risks.iter().for_each(|r| println!("{:?}", r));
    //println!("{:?}, target: {:?}", prev, target);
    return risks[target.0][target.1];
}

pub fn partTwo(input: &str) -> u32 {
    let mut map: Grid = input.parse().unwrap();
    map.expand(5 * map.rows, 5 * map.cols);
    let target = (map.rows-1, map.cols-1);
    let risks = map.dijkstra_lowest_risk((0,0));
    return risks[target.0][target.1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        const INPUT: &str = "1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581";
        let result = partOne(INPUT);
        assert_eq!(40, result);
    }

    #[test]
    fn partTwoExampleTest() {
        const INPUT: &str = "1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581";
        let result = partTwo(INPUT);
        assert_eq!(315, result);
    }
}