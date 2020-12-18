#![allow(non_snake_case)]

use crate::core::*;
use std::str::FromStr;
use regex::Regex;

struct Day {
    input: Vec<String>
}

impl FromStr for Day {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        return Ok(Day {
            input: input.split("\n")
                        .map(|line| line.trim())
                        .filter(|line| !line.is_empty())
                        .map(|line| line.replace(" ", "").to_string())
                        .collect()
        });
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Eighteen";
    }

    fn partOne(&self) -> u64 {
        let mut results: u64 = self.input.iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .map(|line| eval(&mut line.to_vec()))
            .sum();
        println!("{:?}", results);
        // let evals: Vec<u64> = expressions.iter()
        //     .map(|expr| eval(&mut expr.to_vec()))
        //     .collect();
        return results;
    }
}

struct Expr {
    lhs: u64,
    operator: String,
    rhs: u64
}

fn eval(expr: &mut Vec<char>) -> u64 {
    //let mut lhs: u64;
    let mut rhs: u64;
    let mut res: u64 = 0;
    let lhs = expr.pop();
    println!("{:?}", lhs);
    if lhs.is_some() {
        if char::is_numeric(lhs.unwrap()) {
            let lhsArg = lhs.unwrap().to_string().parse().unwrap();
            match expr.pop() {
                None => return lhsArg,
                Some(op) if '+' == op => return lhsArg + eval(expr),
                Some(op) if '*' == op => return lhsArg * eval(expr),
                Some(_) => return eval(expr)
            }
        } else {
            res = eval(expr);
        }
    } else {
        println!("end of line reached");
        return res;
    }
    return res;
    // for c in expr.into_iter() {
    //     match *c {
    //         "(" => ,
    //         "+" => lhs + eval(),
    //         "*" => lhs * eval(),
    //         _ => 0
    // //         c if char::is_numeric => // then c is an argument
    // //         "*" => , // then operator is '*'
    // //         "+" => 
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn eighteenPartOneSimpleExampleTest() {
        const INPUT: &str = "1 + 2 * 3 + 4 * 5 + 6";
        let result = INPUT.parse::<Day>().unwrap().partOne();
        assert_eq!(result, 71);
    }

    #[test]
    fn eighteenPartOneParTestExampleTest() {
        const INPUT: &str = "1 + (2 * 3) + (4 * (5 + 6))";
        let result = INPUT.parse::<Day>().unwrap().partOne();
        assert_eq!(result, 71);
    }
}