use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct LeftOrd {
    value: u32,
    ord: u32
}

#[derive(Debug, Clone)]
enum SnailfishNode {
    Leaf(LeftOrd),
    Node(Box<SnailfishNode>, Box<SnailfishNode>)
}

impl FromStr for SnailfishNode {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<SnailfishNode> = Vec::new();
        let mut numberStack: Vec<String> = Vec::new();
        let mut leftCount = 1;
        for c in input.chars() {
            if '[' == c || ',' == c {
                match numberStack.pop() {
                    Some(nr_string) => {
                        let nr: u32 = nr_string.parse().unwrap();
                        let snailfish = LeftOrd { value: nr, ord: leftCount };
                        stack.push(SnailfishNode::Leaf(snailfish));
                        leftCount += 1;
                    },
                    None => ()
                };
            } else if ']' == c {
                match numberStack.pop() {
                    Some(nr_string) => {
                        let nr: u32 = nr_string.parse().unwrap();
                        let snailfish = LeftOrd { value: nr, ord: leftCount };
                        stack.push(SnailfishNode::Leaf(snailfish));
                        leftCount += 1;
                    },
                    None => ()
                };
                let rhs = stack.pop();
                let lhs = stack.pop();
                
                let res = match (lhs, rhs) {
                    (Some(left), Some(right)) => SnailfishNode::Node(Box::new(left), Box::new(right)),
                    (_,_) => return Err("Unable to create SnailfishNode")
                };
                stack.push(res);
            } else {
                if let Some(currentToken) = numberStack.last_mut() {
                    currentToken.push(c);
                } else {
                    numberStack.push(c.to_string());
                }
            }
        }
        return match stack.pop() {
            Some(node) => Ok(node),
            _ => Err("Unable to parse into SnailfishNode")
        };
    }
}

impl SnailfishNode {
    fn toString(&self) -> String {
        let mut out = String::new();
        match self {
            SnailfishNode::Leaf(ordinal) => {
                out.push_str(&ordinal.value.to_string());
                },
            SnailfishNode::Node(lhs, rhs) => {
                out.push('[');
                out.push_str(&lhs.toString());
                out.push(',');
                out.push_str(&rhs.toString());
                out.push(']');
            }
        };
        return out;
    }

    fn magnitude(&self) -> u64 {
        return match self {
            SnailfishNode::Leaf(ordinal) => ordinal.value as u64,
            SnailfishNode::Node(lhs, rhs) => 3 * lhs.magnitude() + 2 * rhs.magnitude()
        };    
    }

    fn addition(&self, rhs: &SnailfishNode) -> SnailfishNode {
        let res = SnailfishNode::Node(Box::new(self.clone()), Box::new(rhs.clone()))
            .order(&mut ExplodeState::new());
        return res;
    }

    fn order(&self, recurState: &mut ExplodeState) -> SnailfishNode {
        let res = match self {
            SnailfishNode::Leaf(leaf) => SnailfishNode::Leaf(LeftOrd {
                value: leaf.value,
                ord: recurState.order()
            }),
            SnailfishNode::Node(lhs, rhs) => SnailfishNode::Node(
                Box::new(lhs.order(recurState)),
                Box::new(rhs.order(recurState))
            )
        };
        return res;
    }

    fn split(&self, recurState: &mut ExplodeState) -> SnailfishNode {
        let res = match self {
            SnailfishNode::Node(lhs, rhs) => SnailfishNode::Node(
                Box::new(lhs.split(recurState)), Box::new(rhs.split(recurState))
            ),
            SnailfishNode::Leaf(leaf) if leaf.value >= 10 && !recurState.exploded => {
                //println!("Splitting node {}", &self.toString());
                recurState.exploded = true;
                SnailfishNode::Node(
                    Box::new(SnailfishNode::Leaf(LeftOrd {
                        value: (leaf.value as f32 / (2 as f32)).floor() as u32,
                        ord: recurState.order()
                    })),
                    Box::new(SnailfishNode::Leaf(LeftOrd {
                        value: (leaf.value as f32 / (2 as f32)).ceil() as u32,
                        ord: recurState.order() 
                    }))
            )},
            SnailfishNode::Leaf(leaf) => SnailfishNode::Leaf(LeftOrd {
                value: leaf.value,
                ord: recurState.order() 
            })
        };
        return res;
    }

