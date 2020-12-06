#![allow(non_snake_case)]

use crate::core::AdventOfCodeProblem;
use std::collections::HashSet;

pub struct DaySix {
    input: String
}

impl DaySix {
    pub fn new(input: &str) -> DaySix {
        return DaySix {
            input: input.to_string()
        }
    }
}

impl AdventOfCodeProblem for DaySix {
    fn partOne(&self) -> u32 {
        let groups: Vec<String> = self.input.split("\n")
            .map(|line| line.trim())
            .fold(Vec::new(), |mut acc, line| {
                println!("line {}", line);
                if line.is_empty() {
                    println!("empty line, new group");
                    acc.push("".to_string());
                } else {
                    let group = match acc.pop() {
                        Some(g) => g,
                        None => "".to_string(),
                    };
                    let newGroup = format!("{}{}\n", group, line);
                    println!("group: {}", newGroup);
                    acc.push(newGroup);    
                }
                return acc;
            });
        let answerCount: u32 = groups.iter()
            .map(|group| answeredQuestions(group))
            .map(|answers| answers.len() as u32)
            .sum();
        println!("{:?}", answerCount);
        return answerCount;
    }

    fn partTwo(&self) -> u32 {
        return 0;
    }
}

fn answeredQuestions(group: &str) -> HashSet<String> {
    return group.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_string())
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        const INPUT: &str = "abc
        
        a
b
c
        
ab
ac
        
a
a
a
a
        
b" ;
        let result = DaySix::new(INPUT).partOne();
        assert_eq!(result, 11);
    }
}