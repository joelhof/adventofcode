#![allow(non_snake_case)]

pub fn solve(input: &str) -> u32 {
    return input.split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| validPassword(line))
            .sum();
}

fn validPassword(input: &str) -> u32 {
    let tmp: Vec<&str> = input.split(":").collect();
    let policy = Policy::new(tmp[0]);
    let password = tmp[1];
    println!("policy {:?} password {:2}", policy, password);

    return 0;
}

#[derive(Debug, Clone, PartialEq)]
struct Policy {
    min: u32,
    max: u32,
    key: String
}

impl Policy {
    pub fn new(input: &str) -> Policy {
        return Policy {
            min: input[0..1].parse().unwrap(),
            max: input[2..3].parse().unwrap(),
            key: input[4..].to_string()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exampleTest() {
        const INPUT: &str = "1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc";
        let result = solve(INPUT);
        assert_eq!(result, 2);
    }
}