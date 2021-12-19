use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct LeftOrd {
    value: u32,
    ord: u32
}

#[derive(Debug, Clone)]
enum Snailfish {
    Pair(Box<Snailfish>, Box<Snailfish>),
    RightRegular(Box<Snailfish>, LeftOrd),
    LeftRegular(LeftOrd, Box<Snailfish>),
    Regular(LeftOrd, LeftOrd)
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
        let mut leftCount = 0;
        for c in input.chars() {
            if '[' == c || ',' == c {
                // do nothing?
            } else if ']' == c {
                let rhs = stack.pop();
                let lhs = stack.pop();
                
                //println!("{:?}, {:?}", lhs, rhs);
                let res = match (lhs, rhs) {
                    (Some(StackElement::Char(left)), Some(StackElement::Char(right))) => {
                        let leftOrd = leftCount;
                        leftCount += 1;
                        let rightOrd = leftCount;
                        leftCount += 1; 
                        Snailfish::Regular(
                            LeftOrd { value: left.to_digit(10).unwrap(), ord: leftOrd },
                            LeftOrd { value: right.to_digit(10).unwrap(), ord: rightOrd }
                        )
                    },
                    (Some(StackElement::Char(left)), Some(StackElement::Pair(p))) => {
                        let leftOrd = leftCount;
                        leftCount += 1;
                        Snailfish::LeftRegular(LeftOrd {value: left.to_digit(10).unwrap(), ord: leftOrd }, Box::new(p))
                    },
                    (Some(StackElement::Pair(p)), Some(StackElement::Char(right))) => {
                        let leftOrd = leftCount;
                        leftCount += 1;
                        Snailfish::RightRegular(Box::new(p), LeftOrd { value: right.to_digit(10).unwrap(), ord: leftOrd })
                    },
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

#[derive(Debug)]
struct ExplodeState {
    depth: u32,
    order: u32,
    left_stack: Vec<LeftOrd>,
    right_stack: Vec<LeftOrd>,
    exploded: bool
}

impl ExplodeState {
    fn exploding(&self) -> bool {
        return !self.exploded && self.depth == 4;
    }

    fn explode(&mut self, right: Option<LeftOrd>, left: Option<LeftOrd>) {
        self.exploded = true;
        //self.depth -= 1;
        match right {
            None => (),
            Some(r) => self.right_stack.push(r) 
        };
        match left {
            None => (),
            Some(l) => self.left_stack.push(l) 
        };
    }

    fn incDepth(&mut self) {
        self.depth += 1;
    }

    fn new() -> Self {
        return ExplodeState {
            depth: 0,
            order: 0,
            left_stack: Vec::new(),
            right_stack: Vec::new(),
            exploded: false

        }
    }

    fn getLeftValue(&mut self, node: &LeftOrd) -> u32 {
        return match self.left_stack.pop() {
            Some(x) => x.value,
            None if match self.right_stack.last() {
                Some(leftOrd) => node.ord > leftOrd.ord,
                None => false
            } => self.right_stack.pop().unwrap().value,
            None => 0
        };
    }

    fn getRightValue(&mut self, node: &LeftOrd) -> u32 {
        return match self.right_stack.pop() {
            Some(x) => x.value,
            None if match self.left_stack.last() {
                Some(leftOrd) => node.ord < leftOrd.ord,
                None => false
            } => self.left_stack.pop().unwrap().value,
            None => 0
        };
    }

    fn order(&mut self) -> u32 {
        self.order += 1;
        return self.order;
    }
}

impl Snailfish {

    fn addition(&self, rhs: &Snailfish) -> Snailfish {
        // re-caclulate order
        return Snailfish::Pair(Box::new(self.clone()), Box::new(rhs.clone()));
    }

    fn magnitude(&self) -> u64 {
        return match self {
            Snailfish::Regular(lhs, rhs) => (2 * rhs.value + 3 * lhs.value) as u64,
            Snailfish::LeftRegular(lhs, rhs) => (3 * lhs.value) as u64 + 2 * rhs.magnitude(),
            Snailfish::RightRegular(lhs, rhs) => 3 * lhs.magnitude() + (2 * rhs.value) as u64,
            Snailfish::Pair(lhs, rhs) => 3 * lhs.magnitude() + 2 * rhs.magnitude()
        };
    }

    fn explode(&self, recurState: &mut ExplodeState) -> Snailfish {
        println!("{:?} {}", recurState, self.toString());
        recurState.incDepth();
        let res = match self {
            Snailfish::Pair(lhs, rhs) => Snailfish::Pair(
                Box::new(lhs.explode(recurState)),
                Box::new(rhs.explode(recurState))),
            Snailfish::RightRegular(lhs, rhs) if recurState.exploding() => match **lhs {
                Snailfish::Regular(left, right) => {
                    recurState.explode(None, Some(left));
                    Snailfish::Regular(
                        LeftOrd {value: 0, ord: recurState.order() },
                        LeftOrd { value: rhs.value + right.value, ord: recurState.order() }
                    )
                },
                _ => self.clone()
            },
            Snailfish::LeftRegular(lhs, rhs) if recurState.exploding() => match **rhs {
                Snailfish::Regular(left, right) => {
                    recurState.explode(Some(right), None);
                    Snailfish::Regular(
                        LeftOrd { value: lhs.value + left.value, ord: recurState.order() },
                        LeftOrd { value: 0, ord: recurState.order() }
                    )
                },
                _ => self.clone()
            },
            Snailfish::RightRegular(lhs, rhs) => Snailfish::RightRegular(
                Box::new(lhs.explode(recurState)),
                LeftOrd { value: rhs.value + recurState.getRightValue(rhs), ord: recurState.order() }
            ),
            Snailfish::LeftRegular(lhs, rhs) => Snailfish::LeftRegular(
                LeftOrd { value: lhs.value + recurState.getLeftValue(lhs), ord: recurState.order() },
                Box::new(rhs.explode(recurState))),
            Snailfish::Regular(_,_) => self.clone()
        };
        recurState.depth -= 1;
        return res;
    }

    fn reduce(& mut self) {
        self.explode(&mut ExplodeState::new());
    }

    fn toString(&self) -> String {
        let mut out = String::new();
        out.push('[');
        match self {
            Snailfish::Pair(left, right) => {
                out.push_str(&left.toString());
                out.push(',');
                out.push_str(&right.toString());
                },
            Snailfish::Regular(left, right) => {
                 out.push_str(&left.value.to_string());
                 out.push(',');
                 out.push_str(&right.value.to_string());
            },
            Snailfish::LeftRegular(left, right) => {
                out.push_str(&left.value.to_string());
                out.push(',');
                out.push_str(&right.toString());
            },
            Snailfish::RightRegular(left, right) => {
                out.push_str(&left.toString());
                out.push(',');
                out.push_str(&right.value.to_string());
            }
        };
        out.push(']');
        return out;
    }
}

// fn reduce

// fn to split

// fn to re-order

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

    #[test]
    fn explodeTest() {
        let nr: Snailfish = "[[[[[9,8],1],2],3],4]".parse().unwrap();
        let res = nr.explode(&mut ExplodeState::new());
        println!("Exploded {:?}", res);
        assert_eq!(548, res.magnitude());

        let nr: Snailfish = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
        let res = nr.explode(&mut ExplodeState::new());
        println!("Exploded {:?}", res);
        assert_eq!(285, res.magnitude());

        let nr: Snailfish = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
        let res = nr.explode(&mut ExplodeState::new());
        println!("Exploded {:?}", res);
        assert_eq!(402, res.magnitude());

        let nr: Snailfish = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
        let res = nr.explode(&mut ExplodeState::new());
        println!("Exploded {:?}", res);
        assert_eq!(769, res.magnitude());

        let res = res.explode(&mut ExplodeState::new());
        println!("Exploded {}", res.toString());
        assert_eq!(633, res.magnitude());
    }
}