use std::ops::Mul;
use crate::core::{Day};
use lazy_static::lazy_static;
use regex::Regex;

pub struct DayEleven {
   input: String
}

impl From<String> for DayEleven {
    fn from(input: String) -> Self {
        DayEleven { input }
    }
}

lazy_static! {
    static ref MONKEY_ID_RE: Regex = Regex::new(r"Monkey (\d+):").unwrap();
    static ref FIELD_VALUE_RE: Regex = Regex::new(r"(?P<field>\w+):(?P<value>\w+)").unwrap();
    static ref STARTING_ITEMS: Regex = Regex::new(r"Starting items:([\d\+, ]+)").unwrap();
    static ref MONKEY_FRIENDS: Regex = Regex::new(r"throw to monkey([\d\+, ]+)").unwrap();
    static ref TEST_RE: Regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
    static ref OPERATION_RE: Regex = Regex::new(r"Operation.+(old|\d+) ([\+\*-]) (old|\d+)").unwrap();
}

impl Day for DayEleven {
    type R = u64;

    fn day() -> String where Self: Sized {
        "11".to_string()
    }

    fn part_one(&self) -> Self::R {
        let mut monkeys: Vec<Monkey> = self.parseMonkeys();
        let length = monkeys.len();
        //monkeys.iter().for_each(|monkey| println!("{:?}", monkey));

        for _round in 0..20 {
            for i in 0..length {//monkeys.iter_mut().enumerate() {
                let inspected = monkeys[i].inspectPartOne();
                for item in inspected {
                    let receiver = monkeys[i].getReceiver(&item);
                    if i == receiver {
                        continue
                    }
                    match monkeys.get_mut(receiver) {
                        Some(m) => m.receive(item.clone()),
                        None => panic!("Receiving monkey not found!")
                    }
                }
            }
        }

       return Self::amount_of_monkey_business(&mut monkeys);

    }

    fn part_two(&self) -> Self::R {
        let mut monkeys: Vec<Monkey> = self.parseMonkeys();
        let length = monkeys.len();
        let gcd: u64 = monkeys.iter().map(|m| &(m.divisor)).product();

        for _round in 0..10000 {
            for i in 0..length {
                let inspected = monkeys[i].inspectPartTwo(&gcd);
                for item in inspected {
                    let receiver = monkeys[i].getReceiver(&item);
                    if i == receiver {
                        continue
                    }
                    match monkeys.get_mut(receiver) {
                        Some(m) => m.receive(item),
                        None => panic!("Receiving monkey not found!")
                    }
                }
            }
        }
        return Self::amount_of_monkey_business(&mut monkeys);
    }
}

impl DayEleven {
    fn parseMonkeys(&self) -> Vec<Monkey> {
        MONKEY_ID_RE.split(&self.input)
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .enumerate()
            .map(|(_i, monkey)| Monkey::from(monkey))
            .collect()
    }

    fn amount_of_monkey_business(monkeys: &Vec<Monkey>) -> u64 {
        let mut monkey_business = monkeys.iter()
            .map(|m| m.nr_of_inspections as <DayEleven as Day>::R)
            .collect::<Vec<<DayEleven as Day>::R>>();
        monkey_business.sort();
        monkey_business.reverse();
        monkey_business[0..2].iter().product()
    }
}

#[derive(Debug)]
struct Operation {
    op: char,
    lhs: String,
    rhs: String
}

impl From<&str> for Operation {
    fn from(input: &str) -> Self {
        let cap = OPERATION_RE.captures(input).unwrap();
        Operation{
            op: (&cap[2]).parse().unwrap(),
            lhs: (&cap[1]).to_string(),
            rhs: (&cap[3]).to_string()
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    divisor: u64,
    friends: Vec<usize>,
    nr_of_inspections: usize
}

impl Monkey {
    fn inspectPartOne(&mut self) -> Vec<u64> {
        let inspected: Vec<u64> = self.items.iter()
            .map(|item| self.getWorryLevel(item))
            .map(|item| item / u64::from(3u8)).collect();
        self.items = vec![];
        self.nr_of_inspections = self.nr_of_inspections + inspected.len();
        inspected
    }

    fn inspectPartTwo(&mut self, modulo: &u64) -> Vec<u64> {
        let inspected: Vec<u64> = self.items.iter()
            .map(|item| self.getWorryLevel(item))
            .map(|item| item % modulo)
            .collect();
        self.items = vec![];
        self.nr_of_inspections = self.nr_of_inspections + inspected.len();
        inspected
    }

    fn getWorryLevel(&self, item: &u64) -> u64 {
        let lhs: u64 = match (&self).op.lhs.as_str() {
            "old" => item.clone(),
            lhs => lhs.parse::<u64>().unwrap()
        };

        let rhs: u64= match (&self).op.rhs.as_str() {
            "old" => item.clone(),
            lhs => lhs.parse::<u64>().unwrap()
        };
        match (&self).op.op {
            '*' => lhs.mul(rhs),
            '+' => lhs + rhs,
            '-' => lhs - rhs,
            _ => u64::from(0u8)
        }
    }

    fn getReceiver(&self, item: &u64) -> usize {
        return if item % &self.divisor == u64::from(0u8) {
            self.friends[0]
        } else {
            self.friends[1]
        }
    }

    fn receive(&mut self, item: u64) {
        self.items.push(item);
    }
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {

        let startingItems: Vec<u64> = match STARTING_ITEMS.captures(input) {
             Some(caps) => (caps[1]).split(",").into_iter()
                 .map(|d| d.trim())
                 .map(|s| s.parse().unwrap())
                 .collect(),
             None => vec![]
        };
        let friends: Vec<usize> = MONKEY_FRIENDS.captures_iter(input)
            .map(|caps| caps[1].trim().parse().unwrap())
            .collect();
        let divider = TEST_RE.captures_iter(input)
            .map(|c| c[1].trim().parse().unwrap())
            .collect::<Vec<u64>>();
        let divisor = match divider.first() {
            Some(d) => d.clone(),
            None => u64::from(1u8)
        };
        let op = Operation::from(input);
        Monkey {
            items: startingItems,
            op,
            divisor,
            friends,
            nr_of_inspections: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3

            Monkey 1:
              Starting items: 54, 65, 75, 74
              Operation: new = old + 6
              Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0

            Monkey 2:
              Starting items: 79, 60, 97
              Operation: new = old * old
              Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3

            Monkey 3:
              Starting items: 74
              Operation: new = old + 3
              Test: divisible by 17
                If true: throw to monkey 0
                If false: throw to monkey 1";
        let actual_res = DayEleven::from(input.to_string()).part_one();
        assert_eq!(10605, actual_res);
    }

    #[test]
    fn partTwoExampleTest() {
        let input = "Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3

            Monkey 1:
              Starting items: 54, 65, 75, 74
              Operation: new = old + 6
              Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0

            Monkey 2:
              Starting items: 79, 60, 97
              Operation: new = old * old
              Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3

            Monkey 3:
              Starting items: 74
              Operation: new = old + 3
              Test: divisible by 17
                If true: throw to monkey 0
                If false: throw to monkey 1";
        let actual_res = DayEleven::from(input.to_string()).part_two();
        assert_eq!(2713310158, actual_res);
    }
}