use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::VecDeque;
use crate::core::{Day};

pub struct DayThirteen {
    input: String
}

impl From<String> for DayThirteen {
    fn from(input: String) -> Self {
       DayThirteen { input }
    }
}

#[derive(Debug, Clone)]
enum PacketData {
    INT(u32),
    LIST(Vec<PacketData>)
}

impl PacketData {
    fn compare(&self, other: &PacketData) -> Option<bool> {
        //println!("lhs: {:?} rhs: {:?}", self, other);
        match (self, other) {
            (PacketData::INT(lhs), PacketData::INT(rhs)) => Self::compare_ints(lhs, rhs),
            (PacketData::LIST(lhs), PacketData::LIST(rhs)) => Self::compare_list(lhs, rhs),
            (PacketData::LIST(lhs), p) => Self::compare_list(lhs,&vec![p.clone()]),
            (p, PacketData::LIST(rhs)) => Self::compare_list(&vec![p.clone()], rhs)
        }
    }

    fn compare_ints(lhs: &u32, rhs: &u32) -> Option<bool> {
        //print!("comparing ints: lhs: {} rhs {}", lhs, rhs);
        if lhs < rhs {
           // println!(" -> lhs is smaller, right order");
           return Some(true);
        } else if lhs > rhs {
            //println!(" -> lhs is bigger, wrong order");
           return Some(false);
        }
        //print!(" -> equal, continue");
        None
    }

    fn compare_list(lhs: &Vec<PacketData>, rhs: &Vec<PacketData>) -> Option<bool> {
        //println!("list comparison lhs: {:?} rhs: {:?} ", lhs, rhs);
        for (i, left) in lhs.iter().enumerate() {

            // If left side ends first
            // if left.is_none() {
            //     println!("lhs ended first none");
            //     return Some(true);
            // }
            // if right side ends first
            let right = rhs.get(i);
            if right.is_none() {
                //println!("rhs ended first");
                return Some(false);
            }
            // compare
            let right = right.unwrap();

            match left.compare(right) {
                Some(res) => return Some(res),
                None => continue
            }

        }
        //println!("lhs {:?} ended, rhs {:?}", lhs, rhs);
        if lhs.len() == rhs.len() {
            //println!("Equal length, continue {:?} {:?}", lhs, rhs);
            return None;
        } else if rhs.len() > lhs.len() {
            //println!("zxcxzclhs ended first, {:?} {:?}", lhs, rhs);
            return Some(true);
        }
        panic!("Left side ran out but right was smaller!?")
    }


}

struct Pair {
    lhs: PacketData,
    rhs: PacketData
}

impl Pair {
    fn compare(&self) -> bool {
        //println!("-------------");
        //println!("Comparing Pair of {:?} and {:?}", self.lhs, self.rhs);
        let res = self.lhs.compare(&self.rhs);
        //println!("Comparison result: {:?}", res);
        match res {
            Some(comp) => comp,
            None => false
        }
    }

    fn toVec(&self) -> Vec<&PacketData> {
        vec![&self.lhs, &self.rhs]
    }

    fn parse(packet_data: &mut VecDeque<char>) -> PacketData {
        let mut element = Vec::new();
        let mut child = None;
        let mut list = vec![];
        while let Some(left) = packet_data.pop_front() {
            //println!("{}", left);
            if left.is_digit(10) {
                element.push(left);
            } else if '[' == left {
                //println!("new PacketData");
                child = Some(Self::parse(packet_data));
                //println!("child: {:?}", child);
            } else if ',' == left {
                let nr: String = element.iter().collect();
                list.push(child.or(
                    nr.parse().map_or(
                        None,
                        |n| Some(PacketData::INT(n))
                    )
                ));
                element.clear();
                child = None;
                //println!("Added element {:?}", list.last());
            } else if ']' == left {
                let nr: String = element.iter().collect();
                list.push(child.or(
                    nr.parse().map_or(
                        None,
                        |n| Some(PacketData::INT(n))
                    )
                ));
                //println!("end of list {:?}", list);
                return PacketData::LIST(list.into_iter()
                    .filter(|c| c.is_some())
                    .map(|c| c.unwrap())
                    .collect::<Vec<PacketData>>().clone()
                );
            }
        }
       child.expect("Not a Packet!")

    }
}

impl From<&str> for Pair {
    fn from(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().map(|l| l.trim())
            .collect();
        if lines.len() == 2 {
            let lhs = Pair::parse(&mut lines[0].chars().collect());
            let rhs = Pair::parse(&mut lines[1].chars().collect());
            Pair { lhs, rhs }
        } else { panic!("A Pair must have exactly 2 sides, {}", input) }
    }
}

impl Day for DayThirteen {
    type R = u32;

    fn day() -> String where Self: Sized {
        "13".to_string()
    }

    fn part_one(&self) -> Self::R {
        let pairs: Vec<Pair> = self.input.split("\n\n")
            .map(|pair| Pair::from(pair))
            .collect();
        let mut right: Vec<usize> = Vec::new();
        for (i, pair) in pairs.iter().enumerate() {
            if pair.compare() {
                right.push(i);
            }
        }

        right.iter().map(|i| (i+1) as u32).sum()
    }

    fn part_two(&self) -> Self::R {

        let pairs: Vec<Pair> = self.input.split("\n\n")
            .map(|pair| Pair::from(pair))
            .collect();
        let mut packets: Vec<&PacketData> = pairs.iter()
            .flat_map(|p| p.toVec()).collect();
        let cp1 = PacketData::LIST(vec![PacketData::INT(2)]);
        packets.push(&cp1);

        let cp2 = PacketData::LIST(vec![PacketData::INT(6)]);
        packets.push(&cp2);

        packets.sort_by(|left, right| match left.compare(right) {
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
            None => Equal
        } );
        let controlPacketIndex1 = packets.iter()
            .position(|packet| match cp1.compare(packet) {
                None => true,
                Some(_) => false
            } );

        let controlPacketIndex2 = packets.iter()
            .position(|packet| match cp2.compare(packet) {
                None => true,
                Some(_) => false
            });
        return match (controlPacketIndex1, controlPacketIndex2) {
            (Some(x), Some(y)) => ((x + 1) * (y + 1)) as u32,
            (_, _) => panic!("Could not find controlpackets!!")
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "[1,1,3,1,1]
                [1,1,5,1,1]

                [[1],[2,3,4]]
                [[1],4]

                [9]
                [[8,7,6]]

                [[4,4],4,4]
                [[4,4],4,4,4]

                [7,7,7,7]
                [7,7,7]

                []
                [3]

                [[[]]]
                [[]]

                [1,[2,[3,[4,[5,6,7]]]],8,9]
                [1,[2,[3,[4,[5,6,0]]]],8,9]";
        let actual_res = DayThirteen::from(input.to_string()).part_one();
        assert_eq!(13, actual_res);
    }

    #[test]
    fn partTwoExampleTest() {
        let input = "[1,1,3,1,1]
                [1,1,5,1,1]

                [[1],[2,3,4]]
                [[1],4]

                [9]
                [[8,7,6]]

                [[4,4],4,4]
                [[4,4],4,4,4]

                [7,7,7,7]
                [7,7,7]

                []
                [3]

                [[[]]]
                [[]]

                [1,[2,[3,[4,[5,6,7]]]],8,9]
                [1,[2,[3,[4,[5,6,0]]]],8,9]";
        let actual_res = DayThirteen::from(input.to_string()).part_two();
        assert_eq!(140, actual_res);
    }
}
