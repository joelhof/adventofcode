#![allow(non_snake_case)]

use crate::core::*;

pub struct PartOne {
    arrival: u64,
    schedule: Vec<u64>
}

impl PartOne {
    fn init(input: &str) -> PartOne {
        let mut lines = input.split("\n")
            .map(|line| line.trim());
        let a: u64 = lines.next().unwrap().parse().unwrap();
        let schedule: Vec<u64> = lines.next().unwrap()
            .replace("x", "")
            .split(",")
            .filter_map(|s| Some(s.parse()))
            .filter_map(Result::ok)
            .collect();
            
        return PartOne {
            arrival: a,
            schedule: schedule
        };
    }

    pub fn new() -> PartOne {
        return PartOne::init(&loadInput("Thirteen"));
    }
}

impl AdventOfCodeSolver for PartOne {
    fn day(&self) -> &str {
        return "Thirteen";
    }

    fn partOne(&self) -> u64 {
        let mut ratios: Vec<(u64, u64)> = self.schedule[..].into_iter()
            .map(|freq| (freq, self.arrival / freq, self.arrival % freq))
            .map(|(f, n, m)| (multiple((*f, n, m)) - self.arrival, *f))
            .collect();
        ratios.sort_by(|(wait1, _id1), (wait2, _id2)| wait1.cmp(&wait2));
        return match ratios.first() {
            Some((departure, id)) => departure * id,
            None => 0
        };
    }

    fn partTwo(&self) -> u64 {
        let part2 = PartTwo::new();
        return 0;//part2.solve();
    }
 }

fn multiple(input: (u64, u64, u64)) -> u64 {
    let (f, n, m) = input;
    if m > 0 {
        return f * (n + 1);
    } else {
        return f * n;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Constraint {
    offset: u64,
    id: Result<u64, std::num::ParseIntError>
}

impl Constraint {
    fn checkCandidate(&self, candidate: u64) -> bool {
        return match self.id {
            Ok(id) => (candidate + self.offset) % id == 0,
            Err(_) => true
        }
    }
}

pub struct PartTwo {
    constraints: Vec<Constraint>
}

impl PartTwo {
    fn init(input: &str) -> PartTwo {
        let mut lines = input.split("\n")
            .map(|line| line.trim());
        lines.next();
        let mut schedule: Vec<Constraint> = lines.next().unwrap()
            .split(",")
            .enumerate()
            .map(|(i, s)| Constraint {
                offset: i as u64,
                id: s.parse()
            })
            .collect();
        schedule.sort_by(|a, b| {
            let idA = match &a.id {
                Ok(x) => *x as i64,
                Err(_e) => -1
            };
            let idB = match &b.id {
                Ok(x) => *x as i64,
                Err(_e) => -1
            };
            return idA.cmp(&idB);
        });
        return PartTwo {
            constraints: schedule
        };
    }

    pub fn new() -> PartTwo {
        return PartTwo::init(&loadInput("Thirteen"));
    }

    fn solve(&self) -> u64 {
        //t     = 7 * x1
        //t + 1 = 13 * x2
        //t + 2 = n
        //t + 3 = m
        //t + 4 = 59 * x5 => t = 59 * x5 - 4
        //t + 5 = k
        //t + 6 = 31 * x7
        //t + 7 = 19 * x8 
        // => 
        //t     = 7 * x1
        //7 * x1 + 1 = 13 * x2 => x2 = (t + 1) / 13 = (7 * x1) / 13
        //t + 4 = 59 * x5 => t + 1 + 3 = 59 * x5 => (7 * x1) / 13 + 3 = 59 * x5
        //t + 6 = 31 * x7 => t + 1 + 5 = 31 * x7
        //t + 7 = 19 * x8 => t + 1 + 6 = 19 * x8
        /*
            0|  0   1   2   3   4   5   6   7   8
            -------------------------------------               
            0|  7                                               | x1
            1|      13                                          | x2  
            2|          0                                       | x3
            3|              0                                   | x4
            4|                  59                          X   | x5    = t
            5|                      0                           | x6
            6|                          31                      | x7
            7|                              19                  | x8
        */

        let mut constraints: Vec<Constraint> = self.constraints.iter()
            .cloned()
            .filter(|condition| condition.id.is_ok())
            .collect();
        
        let mut n = 1;
        println!("orignal length {}", constraints.len());
        let constraint = match constraints.last() {
            Some(c) => c,
            None => return 0
        };

        
        while n < u64::MAX {
            let candidateT = constraint.id.as_ref().unwrap() * n - constraint.offset;
            if self.checkCandidate(candidateT) {
                return candidateT;
            }
            if n % 1000 == 0 {
                println!("n={}", n);
            }

            n += 1;
        }
        return 0;
    }

    fn recurseCheck(mut constraints: Vec<Constraint>, candidate: u64) -> Option<u64> {
        match constraints.pop() {
            Some(c) if c.checkCandidate(candidate) => PartTwo::recurseCheck(constraints, candidate),
            Some(_) => None,
            None => Some(candidate)
        }
    }

    fn checkCandidate(&self, candidate: u64) -> bool {
        return self.constraints.iter()
            .filter(|condition| condition.id.is_ok())
            .map(|condition| (condition.offset, condition.id.as_ref().unwrap()))
            .map(|(i, v)| {
                let res = (candidate + i as u64) % v;
                //println!("{} {}", v, res);
                return res;
            })
            .all(|m| m == 0);
        //return false;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn thirteenPartOneExampleTest() {
        const INPUT: &str = "939
        7,13,x,x,59,x,31,19";
        let result = PartOne::init(INPUT).partOne();
        assert_eq!(result, 295);
    }

    #[test]
    fn thirteenPartTwoExampleTest() {
        const INPUT: &str = "939
        7,13,x,x,59,x,31,19";
        let result = PartTwo::init(INPUT).solve();
        assert_eq!(result, 1068781);
    }

    #[test]
    fn thirteenPartTwoTest2() {
        const INPUT: &str = "939
        67,x,7,59,61";
        let result = PartTwo::init(INPUT).solve();
        assert_eq!(result, 779210);
    }

    #[test]
    fn thirteenPartTwoTest3() {
        const INPUT: &str = "939
        67,7,59,61";
        let result = PartTwo::init(INPUT).solve();
        assert_eq!(result, 754018);
    }

    #[test]
    fn thirteenPartTwoTest4() {
        const INPUT: &str = "939
        67,7,x,59,61";
        let result = PartTwo::init(INPUT).solve();
        assert_eq!(result, 1261476);
    }

    #[test]
    fn thirteenPartTwoTest5() {
        const INPUT: &str = "939
        1789,37,47,1889";
        let result = PartTwo::init(INPUT).solve();
        assert_eq!(result, 1202161486);
    }
}