use regex::Regex;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug)]
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coordinate(i32,i32);

impl FromStr for Coordinate {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let coords: Result<Vec<i32>, _> = input.split(",").map(|c| c.parse()).collect();
        return match coords {
            Ok(coords) if coords.len() == 2 => Ok(Coordinate(coords[0], coords[1])),
            Ok(_c) => Err("Only supports 2D coordinates"),
            Err(_) => Err("Unable to parse coordinates")
        }
    }
}

#[derive(Debug)]
enum Folding {
    XAXIS(i32),
    YAXIS(i32)
}

impl Folding {

    fn fold_on_x(offset: &i32, coordinate: &Coordinate) -> Coordinate {
        println!("fold on x {} {},{}", offset, coordinate.0, coordinate.1);
        let c: Coordinate = match offset {
            offset if offset > &coordinate.0 => Coordinate::from(*coordinate),
            offset => {
                let delta = coordinate.0 - offset;
                Coordinate(offset - delta, coordinate.1)
            }
        };
        println!("{},{}", c.0, c.1);
        return c;
    }

    fn fold_on_y(offset: &i32, coordinate: &Coordinate) -> Coordinate {
        println!("fold on y {} {},{}", offset, coordinate.0, coordinate.1);
        let c = if offset > &coordinate.1 {
            Coordinate::from(*coordinate)
        } else {
            let delta = coordinate.1 - offset;
            Coordinate(coordinate.0, offset - delta)
        };
        println!("{},{}", c.0, c.1);
        return c;
    }

    fn fold(&self, coordinate: &Coordinate) -> Coordinate {
        return match self {
            Folding::XAXIS(offset) => Folding::fold_on_x(offset, coordinate),
            Folding::YAXIS(offset) => Folding::fold_on_y(offset, coordinate)
        }
    }
}

impl FromStr for Folding {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let foldInstructionPattern: Regex = Regex::new(r"fold along ([x|y]=\d+)").unwrap();
        let caps = foldInstructionPattern.captures(input);
        if caps.is_none() {
            return Err("Failed to parse Folding instruction");
        }

        return match caps.unwrap().get(1) {
            None => Err("Failed to parse Folding instruction"),
            Some(instruction) => {
                let mut split = instruction.as_str().split("=");
                let axis = split.next();
                let offset = split.next().unwrap().parse().unwrap();
                match axis {
                    Some("x") => Ok(Folding::XAXIS(offset)),
                    Some("y") => Ok(Folding::YAXIS(offset)),
                    _other => Err("Only supports folding along x or y axis")
                }
            }
        }
    }
}

pub fn partOne(input: &str) -> u32 {
    let coordinatePattern: Regex = Regex::new(r"\d+,\d+").unwrap();
    let points: HashSet<Coordinate> = input.lines()
        .map(|l| l.trim())
        .filter(|line| coordinatePattern.is_match(line))
        .filter_map(|line| line.parse().ok())
        .collect();
    let foldingInstructions: Vec<Folding> = input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .filter(|line| !coordinatePattern.is_match(line))
        .filter_map(|line| line.parse().ok())
        .collect();
    
    let folded: HashSet<Coordinate> = points.iter().map(|p| foldingInstructions[0].fold(p)).collect();
    //let folded: HashSet<Coordinate> = folded.iter().map(|p| foldingInstructions[1].fold(p)).collect();
    // let folded = foldingInstructions.iter()
    //     .fold(points, |folded, folding| folded.iter().map(|point| folding.fold(point)).collect());
    //folded.iter().for_each(|p| println!("{:?}", p));
    return folded.len() as u32;
}

pub fn partTwo(input: &str) -> u32 {
    let coordinatePattern: Regex = Regex::new(r"\d+,\d+").unwrap();
    let points: HashSet<Coordinate> = input.lines()
        .map(|l| l.trim())
        .filter(|line| coordinatePattern.is_match(line))
        .filter_map(|line| line.parse().ok())
        .collect();
    let foldingInstructions: Vec<Folding> = input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .filter(|line| !coordinatePattern.is_match(line))
        .filter_map(|line| line.parse().ok())
        .collect();

    let folded = foldingInstructions.iter()
        .fold(points, |folded, folding| folded.iter().map(|point| folding.fold(point)).collect());
    let x_max = folded.iter().map(|p| p.0).max().unwrap();
    let y_max = folded.iter().map(|p| p.1).max().unwrap();
    for x in 0..(x_max+5) {
        let mut line: String = String::from("");
        for y in 0..(y_max+5) {
            let marker = if folded.contains(&Coordinate(x,y)) { "#" } else { "."};
            line.push_str(marker);
        }
        println!("{}", line);
    }
    //folded.iter().for_each(|p| println!("{:?}", p));
    return folded.len() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneSmallExample() {
        let input = "6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0
        
        fold along y=7
        fold along x=5";
        let res = partOne(input);
        assert_eq!(334, res);
    }
}