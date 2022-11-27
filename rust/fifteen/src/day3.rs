use crate::core::Day;
use std::collections::HashSet;

pub struct DayThree {
    input: String
}

// this implementation should/could be a derive-macro
impl From<String> for DayThree {
    fn from(input: String) -> Self {
        DayThree { input }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position(isize, isize);

impl Position {
    fn movement(&self, movement: Position) -> Self {
        Position(self.0 + movement.0, self.1 + movement.1)
    }
}

impl DayThree {
    fn visit(&self, filter: &dyn Fn(&(usize, char)) -> bool) -> HashSet<Position> {
        let mut pos = Position(0,0);
        let mut visited = HashSet::new();
        visited.insert(pos);
        for (_i, c) in self.input.chars().enumerate().filter(filter) {
            let next_move = match c {
                '^' => Position(0, 1),
                'v' => Position(0, -1),
                '>' => Position(1, 0),
                '<' => Position(-1, 0),
                _ => panic!("Unkown move: '{}'", c)
            };
            pos = pos.movement(next_move);
            visited.insert(pos);
        }
        return visited;
    }
}

impl Day for DayThree {
    type R = u32;

    fn day() -> String where Self: Sized {
        String::from("3")
    }

    fn part_one(&self) -> Self::R {
        return self.visit(&|(_i,_c)| true).len() as u32;
    }



    fn part_two(&self) -> Self::R {
        let santa_visits = self.visit(&|(i, _c)| i % 2 > 0);
        let robo_visits = self.visit(&|(i, _c)| i % 2 == 0);
        return santa_visits.union(&robo_visits).count() as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        vec![(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)].iter()
            .map(|(l, expected)| (DayThree::from(String::from(*l)).part_one(), expected))
            .for_each(|(actual, expected)| assert_eq!(actual, *expected as u32));
    }

    #[test]
    fn partTwoExampleTest() {
        vec![("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)].iter()
            .map(|(l, expected)| (DayThree::from(String::from(*l)).part_two(), expected))
            .for_each(|(actual, expected)| assert_eq!(actual, *expected as u32));
    }
}