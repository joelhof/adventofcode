#![allow(non_snake_case)]

use crate::core::*;

struct Day {
    seats: Vec<Vec<u32>>,
}

impl Day {
    fn test(input: &str) -> Day {
        return Day {
            seats: Vec::new()
        }
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Eleven.txt";
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn elevenPartOneExampleTest() {
        const INPUT: &str = "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        let result = Day::test(INPUT).partOne();
        assert_eq!(result, 37);
    }
}