use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
struct BeaconMap {
    scanners: HashMap<usize, Vec<Coordinate>>
}

struct Scanner {
    id: usize,
    overlapping: Vec<Scanner>,
    beacons: Vec<Coordinate>,
    beaconVectors: HashSet<Coordinate>
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd)]
struct Coordinate(i32,i32,i32);
type CoordinateErr = &'static str;

impl Coordinate {
    
    fn createVector(&self, to: &Coordinate) -> Coordinate {
        return Coordinate( to.0 - self.0, to.1 - self.1, to.2 - self.2 );
    }

    fn toVec(&self) -> Vec<i32> {
        return vec![ self.0, self.1, self.2 ];
    }

    fn fromVec(coords: Vec<i32>) -> Result<Self, CoordinateErr> {
        if coords.len() == 2 {
            return Ok(Coordinate(coords[0], coords[1], 0));
        } else if coords.len() == 3 {
            return Ok(Coordinate(coords[0], coords[1], coords[2]));
        }
        return Err("Unable to create Coordinate from Vec");
    }
}

impl FromStr for Coordinate {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let nrs: Result<Vec<i32>, _> = Regex::new(r"(-?\d+)").unwrap().captures_iter(input)
            .filter_map(|cap| cap.get(0))
            .map(|nr| nr.as_str())
            .map(|nr| nr.parse::<i32>())
            .collect();
        
        if let Ok(coords) = nrs {
            if coords.len() == 2 {
                return Ok(Coordinate(coords[0], coords[1], 0));
            } else if coords.len() == 3 {
                return Ok(Coordinate(coords[0], coords[1], coords[3]));
            }
        }
        return Err("Unable to parse into Coordinate");
    }
}

impl FromStr for BeaconMap {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let scanners: HashMap<usize, Vec<Coordinate>> = Regex::new(r"--- scanner \d+ ---").unwrap()
            .split(input)
            .filter(|block| !block.is_empty())
            .enumerate()
            .map(|(i, block)| {
                println!("{}: {}", i, block);
                let scanners: Vec<Coordinate> = block.lines()
                    .map(|l| l.trim())
                    .filter(|l| !l.is_empty())
                    .map(|l| l.parse().unwrap())
                    .collect();
                (i, scanners)
                }
            ).collect();
        return Ok(BeaconMap { scanners: scanners });
    }
}

#[derive(Debug)]
struct Rotation(Vec<Vec<i32>>);

impl Rotation {
    fn axis_rotations() -> Vec<Rotation> {
        let mut rotations: Vec<Rotation> = Vec::new();

        for a in 1..=3 {
            let alpha = a as f32 * (std::f32::consts::PI / 2f32); 
            for b in 1..=3 {
                let beta = b as f32 * (std::f32::consts::PI / 2f32); 
                for g in 1..=3 {
                    let gamma = g as f32 * (std::f32::consts::PI / 2f32);
                    let rot = vec![
                        vec![
                            (beta.cos() * gamma.cos()) as i32,
                            (alpha.sin() * beta.sin() * gamma.cos() - alpha.cos() * gamma.sin()) as i32,
                            (alpha.cos() * beta.sin() * gamma.cos() + alpha.sin() * gamma.sin()) as i32
                        ],
                        vec![
                            (beta.cos() * gamma.sin()) as i32,
                            (alpha.sin() * beta.sin() * gamma.sin() + alpha.cos() * gamma.cos()) as i32,
                            (alpha.cos() * beta.sin() * gamma.sin() - alpha.sin() * gamma.cos()) as i32
                        ],
                        vec![
                            (-beta.sin()) as i32,
                            (alpha.sin() * beta.cos()) as i32,
                            (alpha.cos() * beta.cos()) as i32
                        ]
                    ];
                    rotations.push(Rotation(rot));
                }
            }
        }
        println!("nr of rotations {:?}", rotations.len());
        return rotations;
    }

    fn rotate(&self, columnVector: &Coordinate) -> Coordinate {
        let coords = columnVector.toVec();
        let mut res = Vec::new();
        for row in self.0.iter() {
            let u = row.iter().zip(coords.iter()).map(|(w,k)| w * k).sum();
            res.push(u);
        }
        return Coordinate::fromVec(res).unwrap();
    }
}

pub fn partOne(input: &str) -> u32 {
    let map: BeaconMap = input.parse().unwrap();
    println!("{:?}", map);
    let rotations = Rotation::axis_rotations();
    let mut beaconToBeaconMap: HashMap<usize, HashSet<Coordinate>> = HashMap::new();
    for (scannerId, beacons) in map.scanners.iter() {
        let beaconToBeacon: HashSet<Coordinate> = beacons.iter()
            .tuple_combinations()
            .map(|(p1, p2)| {
                println!("from {:?} to {:?}", p1, p2);
                if p1 >= p2 { p1.createVector(p2) } else { p2.createVector(p1) }
            })
            // .map(|vector| rotations.iter()
            //     .map(move |r| r.rotate(&vector)))
            // .flatten()
            .collect();
        println!("scanner {} beacon to beacon vec {:?}, size {}", scannerId, beaconToBeacon, beaconToBeacon.len());
        beaconToBeaconMap.insert(*scannerId, beaconToBeacon);
    }
    
    /*
    Task is to find unique beacon coordinates.
    First, find which scanners that overlap.
    Map scanners beacon coordinates to common reference coordinates.
    for each scanner calculate vectors between all beacons. Put in a set.
    for each scanner i    
        for each rotation, compare scanner i beaconVectors with scanner j beaconVectors
            if intersection.len() >= 12
                then scanner i overlaps with scanner j.
                map scanner j's beacons to scanner i coordinates, i.e do a translation from j to i.
                    find translation matrix
                add scanner j's mapped beacon coordinates to scanner i's beacons.
            else 
                no overlap, continue to scanner j+1
    */
    
    let mut uniqueBeacons: HashSet<Coordinate> = HashSet::new();
    let scanner0 = beaconToBeaconMap.get(&1).unwrap();
    let scanner1 = beaconToBeaconMap.get(&0).unwrap();
    let intersection = scanner0.intersection(scanner1);
    // println!("intersection: {:?}", intersection);
    // for rotation in rotations.iter() {
    //     let rotated: HashSet<Coordinate> = scanner0.iter().map(|v| rotation.rotate(v)).collect();
    //     println!("{:?}", rotated);
    //     let intersection = rotated.intersection(scanner1);
    //     println!("{:?}", intersection);
    // }

    return intersection.count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parseTest() {
        let input = "--- scanner 0 ---
        0,2
        4,1
        3,3
        
        --- scanner 1 ---
        -1,-1
        -5,0
        -2,1";
        let scanners: BeaconMap = input.parse().unwrap();
        println!("{:?}", scanners.scanners);
        assert_eq!(2, scanners.scanners.len());
    }

    #[test]
    fn partOneTest() {
        let input = "--- scanner 0 ---
        0,2
        4,1
        3,3
        
        --- scanner 1 ---
        -1,-1
        -5,0
        -2,1";
        let res = partOne(&input);
        println!("{:?}", res);
        assert_eq!(3, res);
    }
}
