#![allow(non_snake_case)]

use crate::core::*;
use std::str::FromStr;

pub struct Day {
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
        let result: u64 = self.input.iter()
            .map(|line| tokenize(line))
            .map(|tokens| parse(&tokens[..], &equalPrecedent))
            .sum();
        return result;
    }

    fn partTwo(&self) -> u64 {
        let result: u64 = self.input.iter()
            .map(|line| tokenize(line))
            .map(|tokens| parse(&tokens[..], &addPrecedent))
            .sum();
        return result;
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Token {
    INT(u64),
    OP(char),
    START_PAR,
    END_PAR
}

fn parse(tokens: &[Token], comparator: &dyn Fn(&Token, &Token) -> bool) -> u64 {
    let mut stack: Vec<Token> = Vec::new();
    let mut res: Vec<Token> = Vec::new();
    for t in tokens {
        match t {
            Token::INT(_) => res.push(*t),
            Token::OP(_) => {
                while match stack.last() {
                    None => false,
                    Some(t2) if comparator(t, t2) => true,
                    Some(_) => false
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
    }
    while match stack.last() {
        None => false,
        Some(Token::START_PAR) => false,
        _ => true
    } {
        res.push(stack.pop().unwrap())
    }
    return eval(&res).unwrap();
}

impl Token {
    fn precedent(&self) -> u8 {
        return match self {
            Token::START_PAR => 0,
            Token::OP(_) => 1,
            _ => 0
        };
    }

    fn precedentPartTwo(&self) -> u8 {
        return match self {
            Token::START_PAR => 0,
            Token::OP('+') => 2,
            Token::OP('*') => 1,
            _ => 0
        };
    }
}

fn equalPrecedent(t: &Token, t2: &Token) -> bool {
    return t.precedent() <= t2.precedent();
}

fn addPrecedent(t: &Token, t2: &Token) -> bool {
    return t.precedentPartTwo() <= t2.precedentPartTwo();
}

fn eval(postFixExpr: &Vec<Token>) -> Option<u64> {
    let mut expression = postFixExpr.into_iter().rev().collect::<Vec<&Token>>();
    let mut token = expression.pop();
    let mut evalStack: Vec<u64> = Vec::new();
    while let Some(t) = token {
        match t {
            Token::INT(x) => evalStack.push(*x),
            Token::OP('+') => {
                let lhs = evalStack.pop().unwrap();
                let rhs = evalStack.pop().unwrap();
                evalStack.push(lhs + rhs);
            },
            Token::OP('*') => {
                let lhs = evalStack.pop().unwrap();
                let rhs = evalStack.pop().unwrap();
                evalStack.push(lhs * rhs);
            },
            Token::START_PAR => {

            }
            _ => println!("not implemented")
        }
        token = expression.pop();
    }
    return evalStack.pop();
}

fn tokenize(inputStr: &str) -> Vec<Token> {
    let mut input = inputStr.chars();
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

    #[test]
    fn eighteenPartOneExampleTest2() {
        const INPUT: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let result = INPUT.parse::<Day>().unwrap().partOne();
        assert_eq!(result, 437);
    }

    #[test]
    fn eighteenPartOneExampleTest3() {
        const INPUT: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let result = INPUT.parse::<Day>().unwrap().partOne();
        assert_eq!(result, 13632);
    }

    #[test]
    fn eighteenPartTwoExampleTest() {
        const INPUT: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let result = INPUT.parse::<Day>().unwrap().partTwo();
        assert_eq!(result, 23340);
    }

    #[test]
    fn eighteenPartTwoSimpleExampleTest() {
        const INPUT: &str = "1 + 2 * 3 + 4 * 5 + 6";
        let result = INPUT.parse::<Day>().unwrap().partTwo();
        assert_eq!(result, 231);
    }

    #[test]
    fn eighteenPartTwoParTestExampleTest() {
        const INPUT: &str = "1 + (2 * 3) + (4 * (5 + 6))";
        let result = INPUT.parse::<Day>().unwrap().partTwo();
        assert_eq!(result, 51);
    }

    #[test]
    fn eighteenPartTwoExampleTest2() {
        const INPUT: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let result = INPUT.parse::<Day>().unwrap().partTwo();
        assert_eq!(result, 1445);
    }

}