
pub fn day_one(input: &str) -> i64 {
    return -1i64;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        const INPUT: &str = "
            1721
            979
            366
            299
            675
            1456";
        let result = day_one(INPUT);
        assert_eq!(result, 514579i64);
    }
}
