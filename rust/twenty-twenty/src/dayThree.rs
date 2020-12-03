#![allow(non_snake_case)]

use std::collections::HashMap;

pub fn partOne(input: &str) -> u32 {
    let treeMap: Vec<HashMap<usize, u32>> = input.split("\n")
        .map(|line| line.trim())
        .map(|line| parseLine(line))
        .collect();
    //println!("{:?}", treeMap);
    let length = input.split("\n").next().unwrap().chars().count();
    let mut pos: usize = 0;
    let mut sum: u32 = 0;
    for row in treeMap {
        sum = sum + match row.get(&pos) {
            Some(tree) => tree,
            None => &0
        };
        if pos + 3 >= length {
            pos = (pos + 3) - length;
        } else {
            pos = pos + 3;
        }
        //println!("pos {}", pos);
    }
    return sum;
}

fn parseLine(line: &str) -> HashMap<usize, u32> {
    let mut trees = HashMap::new();
    for (i, c) in line.chars().enumerate() {
        if c == '#' {
            trees.insert(i, 1);
        }
    }
    return trees;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exampleTest() {
        const INPUT: &str = 
        "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#" ;
        let result = partOne(INPUT);
        assert_eq!(result, 7);
    }
}