use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
enum Snailfish {
    Pair(Box<Snailfish>, Box<Snailfish>),
    RightRegular(Box<Snailfish>, u32),
    LeftRegular(u32, Box<Snailfish>),
    Regular(u32,u32)
}

lazy_static! {
    static ref REGULAR_NUMBERS: Regex = Regex::new(r"\[(\d+),(\d+)\]").unwrap();
}

#[derive(Debug)]
enum StackElement {
    Char(char),
    Pair(Snailfish)
}

impl FromStr for Snailfish {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<StackElement> = Vec::new();

        for c in input.chars() {
            if '[' == c || ',' == c {
                // do nothing?
            } else if ']' == c {
                let rhs = stack.pop();
                
                let lhs = stack.pop();
                println!("{:?}, {:?}", lhs, rhs);
                let res = match (lhs, rhs) {
                    (Some(StackElement::Char(left)), Some(StackElement::Char(right))) => Snailfish::Regular(
                        left.to_digit(10).unwrap(), right.to_digit(10).unwrap()),
                    (Some(StackElement::Char(left)), Some(StackElement::Pair(p))) => Snailfish::LeftRegular(left.to_digit(10).unwrap(), Box::new(p)),
                    (Some(StackElement::Pair(p)), Some(StackElement::Char(right))) => Snailfish::RightRegular(Box::new(p), right.to_digit(10).unwrap()),
                    (Some(StackElement::Pair(left)), Some(StackElement::Pair(right))) => Snailfish::Pair(Box::new(left), Box::new(right)),
                    (_,_) => return Err("Unable to create SnailfishNumber")

                };
                stack.push(StackElement::Pair(res));
            } else {
                stack.push(StackElement::Char(c));
            }
        }
        return match stack.pop() {
            Some(StackElement::Pair(p)) => Ok(p),
            _ => Err("Unable to parse into Snailfish Number")
        };
    }
}

impl Snailfish {
    pub fn addition(&self, rhs: &Snailfish) -> Snailfish {
        return Snailfish::Pair(Box::new(self.clone()), Box::new(rhs.clone()));
    }

    pub fn magnitude(&self) -> u64 {
        return match self {
            Snailfish::Regular(lhs, rhs) => (2 * rhs + 3 * lhs) as u64,
            Snailfish::LeftRegular(lhs, rhs) => (3 * lhs) as u64 + 2 * rhs.magnitude(),
            Snailfish::RightRegular(lhs, rhs) => 3 * lhs.magnitude() + (2 * rhs) as u64,
            Snailfish::Pair(lhs, rhs) => 3 * lhs.magnitude() + 2 * rhs.magnitude()
        };
    }
}

// fn addition

// fn reduce

// fn to explode

// fn to split

// fn magnitude

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let lhs: Snailfish = "[1,2]".parse().unwrap();
        let rhs: Snailfish = "[[3,4],5]".parse().unwrap();
        let res = lhs.addition(&rhs);
        println!("{:?}", res);
        assert_eq!(1, 0);
    }

    #[test]
    fn magnitudeTest() {
        let lhs: Snailfish = "[9,1]".parse().unwrap();
        let res = lhs.magnitude();
        assert_eq!(29, res);

        let lhs: Snailfish = "[[9,1],[1,9]]".parse().unwrap();
        let res = lhs.magnitude();
        assert_eq!(129, res);

        let lhs: Snailfish = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();
        let res = lhs.magnitude();
        assert_eq!(1384, res);
        
        let lhs: Snailfish = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse().unwrap();
        let res = lhs.magnitude();
        assert_eq!(3488, res);
    }
}