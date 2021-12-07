use std::str::FromStr;

struct Crabs {
    positions: Vec<i32>
}

impl FromStr for Crabs {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let positions: Vec<i32> = input.split(",")
            .map(|sub_str| sub_str.trim())
            .map(|nr| match nr.parse() { Err(_e) => { println!("{}", nr); -1 }, Ok(x) => x } )
            .collect();
        return Ok(Crabs { positions });
    }
}

pub fn partOne(input: &str) -> i32 {
   
    let crabs: Crabs = input.parse().unwrap();
    let max = match crabs.positions.iter().max() { Some(x) => *x, None => 0 };
    let mut min_nr_of_steps = i32::MAX;
    for i in 0..max {
        let delta: i32 = crabs.positions.iter()
            .map(|pos| pos - i)
            .map(|delta| delta.abs())
            .sum();
        
        if delta < min_nr_of_steps {
            min_nr_of_steps = delta;
        }
    }

    return min_nr_of_steps;
}

pub fn partTwo(input: &str) -> i32 {
    let crabs: Crabs = input.parse().unwrap();
    let max = match crabs.positions.iter().max() { Some(x) => *x, None => 0 };
    let mut min_nr_of_steps = i32::MAX;

    for i in 0..max {
        let delta: i32 = crabs.positions.iter()
            .map(|pos| pos - i)
            .map(|delta| fuelCost(&delta))
            .sum();
        
        if delta < min_nr_of_steps {
            min_nr_of_steps = delta;
        }
    }

    return min_nr_of_steps;
}

fn fuelCost(distance: &i32) -> i32 {
    let delta = distance.abs();
    return delta * (1 + delta) / 2;
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
    
    #[test]
    fn partTwoExample() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let res = partTwo(input);
        assert_eq!(168, res);
    }

    #[test]
    fn fuelCostTest() {
        let input = vec![
            (16 - 5, 66),
            (1 - 5, 10),
            (7 - 5, 3)
        ];
        input.iter()
            .for_each(|(delta, expected)| assert_eq!(*expected, fuelCost(delta)));
    }
}