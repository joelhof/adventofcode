use std::str::FromStr;
use std::collections::HashMap;

struct VentMap {
    vents:  HashMap<Coordinate, u32>
}

#[derive(Debug)]
#[derive(Eq, Hash, Copy, Clone)]
struct Coordinate(u32,u32);

impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        return self.0 == other.0 && self.1 == other.1;
    }
}

impl Coordinate {
    fn pointsOnLine(&self, other: &Coordinate, includeDiagonal: bool) -> Vec<Coordinate> {
        let mut points: Vec<Coordinate> = Vec::new();
        let diffX: i64 = (other.0 as i64) - (self.0 as i64); 
        let diffY: i64 = (other.1 as i64) - (self.1 as i64);
        println!("{} {}", diffX, diffY);
        if self.0 == other.0 {
            println!("vertical line {:?} -> {:?}", self, other);
            let start = if self.1 >= other.1 { other.1 } else { self.1 };
            let end = if self.1 >= other.1 { self.1 } else { other.1 } + 1;
            for y in start..end {
                points.push(Coordinate(self.0, y));
            }
        } else if self.1 == other.1 {
            println!("horizontal line {:?} -> {:?}", self, other);
            let start = if self.0 >= other.0 { other.0 } else { self.0 };
            let end = if self.0 >= other.0 { self.0 } else { other.0 } + 1;
            for x in start..end {
                points.push(Coordinate(x, self.1));
            }
        } else if includeDiagonal && diffX.abs() == diffY.abs() {
            println!("diagonal line {:?} -> {:?}", self, other);
            for step in 0..diffX.abs() + 1 {
                let x = self.0 as i64 + step * if diffX >= 0 { 1 } else { -1 };
                let y = self.1 as i64 + step * if diffY >= 0 { 1 } else { -1 };
                println!("x = {}, y = {}", x, y);
                points.push(Coordinate(x as u32, y as u32));
                
            }

        }

        return points;
    }
}

impl FromStr for Coordinate {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let coords: Result<Vec<u32>, std::num::ParseIntError> = input.split(",")
            .map(|p| p.trim())
            .map(|p| p.parse())
            .collect();
        return match coords {
            Ok(points) if points.len() == 2 => Ok(Coordinate(points[0], points[1])),
            Ok(_) => Err("Not 2 points"),
            Err(_msg) => Err("Failed to parse str to Coordinate")
        };
        
    }
}

impl FromStr for VentMap {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let coords: HashMap<Coordinate, u32> = input.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.split("->")
                .map(|sub_str| sub_str.trim())
                .filter(|sub_str| !sub_str.is_empty())
                .map(|p| p.parse())
                .collect::<Result<Vec<Coordinate>, &str>>()
        )
        .map(|points| match points {
            Ok(p) => p[0].pointsOnLine(&p[1], false),
            Err(_) => vec![]
        })
        .flatten()
        .fold(HashMap::new(), |mut points, point| {
            let counter = points.entry(point).or_insert(0);
            *counter += 1;
            return points
        });
        //println!("{:?}", points);
        return Ok(VentMap { vents: coords });
    }
}

pub fn partOne(input: &str) -> u32 {
    let vents: VentMap = input.parse().unwrap();
    println!("{}", vents.vents.keys().count());
    return vents.vents.values().filter(|nr_of_vents| **nr_of_vents >= 2).count() as u32;
}

pub fn partTwo(input: &str) -> u32 {
    let vents: HashMap<Coordinate, u32> = input.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.split("->")
                .map(|sub_str| sub_str.trim())
                .filter(|sub_str| !sub_str.is_empty())
                .map(|p| p.parse())
                .collect::<Result<Vec<Coordinate>, &str>>()
        )
        .map(|points| match points {
            Ok(p) => p[0].pointsOnLine(&p[1], true),
            Err(_) => vec![]
        })
        .flatten()
        .fold(HashMap::new(), |mut points, point| {
            let counter = points.entry(point).or_insert(0);
            *counter += 1;
            return points
        });
    println!("{}", vents.keys().count());
    return vents.values().filter(|nr_of_vents| **nr_of_vents >= 2).count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExample() {
        let input = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        
        ";
        let res = partOne(input);
        assert_eq!(5, res);
    }

    #[test]
    fn notHorizontalOrVerticalpointsOnLineTest() {
        let points = vec![
            vec![Coordinate(0,8), Coordinate(8,0)],
            vec![Coordinate(6,4), Coordinate(2,0)],
            vec![Coordinate(0,0), Coordinate(8,8)]
        ];
        points.iter().for_each(|p| {
            let points = p[0].pointsOnLine(&p[1], false);
            assert_eq!(0, points.len());    
        });
    }

    #[test]
    fn pointsOnLineTest() {
        let points = vec![
            vec![Coordinate(3,4), Coordinate(1,4)],
            vec![Coordinate(1,4), Coordinate(3,4)]
        ];
        points.iter().for_each(|p| {
            let points = p[0].pointsOnLine(&p[1], false);
            assert_eq!(3, points.len()); 
            assert_eq!(Coordinate(1,4), points[0]);
            assert_eq!(Coordinate(2,4), points[1]);
            assert_eq!(Coordinate(3,4), points[2]);   
        });
    }

    #[test]
    fn partTwoExample() {
        let input = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        
        ";
        let res = partTwo(input);
        assert_eq!(12, res);
    }

    #[test]
    fn diagonalPointsOnLineTest() {
        let points = vec![Coordinate(1,1), Coordinate(3,3)];
        let linePoints = points[0].pointsOnLine(&points[1], true);
            assert_eq!(3, linePoints.len()); 
            assert_eq!(Coordinate(1,1), linePoints[0]);
            assert_eq!(Coordinate(2,2), linePoints[1]);
            assert_eq!(Coordinate(3,3), linePoints[2]);   

        let points = vec![Coordinate(9,7), Coordinate(7,9)];
        let linePoints = points[0].pointsOnLine(&points[1], true);
            assert_eq!(3, linePoints.len()); 
            assert_eq!(Coordinate(9,7), linePoints[0]);
            assert_eq!(Coordinate(8,8), linePoints[1]);
            assert_eq!(Coordinate(7,9), linePoints[2]);
    }
}