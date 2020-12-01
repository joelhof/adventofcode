
pub fn day_one(input: &str) -> i64 {
    let numbers = parseInput(input);
    let sumsToTwenty: i64 = numbers.clone()
        .into_iter()
        .filter(|n| sum_vector(n, &numbers)
            .iter()
            .any(|sum| *sum == 2020))
        .fold(1, |acc, x| acc * x);
    return sumsToTwenty;
}

pub fn dayOnePartTwo(input: &str) -> i64 {
    let numbers = parseInput(input);
    let mut product = 0;
    for i in &numbers {
        for j in &numbers {
            for k in &numbers {
                let sum = i + j + k;
                if sum == 2020 {
                    product = i * j * k;
                }
            }
        }
    }
    return product;
} 

fn parseInput(input: &str) -> Vec<i64> {
    let parseResult: Result<Vec<i64>, _> = input.split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<i64>())
            .collect();
    return parseResult.unwrap();
}

fn sum_vector(n: &i64, numbers: &[i64]) -> Vec<i64> {
    return numbers.iter().map(|x| x + n).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        const INPUT: &str = "1721
            979
            366
            299
            675
            1456";
        let result = day_one(INPUT);
        assert_eq!(result, 514579i64);
    }

    #[test]
    fn ignoreBlankLines() {
        const INPUT: &str = "1721
            979
            366
            299

            675
            1456";
        let result = day_one(INPUT);
        assert_eq!(result, 514579i64);
    }

    #[test]
    fn findThreeNumbers() {
        const INPUT: &str = "1721
            979
            366
            299

            675
            1456";
        let result = dayOnePartTwo(INPUT);
        assert_eq!(result, 241861950);
    }
}
