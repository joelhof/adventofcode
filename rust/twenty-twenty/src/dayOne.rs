
pub fn day_one(input: &str) -> i64 {
    let parseResult: Result<Vec<i64>, _> = input.split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<i64>())
            .collect();
    let numbers = parseResult.unwrap();
    println!("{:?}", numbers);
    let sumsToTwenty: i64 = numbers.clone()
        .into_iter()
        .filter(|n| sum_vector(n, &numbers)
            .iter()
            .any(|sum| *sum == 2020))
        .fold(1, |acc, x| acc * x);
    return sumsToTwenty;
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
}
