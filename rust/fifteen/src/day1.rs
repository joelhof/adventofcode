use crate::core::{Day, load_input};

pub struct DayOne {

}

impl DayOne {

    fn problemOne(input: &str) -> i32 {
        let mut floor = 0;
        input.chars().for_each(|c| match c {
            '(' => floor = floor + 1,
            ')' => floor = floor - 1,
            _ => {}
        });
        return floor;
    }

    fn problemTwo(input: &str) -> i32 {
        let mut floor = 0;
        for (i, c) in input.chars().enumerate()
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
}

impl Day for DayOne {
    type R = i32;

    fn part_one() -> Self::R {
        let dayOneInput = load_input("1");
        return match dayOneInput {
            Ok(input) => DayOne::problemOne(&input),
            Err(_err) => {
                println!("Failed to read input!");
                panic!();
            }
        };
    }

    fn part_two() -> Self::R {
        let dayOneInput = load_input("1");
        return match dayOneInput {
            Ok(input) => DayOne::problemTwo(&input),
            Err(_err) => {
                println!("Failed to read input!");
                panic!();
            }
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "(())";
        let res = DayOne::problemOne(input);
        assert_eq!(res, 0);
        let input = "()()";
        let res = DayOne::problemOne(input);
        assert_eq!(res, 0);
    }

    #[test]
    fn partOneExample2Test() {
        let input = "(((";
        let res = DayOne::problemOne(input);
        assert_eq!(res, 3);
        let input = "))(((((";
        let res = DayOne::problemOne(input);
        assert_eq!(res, 3);
        let input = ")())())";
        let res = DayOne::problemOne(input);
        assert_eq!(res, -3);
    }

    #[test]
    fn partTwoExampleTest() {
        let input = ")";
        let res = DayOne::problemTwo(input);
        assert_eq!(res, 1);
        let input = "()())";
        let res = DayOne::problemTwo(input);
        assert_eq!(res, 5);
    }

}