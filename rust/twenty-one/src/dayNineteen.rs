use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

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

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
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
                return Ok(Coordinate(coords[0], coords[1], coords[2]));
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

#[derive(Debug, PartialEq, Eq, Hash)]
struct Orientation(Vec<Vec<i32>>);

impl Orientation {
    fn axis_rotations() -> Vec<Orientation> {
        let mut orientations: HashSet<Orientation> = HashSet::new();
        for a in 0..=3 {
            let alpha = a as f32 * (std::f32::consts::PI / 2f32); 
            for b in 0..=3 {
                let beta = b as f32 * (std::f32::consts::PI / 2f32); 
                for g in 0..=3 {
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
                    orientations.insert(Orientation(rot));
                }
            }
        }
        println!("nr of rotations {:?}", orientations.len());
        return Vec::from_iter(orientations.into_iter());
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
    let rotations = Orientation::axis_rotations();
    let mut beaconToBeaconMap: HashMap<usize, HashSet<Coordinate>> = HashMap::new();
    for (scannerId, beacons) in map.scanners.iter() {
        let beaconToBeacon: HashSet<Coordinate> = beacons.iter()
            .tuple_combinations()
            .map(|(p1, p2)| {
                //println!("from {:?} to {:?}", p1, p2);
                if p1 >= p2 { p1.createVector(p2) } else { p2.createVector(p1) }
            })
            .collect();
        println!("scanner {} beacon to beacon, size {}", scannerId, beaconToBeacon.len());
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
    let scanner0 = beaconToBeaconMap.get(&0).unwrap();
    let scanner1 = beaconToBeaconMap.get(&1).unwrap();
    let mut sorted0 = Vec::from_iter(scanner0.iter());
    sorted0.sort();
    let mut sorted1 = Vec::from_iter(scanner1.iter());
    sorted1.sort();
    sorted0.iter().zip(sorted1.iter()).for_each(|(a,b)| println!("{:?}      {:?}", a,b));
    let intersection = scanner0.intersection(scanner1);
    println!("intersection: {:?}", intersection);
    for rotation in rotations.iter() {
        let rotated: HashSet<Coordinate> = scanner0.iter().map(|v| rotation.rotate(v)).collect();
        let intersection = rotated.intersection(scanner1);
        println!("intersection {:?}", intersection);
    }

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
    fn partOneMiniTest() {
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


    #[test]
    fn rotationTest() {
        let input = "--- scanner 0 ---
            -1,-1,1
            -2,-2,2
            -3,-3,3
            -2,-3,1
            5,6,-4
            8,0,7";
        let map: BeaconMap = input.parse().unwrap();
        println!("{:?}", map);
        
        let expectedInput = "--- scanner 1 ---
        1,-1,1
        2,-2,2
        3,-3,3
        2,-1,3
        -5,4,-6
        -8,-7,0

        --- scanner 2 ---
        -1,-1,-1
        -2,-2,-2
        -3,-3,-3
        -1,-3,-2
        4,6,5
        -7,0,8

        --- scanner 3 ---
        1,1,-1
        2,2,-2
        3,3,-3
        1,3,-2
        -4,-6,5
        7,0,8

        --- scanner 4 ---
        1,1,1
        2,2,2
        3,3,3
        3,1,2
        -6,-4,-5
        0,7,-8";
        let expected: BeaconMap = expectedInput.parse().unwrap();
        // put scanner 1-4 in a vector of HashSet<Coordinate>
        let expectedRotations: Vec<HashSet<Coordinate>> = expected.scanners.values()
            .map(|beacons| HashSet::from_iter(beacons.iter().cloned()))
            .collect();
        // for each rotation, put rotated points in a Vec<Hashset<Coordinates>>
        let orientations = Orientation::axis_rotations();
        assert_eq!(24, orientations.len());
        orientations.iter().for_each(
            |rot| {
                println!("----");
                map.scanners.get(&0).unwrap().iter().for_each(|v| println!("{:?}", rot.rotate(v)));
            }
        );
        // for each expected Hashset compare against all the rotated ones, if all expected match rotations are ok


        let v1 = Coordinate(-14, -87, 108); // => Coordinate(-14, 87, 108)
        let v2 = Coordinate(-16, -16, 1299); // => Coordinate(-16, 16, 1299)
        println!("{:?}", v1);
        Orientation::axis_rotations().iter().for_each(
            |rot| {
                println!("----");
                println!("{:?} {:?}", rot, rot.rotate(&v1));
                if rot.rotate(&v1) == Coordinate(-14, 87, 108) {
                    println!("{:?} => {:?} = {:?}", rot, v1, Coordinate(-14, 87, 108));
                };
                //rot.rotate(&v2);
            }
        );
        // Coordinate(-33, 26, -1227) => Coordinate(-33, -26, -1227)
        // Coordinate(-36, 6, 33) => Coordinate(-36, -6, 33)
        // Coordinate(-43, 8, 46) => Coordinate(-43, -8, 46)
        //println!("{:?}", res);
        assert_eq!(3, map.scanners.len());
    }

    #[test]
    fn partOneOverlapTest() {
        let input = "--- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401
        
        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390
        
        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562
        
        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596
        
        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14";
        let res = partOne(&input);
        println!("{:?}", res);
        assert_eq!(3, res);
    }
}