    fn updateLeafAt(&mut self, leafOrdinal: u32, added: u32) -> Option<u32> {
       
        // if self is a leaf and has the given leafOrdinal, update self value to self += value
        match self {
            SnailfishNode::Node(lhs, rhs) => if let Some(old_value) = lhs.updateLeafAt(leafOrdinal, added) {
                Some(old_value)
            } else {
                rhs.updateLeafAt(leafOrdinal, added)
            },
            SnailfishNode::Leaf(leaf) if leaf.ord == leafOrdinal => {
                let old_value = leaf.value;
                leaf.value += added;
                return Some(old_value);
            },
            SnailfishNode::Leaf(_leaf) => None
        }
    }

    fn explode(&mut self) -> Option<(LeftOrd, LeftOrd)> {
        let mut state = ExplodeState::new();
        let res = self.explodeSelf(&mut state);
        if let Some((lhs, rhs)) = res {
            if let Some(leftOrd) = lhs.ord.checked_sub(1) {
                let _old_left = self.updateLeafAt(leftOrd, lhs.value);
                //println!("old left value: {:?}", old_left);
            } 
            let _old_right = self.updateLeafAt(rhs.ord + 1, rhs.value);
            //println!("old right value: {:?}", old_right);
        };
        *self = self.order(&mut ExplodeState::new());
        /*
        find left and right neigbour leafs to exploded leafs
        update their values
        update ordering 
        */
        return res;
    }

    fn explodeSelf(&mut self, recurState: &mut ExplodeState) -> Option<(LeftOrd, LeftOrd)> {
        //println!("------");
        //println!("{:?} {}", recurState, self.toString());
        recurState.incDepth();
        let res = match self {
            SnailfishNode::Node(lhs, rhs) => {
                match (*lhs.clone(), *rhs.clone()) {
                    (SnailfishNode::Leaf(left), SnailfishNode::Leaf(right)) if recurState.exploding() => {
                        //println!("Explode node: {:?}", &self.toString());
                        recurState.explode(Some(right), Some(left));
                        *self = SnailfishNode::Leaf(LeftOrd { value: 0, ord: u32::MAX });
                        Some((left, right))
                    },
                    (_left, _right) => {
                        // left and right are clones here, not the true objects
                        if let Some(exploded) = lhs.explodeSelf(recurState) {
                            Some(exploded)
                        } else {
                            rhs.explodeSelf(recurState)
                        }
                    }
                }
            },
            SnailfishNode::Leaf(_leaf) => None
        };
        recurState.depth -= 1;
        //println!("post explode {:?} new node {:?}", recurState, self.toString());
        return res;
    }

