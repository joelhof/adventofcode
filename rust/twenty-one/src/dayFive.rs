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
    fn pointsOnLine(&self, other: &Coordinate) -> Vec<Coordinate> {
        let mut points: Vec<Coordinate> = Vec::new();
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
            Ok(p) => p[0].pointsOnLine(&p[1]),
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
            let points = p[0].pointsOnLine(&p[1]);
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
            let points = p[0].pointsOnLine(&p[1]);
            assert_eq!(3, points.len()); 
            assert_eq!(Coordinate(1,4), points[0]);
            assert_eq!(Coordinate(2,4), points[1]);
            assert_eq!(Coordinate(3,4), points[2]);   
        });
    }
}