use crate::core::{Day};

pub struct DayOne {
    input: String
}

impl Day for DayOne {
    type R = i32;

    fn day() -> String {
       String::from("1")
    }

    fn part_one(&self) -> Self::R {
        let mut floor = 0;
        self.input.chars().for_each(|c| match c {
            '(' => floor = floor + 1,
            ')' => floor = floor - 1,
            _ => {}
        });
        floor
    }

    fn part_two(&self) -> Self::R {
        let mut floor = 0;
        for (i, c) in self.input.chars().enumerate()
        {
            match c {
                '(' => floor = floor + 1,
                ')' => floor = floor - 1,
                _ => {}
            };
            if floor < 0 {
                return i as i32 + 1;
            }
        };
        return 0;
    }

    fn from(input: String) -> Box<dyn Day<R=Self::R>> {
        Box::new(DayOne { input })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "(())";
        let mut res = 0;
        input.chars().for_each(|c| match c {
            '(' => res = res + 1,
            ')' => res = res - 1,
            _ => {}
        });
        assert_eq!(res, 0);
        let input = "()()";
        let mut res = 0;
        input.chars().for_each(|c| match c {
            '(' => res = res + 1,
            ')' => res = res - 1,
            _ => {}
        });
        assert_eq!(res, 0);
    }

    #[test]
    fn partOneExample2Test() {
        let input = String::from("(((");
        let d = DayOne { input };
        let res = d.part_one();
        assert_eq!(res, 3);
        let input = String::from("))(((((");
        let d = DayOne { input };
        let res = d.part_one();
        assert_eq!(res, 3);
        let input = String::from(")())())");
        let d = DayOne { input };
        let res = d.part_one();
        assert_eq!(res, -3);
    }

    #[test]
    fn partTwoExampleTest() {
        let input = String::from(")");
        let d = DayOne { input };
        let res = d.part_two();
        assert_eq!(res, 1);
        let input = String::from("()())");
        let d = DayOne { input };
        let res = d.part_two();
        assert_eq!(res, 5);
    }

}