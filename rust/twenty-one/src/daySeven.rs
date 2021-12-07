
pub fn partOne(input: &str) -> i32 {
    let positions: Vec<i32> = input.split(",")
        .map(|sub_str| sub_str.trim())
        .map(|nr| match nr.parse() { Err(_e) => { println!("{}", nr); -1 }, Ok(x) => x } )
        .collect();

    let max = match positions.iter().max() { Some(x) => *x, None => 0 };
    let mut min_nr_of_steps = i32::MAX;
    for i in 0..max {
        let delta: i32 = positions.iter()
            .map(|pos| pos - i)
            .map(|delta| delta.abs())
            .sum();
        
        if delta < min_nr_of_steps {
            min_nr_of_steps = delta;
        }
    }

    return min_nr_of_steps;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExample() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let res = partOne(input);
        assert_eq!(37, res);
    }
}