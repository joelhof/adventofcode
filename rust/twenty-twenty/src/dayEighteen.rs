#![allow(non_snake_case)]

use crate::core::*;
use std::str::FromStr;
use std::iter::FromIterator;
use std::collections::LinkedList;
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
        println!("{:?}", self.input);
        let mut stack: Vec<u64> = Vec::new();
        let mut result: u64 = self.input.iter()
            //.map(|line| line.chars().rev().collect::<Vec<char>>())
            .map(|line| tokenize(line))
            .map(|tokens| parse(&tokens[..]))
            .sum();
        println!("{:?}", result);

        return result;
    }
}

struct Expr {
    lhs: u64,
    operator: String,
    rhs: u64
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Token {
    INT(u64),
    OP(char),
    START_PAR,
    END_PAR
}

fn parse(tokens: &[Token]) -> u64 {
    let mut stack = Vec::new();
    println!("parse {:?}", tokens);
    let mut res: Vec<Token> = Vec::new();
    for t in tokens {
        println!("token {:?}, stack {:?}", t, stack);
        match t {
            Token::INT(_) => res.push(*t),
            Token::OP(_) => {
                while match stack.last() {
                    None => false,
                    Some(_) => true
                } {
                    res.push(stack.pop().unwrap())
                }
                stack.push(*t);
            },
            Token::START_PAR => stack.push(*t),
            Token::END_PAR => {
                while match stack.last() {
                    None => false,
                    Some(Token::START_PAR) => false,
                    _ => true
                } {
                    res.push(stack.pop().unwrap())
                }
                stack.pop();
            },
            _ => println!("unhandled")
        }
        //println!("stack {:?}", stack);
    }
    while match stack.last() {
        None => false,
        Some(Token::START_PAR) => false,
        _ => true
    } {
        res.push(stack.pop().unwrap())
    }
    let mut expression = res.into_iter().rev().collect::<Vec<Token>>();
    println!("res {:?}", expression);
    let mut token = expression.pop();
    let mut evalStack: Vec<u64> = Vec::new();
    while let Some(t) = token {
        println!("evalstack {:?}", evalStack);
        match t {
            Token::INT(x) => evalStack.push(x),
            Token::OP('+') => {
                let lhs = evalStack.pop().unwrap();
                let rhs = evalStack.pop().unwrap();
                evalStack.push(lhs + rhs);
            },
            Token::OP('*') => {
                let lhs = evalStack.pop().unwrap();
                let rhs = evalStack.pop().unwrap();
                evalStack.push(lhs * rhs);
            }
            _ => println!("not implemented")
        }
        token = expression.pop();
    }

    return evalStack.pop().unwrap();
}

fn tokenize(inputStr: &str) -> Vec<Token> {
    let mut input = inputStr.chars();
    println!("input {:?}", input);
    let mut res =  Vec::new();
    let mut current = input.next();
    while let Some(x) = current {
        match current {
            Some(c) if char::is_numeric(c) => res.push(Token::INT(c.to_digit(10).unwrap() as u64)),
            Some('+') => res.push(Token::OP('+')),
            Some('*') => res.push(Token::OP('*')),
            Some('(') => res.push(Token::START_PAR),
            Some(')') => res.push(Token::END_PAR),
            _ => panic!("Unkown character")
        };
        current = input.next();
    }
    return res;
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
        assert_eq!(result, 51);
    }
}