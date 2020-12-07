#![allow(non_snake_case)]
extern crate regex;

use crate::core::*;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

pub struct DaySeven {
    bagGraph: HashMap<String, Vec<String>>,

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
            let mut visited: HashSet<String> = HashSet::new();
            self.dfs(node, target, &mut visited);
            if visited.iter().any(|child| child == target) {
                reachesTarget.insert(String::from(node));
            }
        }
        //println!("{:?}", reachesTarget);
        return reachesTarget;
    }

    fn dfs(&self, source: &str, target: &str, visited: &mut HashSet<String>) {
        let children = self.bagGraph.get(source).unwrap();
        visited.insert(source.to_string());
        children.iter()
            .for_each(|c| self.dfs(c, target, visited));
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
        return 0;
    }
}

fn constructGraph(input: &str) -> HashMap<String, Vec<String>> {
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

fn getChildren(childOpt: Option<&&str>) -> Vec<String> {
    let childString: String = match childOpt {
        Some(s) => s.to_string(),
        None => String::from("")
    };
    let numbers = Regex::new(r"[0-9]").unwrap();
    let children: Vec<String> = childString.split(",")
        .map(|child| numbers.replace_all(child, "")
                .replace(".", "")
                .replace("no other", "")
                .trim()
                .to_string())
        .filter(|child| !child.is_empty())
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
}
