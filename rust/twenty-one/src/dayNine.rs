use std::str::FromStr;


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
    println!("{:?}", heightMap);
    return heightMap.total_risk();
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
}