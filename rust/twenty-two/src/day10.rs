use std::collections::HashMap;
use std::ops::Range;
use crate::core::{Day};
use crate::day10::Instruction::{AddX, NoOp};

pub struct DayTen {
    input: String
}

impl From<String> for DayTen {
    fn from(input: String) -> Self {
       DayTen { input }
    }
}

enum Instruction {
    NoOp,
    AddX(i32)
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        match input {
            "noop" => NoOp,
            s if s.starts_with("addx") => AddX(s.split(" ").last()
                .map_or(0, |x| x.parse().unwrap())),
            s => panic!("Unknown instruction '{}'", s)
        }
    }
}

impl Day for DayTen {
    type R = i32;

    fn day() -> String where Self: Sized {
        String::from("10")
    }

    fn part_one(&self) -> Self::R {
        let mut cycle: usize = 0;
        let mut X = 1;
        let mut startCycle = cycle;
        let mut operationHistory: HashMap<Range<usize>, i32> = HashMap::new();
        for op in self.input.lines()
            .map(|l| l.trim())
            .map(|l| Instruction::from(l)) {
            let executionTime = match op {
                NoOp => 1,
                AddX(x) => {
                    //println!("AddX start cycle {}, X during execution: {}, end cycle {} X after execution: {}", startCycle, X, cycle + 2, X + x);
                    operationHistory.insert(Range { start: startCycle, end: cycle + 2 }, X);
                    startCycle = cycle + 2;
                    X = X + x;
                    2
                }
            };
            cycle = cycle + executionTime;
            //println!("Cycle: {} X: {}", cycle, X);
        }
        operationHistory.insert(Range { start: startCycle, end: cycle }, X);
        //println!("history: {:?}", operationHistory);
        let samplePoints = vec!(19, 59, 99, 139, 179, 219);
        return samplePoints.iter().map(|cycle| operationHistory.iter()
            .find(|(range, _x)| (*range).contains(cycle))
            .map_or(0, |(_r, x)| {
                //println!("range:{:?} {}", _r, x);
                (*cycle as i32 + 1) * *x
            }))
            .sum();
    }

    fn part_two(&self) -> Self::R {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "addx 15
                addx -11
                addx 6
                addx -3
                addx 5
                addx -1
                addx -8
                addx 13
                addx 4
                noop
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx -35
                addx 1
                addx 24
                addx -19
                addx 1
                addx 16
                addx -11
                noop
                noop
                addx 21
                addx -15
                noop
                noop
                addx -3
                addx 9
                addx 1
                addx -3
                addx 8
                addx 1
                addx 5
                noop
                noop
                noop
                noop
                noop
                addx -36
                noop
                addx 1
                addx 7
                noop
                noop
                noop
                addx 2
                addx 6
                noop
                noop
                noop
                noop
                noop
                addx 1
                noop
                noop
                addx 7
                addx 1
                noop
                addx -13
                addx 13
                addx 7
                noop
                addx 1
                addx -33
                noop
                noop
                noop
                addx 2
                noop
                noop
                noop
                addx 8
                noop
                addx -1
                addx 2
                addx 1
                noop
                addx 17
                addx -9
                addx 1
                addx 1
                addx -3
                addx 11
                noop
                noop
                addx 1
                noop
                addx 1
                noop
                noop
                addx -13
                addx -19
                addx 1
                addx 3
                addx 26
                addx -30
                addx 12
                addx -1
                addx 3
                addx 1
                noop
                noop
                noop
                addx -9
                addx 18
                addx 1
                addx 2
                noop
                noop
                addx 9
                noop
                noop
                noop
                addx -1
                addx 2
                addx -37
                addx 1
                addx 3
                noop
                addx 15
                addx -21
                addx 22
                addx -6
                addx 1
                noop
                addx 2
                addx 1
                noop
                addx -10
                noop
                noop
                addx 20
                addx 1
                addx 2
                addx 2
                addx -6
                addx -11
                noop
                noop
                noop";
        let actual_res = DayTen::from(String::from(input)).part_one();
        assert_eq!(13140, actual_res);
    }
}

