use crate::core::Day;
use itertools::Itertools;

pub struct DayTwo {
    input: String
}

impl From<String> for DayTwo {
    fn from(input: String) -> Self {
        DayTwo { input }
    }
}

struct Present {
    l: u32,
    w: u32,
    h: u32
}

impl Present {
    fn wrapping_paper(&self) -> u32 {
        let areas = vec![
            self.l*self.w,
            self.w*self.h,
            self.h*self.l
        ];
        let smallest_side: &u32 = match areas.iter().min() {
            Some(smallest) => smallest,
            None => panic!("Could not find smallest present side")
        };
        2 * areas[0] + 2 * areas[1] + 2 * areas[2] + smallest_side
    }

    fn ribbon_wrap(&self) -> u32 {
       match vec![self.l, self.w, self.h].iter()
           .tuple_combinations().map(|(a, b)| 2 * a + 2 * b).min() {
           Some(shortest_perimeter) => shortest_perimeter,
           None => 0
       }
    }

    fn ribbon_bow(&self) -> u32 {
        vec![self.l, self.w, self.h].iter().product()
    }
}

impl From<&str> for Present {
    fn from(dimensions: &str) -> Self {
        let d = dimensions.split('x')
            .filter_map(|size| size.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        Present {
            l: d[0],
            w: d[1],
            h: d[2]
        }
    }
}

impl Day for DayTwo {
    type R = u32;

    fn day() -> String where Self: Sized {
        String::from("2")
    }

    fn part_one(&self) -> Self::R {
        self.input.lines()
            .map(|l| Present::from(l))
            .map(|p| p.wrapping_paper())
            .sum::<u32>()
    }

    fn part_two(&self) -> Self::R {
        self.input.lines()
            .map(|l| Present::from(l))
            .map(|p| p.ribbon_wrap() + p.ribbon_bow())
            .sum::<u32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = String::from("2x3x4");
        let mut res = DayTwo { input }.part_one();
        assert_eq!(res, 58);
        let input = String::from("1x1x10");
        res = DayTwo { input }.part_one();
        assert_eq!(res, 43);
    }

    #[test]
    fn ribbonLengthExampleTest() {
        vec![("2x3x4", 10), ("1x1x10", 4)].iter()
            .map(|(l, expected)| (Present::from(*l).ribbon_wrap(), expected))
            .for_each(|(actual, expected)| assert_eq!(actual, *expected as u32));
    }

    #[test]
    fn ribbonBowExampleTest() {
        vec![("2x3x4", 24), ("1x1x10", 10)].iter()
            .map(|(l, expected)| (Present::from(*l).ribbon_bow(), expected))
            .for_each(|(actual, expected)| assert_eq!(actual, *expected as u32));
    }
    #[test]
    fn partTwoExampleTest() {
        let input = String::from("2x3x4");
        let day = DayTwo { input };
        let mut res = day.part_two();
        assert_eq!(res, 34);
        let input = String::from("1x1x10");
        res = DayTwo { input }.part_two();
        assert_eq!(res, 14);
    }
}
