#![allow(non_snake_case)]
extern crate regex;

use crate::core::*;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

pub struct DaySeven {
    bagGraph: HashMap<String, Vec<(u32, String)>>,
}


impl DaySeven {
    pub fn new() -> DaySeven {
        return DaySeven {
            bagGraph: constructGraph(&loadInput("Seven"))
        }
    }

    pub fn test(input: &str) -> DaySeven {
        return DaySeven {
            bagGraph: constructGraph(input)
        }
    }

    fn depthFirstSearch(&self, target: &str) -> HashSet<String> {
        let mut reachesTarget: HashSet<String> = HashSet::new();
        for (node, _) in &self.bagGraph {
            //println!("node {}", node);
            if target == node {
                continue;
            }
            let mut visited: HashSet<(u32, String)> = HashSet::new();
            self.dfs(&(0, node.to_string()), &mut visited);
            if visited.iter().any(|(i, child)| child == target) {
                reachesTarget.insert(String::from(node));
            }
        }
        //println!("{:?}", reachesTarget);
        return reachesTarget;
    }

    fn dfs(&self, (qty, source): &(u32, String), visited: &mut HashSet<(u32, String)>) {
        let children = self.bagGraph.get(source).unwrap();
        visited.insert((*qty, source.to_string()));
        children.iter()
            .for_each(|c| self.dfs(c, visited));
    }

    fn dfsPartTwo(&self, (qty, source): &(u32, String)) -> u32 {
        let children = self.bagGraph.get(source).unwrap();
        if children.len() < 1 {
            println!("found leaf {} {}", qty, source);
            return 1;
        }
        let childSum: u32 = children.into_iter()
            .map(|(qty, c)| {
                println!("{} contains {} * {}", source, qty, c);
                let res = qty * self.dfsPartTwo(&(*qty, c.to_string()));
                println!("{} * {} = {}, + qty = {}", qty, c, res, (if res != *qty {res + qty} else {res}));
                return if res != *qty { res + qty } else { res };
            }).sum();
        return childSum;
    }
}

impl AdventOfCodeSolver for DaySeven {
    fn day(&self) -> &str {
        return "Seven";
    }

    fn partOne(&self) -> u32 {
        let target = "shiny gold";
        return self.depthFirstSearch(target).len() as u32;
    }

    fn partTwo(&self) -> u32 {
        
        return self.dfsPartTwo(&(1, "shiny gold".to_string()));
        
    }
}

fn constructGraph(input: &str) -> HashMap<String, Vec<(u32, String)>> {
    let bag = Regex::new(r" bag[s]?").unwrap();
    return bag.replace_all(input, "")
        .split("\n")
        .map(|line| {
            let n: Vec<_> = line.split("contain").collect();
            return (match n.get(0) {
                    Some(s) => String::from(s.trim()),
                    None => "".to_string()
                }, getChildren(n.get(1)))
        })
        .collect();
}

fn getChildren(childOpt: Option<&&str>) -> Vec<(u32, String)> {
    let childString: String = match childOpt {
        Some(s) => s.to_string(),
        None => String::from("")
    };
    let children: Vec<(u32, String)> = childString.split(",")
        .map(|child| child
                .replace(".", "")
                .replace("no other", "")
                .trim()
                .to_string())
        .filter(|child| !child.is_empty())
        .map(|c| {
            let mut tmp = c.splitn(2, " ");
            let qty: u32 = tmp.next().unwrap().parse().unwrap();
            let node = tmp.next().unwrap();
            return (qty, node.to_string());
        })
        .collect();
    return children;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        const INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags." ;
        let result = DaySeven::test(INPUT).partOne();
        assert_eq!(result, 4);
    }

    #[test]
    fn partTwoExampleTest1() {
        const INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags." ;
        let result = DaySeven::test(INPUT).partTwo();
        assert_eq!(result, 32);
    }

    #[test]
    fn partTwoExampleTest() {
        const INPUT: &str = "shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags." ;
        let result = DaySeven::test(INPUT).partTwo();
        assert_eq!(result, 126);
    }
}