    fn reduce(&mut self) -> SnailfishNode {
        // Keep exploding or splitting until no change
        // 1. explode
        // 2. if explode does nothing, try split
        // else goto 1
        // if split does nothing, return
        // else return to step 1
        let exploded = self.explode();
        //println!("after explode: {:?}", self);
        if exploded.is_none() {
            let mut splitted = self.split(&mut ExplodeState::new());
            //println!("after split: {:?}", splitted);
            if splitted.toString() == self.toString() {
                return splitted;
            }
            else {
                return splitted.reduce(); 
            }
        } else {
            return self.reduce();
        }
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
        //println!("{:?} {:?}", self.exploded, self.depth);
        return !self.exploded && self.depth == 5;
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

    fn order(&mut self) -> u32 {
        self.order += 1;
        return self.order;
    }
}

pub fn partOne(input: &str) -> u64 {
    let res: Option<SnailfishNode> = input.lines()
        .map(|l| l.trim())
        .map(|l| {
            //println!("{}", l);
            l.parse().unwrap()
        })
        .reduce(|result: SnailfishNode, term| {
            let mut sum: SnailfishNode = result.addition(&term);
            sum.reduce()
        });
    //println!("final node {}", match &res { Some(n) => n.toString(), None => "".to_string() } );
    return match res {
        Some(node) => node.magnitude(),
        None => 0
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parseTest() {
        let lhs: SnailfishNode = "[1,2]".parse().unwrap();
        assert_eq!("[1,2]", lhs.toString());
        let rhs: SnailfishNode = "[[3,4],5]".parse().unwrap();
        assert_eq!("[[3,4],5]", rhs.toString());
        let node: SnailfishNode = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]".parse().unwrap();
        assert_eq!("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]", node.toString());
    }

    #[test]
    fn magnitudeTest() {
        let lhs: SnailfishNode = "[9,1]".parse().unwrap();
        let res = lhs.magnitude();
        assert_eq!(29, res);

        let lhs: SnailfishNode = "[[9,1],[1,9]]".parse().unwrap();
        let res = lhs.magnitude();
        assert_eq!(129, res);

        let lhs: SnailfishNode = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();
        let res = lhs.magnitude();
        assert_eq!(1384, res);

        let lhs: SnailfishNode = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse().unwrap();
        let res = lhs.magnitude();
        assert_eq!(3488, res);
    }

    #[test]
    fn explodeTest() {
        let mut nr: SnailfishNode = "[[[[[9,8],1],2],3],4]".parse().unwrap();
        let res = nr.explode();
        println!("Exploded {:?}", nr.toString());
        assert_eq!(548, nr.magnitude());

        let mut nr: SnailfishNode = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
        let res = nr.explode();
        assert_eq!("[7,[6,[5,[7,0]]]]", nr.toString());
        assert_eq!(285, nr.magnitude());

        let mut nr: SnailfishNode = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
        let res = nr.explode();
        println!("Exploded {:?}", res);
        assert_eq!(402, nr.magnitude());

        let mut nr: SnailfishNode = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
        let res = nr.explode();
        println!("Exploded {:?}", res);
        assert_eq!(769, nr.magnitude());

        let res = nr.explode();
        println!("Exploded {:?}", res);
        assert_eq!(633, nr.magnitude());

        
        let mut nr: SnailfishNode = "[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]".parse().unwrap();
        let res = nr.explode();
        assert_eq!("[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]", nr.toString());
        assert_eq!(4888, nr.magnitude());
    }

    #[test]
    fn addTest() {
        let term1: SnailfishNode = "[1,1]".parse().unwrap();
        let term2: SnailfishNode  = "[2,2]".parse().unwrap();
        let term3: SnailfishNode  = "[3,3]".parse().unwrap();
        let term4: SnailfishNode  = "[4,4]".parse().unwrap();
        let res = term1.addition(&term2);
        let res = res.addition(&term3);
        let res = res.addition(&term4);
        assert_eq!("[[[[1,1],[2,2]],[3,3]],[4,4]]", res.toString());

        // let term3: SnailfishNode  = "[3,3]".parse().unwrap();
        // let term4: SnailfishNode  = "[]".parse().unwrap();
        // let res = term3.addition(&term4);
        // assert_eq!("[3,3]", res.toString());
    }

    #[test]
    fn splitTest() {
        let split: SnailfishNode = match "[[[[0,7],4],[15,[0,13]]],[1,1]]".parse::<SnailfishNode>().ok() {
            Some(n) => n.split(&mut ExplodeState::new()),
            None => SnailfishNode::Leaf(LeftOrd { value: 0, ord: 0 })
        };
        assert_eq!("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", split.toString());

        // let input = "[1,1]
        // [2,2]
        // [3,3]
        // [4,4]
        // [5,5]";
        // let res: Option<SnailfishNode> = input.lines().map(|l| l.parse().unwrap()).reduce(|result: SnailfishNode, term| {
        //     let sum: SnailfishNode = result.addition(&term);
        //     sum.reduce()
        // });
        // assert_eq!("[[[[3,0],[5,3]],[4,4]],[5,5]]", res.unwrap().toString());
       
        // let split1: Snailfish = "[[[[0,7],4],[15,[0,13]]],[1,1]]".parse().unwrap();
        // let res = split1.split(&mut ExplodeState::new());
        // println!("{}", res.toString());
        // assert_eq!("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", res.toString());
    }

    #[test]
    fn reduceTest() {
        let lhs: SnailfishNode = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let rhs: SnailfishNode = "[1,1]".parse().unwrap();
        let mut sum = lhs.addition(&rhs);
        assert_eq!("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", sum.toString());
        let res = sum.reduce();
        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", res.toString());
        println!("-----------------------------");
        println!("");
        let lhs: SnailfishNode = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".parse().unwrap();
        let rhs: SnailfishNode = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".parse().unwrap();
        let mut sum = lhs.addition(&rhs);
        println!("{:?}", sum);
        let res = sum.reduce();
        assert_eq!("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]", res.toString());
    }

    #[test]
    fn partOneExamplesTest() {
        let input = "[1,1]
        [2,2]
        [3,3]
        [4,4]";
        let res = partOne(&input);
        assert_eq!(445, res);

        let input = "[1,1]
        [2,2]
        [3,3]
        [4,4]
        [5,5]";
        let res = partOne(&input);
        assert_eq!(791, res);

        let input = "[1,1]
        [2,2]
        [3,3]
        [4,4]
        [5,5]
        [6,6]";
        let res = partOne(&input);
        assert_eq!(1137, res);

        let input = "[[[[4,3],4],4],[7,[[8,4],9]]]
        [1,1]";
        let res = partOne(&input);
        assert_eq!(1384, res);

        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let res = partOne(&input);
        assert_eq!(4140, res);
    }
}