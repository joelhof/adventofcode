use std::collections::HashMap;
use std::fmt::{Display, Formatter};
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
        let operationHistory = self.executeInstructions();
        let samplePoints = vec!(19, 59, 99, 139, 179, 219);
        return samplePoints.iter().map(|cycle| operationHistory.iter()
            .find(|(range, _x)| (*range).contains(cycle))
            .map_or(0, |(_r, x)| {
                (*cycle as i32 + 1) * *x
            }))
            .sum();
    }

    fn part_two(&self) -> Self::R {
        let executionLog = self.executeInstructions();
        let lastCycle = executionLog.keys().map(|k| k.end).max().unwrap();
        let mut display = CrtDisplay::new();
        for cycle in 0..lastCycle {
            let col = cycle % 40;
            let row = cycle / 40;
            if executionLog.iter()
                .find(|(k, _x)| k.contains(&cycle))
                .map_or(false, |(_k, pos)| Range { start: pos-1, end: pos+2 }
                    .contains(&(col as i32) )) {
                display.drawAt(row, col);
            }
        }
        println!("{}", display);
        return 0;
    }

}

impl DayTen {
    fn executeInstructions(&self) -> HashMap<Range<usize>, i32> {
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
                    operationHistory.insert(Range { start: startCycle, end: cycle + 2 }, X);
                    startCycle = cycle + 2;
                    X = X + x;
                    2
                }
            };
            cycle = cycle + executionTime;
        }
        operationHistory.insert(Range { start: startCycle, end: cycle }, X);
        operationHistory
    }
}

struct CrtDisplay {
    pixels: Vec<Vec<char>>
}

impl CrtDisplay {
    fn new() -> Self {
        let mut p = Vec::new();
        for _row in 0..6 {
            let r = vec!['.'; 40];
            p.push(r);
        }
        CrtDisplay { pixels: p }
    }

    fn drawAt(&mut self, row: usize, col: usize) {
        self.pixels[row][col] = '#';
    }
}

impl Display for CrtDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       for row in self.pixels.iter() {
            for pixel in row {
                write!(f, "{}", pixel).expect("Failed to render pixel");
            }
            writeln!(f, "").expect("Failed to render pixel");
        };
        return write!(f, "");
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

    #[test]
    fn partTwoTest() {
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
        DayTen::from(input.to_string()).part_two();
    }
}

