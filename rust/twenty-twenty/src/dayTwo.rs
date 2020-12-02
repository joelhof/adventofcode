#![allow(non_snake_case)]

pub fn solve(input: &str) -> u32 {
    return input.split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| validPassword(line))
            .sum();
}

fn validPassword(input: &str) -> u32 {
    return 0;
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