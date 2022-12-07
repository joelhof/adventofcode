use crate::core::{Day};

pub struct DayFour {
    input: String
}

// this implementation should/could be a derive-macro
impl From<String> for DayFour {
    fn from(input: String) -> Self {
        DayFour { input }
    }
}

impl Day for DayFour {
    type R = u64;

    fn day() -> String where Self: Sized {
        String::from("4")
    }

    fn part_one(&self) -> Self::R {
        let mut counter:u64 = 0;
        loop {
            let digest = md5::compute(format!("{}{}", self.input, counter).as_bytes());
            match &digest[..3] {
                [0,0, a] if a <= &9 => break,
                _ => ()
            };
            counter = counter + 1;
        }
        return counter;
    }

    fn part_two(&self) -> Self::R {
        let mut counter:u64 = 0;
        loop {
            let digest = md5::compute(format!("{}{}", self.input, counter).as_bytes());
            match &digest[..3] {
                [0, 0, 0] => break,
                _ => ()
            };
            counter = counter + 1;
        }
        return counter;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        vec![("abcdef", 609043), ("pqrstuv", 1048970)].iter()
            .map(|(l, expected)| (DayFour::from(String::from(*l)).part_one(), expected))
            .for_each(|(actual, expected)| assert_eq!(actual, *expected as u64));
    }
}
