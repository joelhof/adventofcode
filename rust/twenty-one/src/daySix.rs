use std::str::FromStr;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Timer(u8);

impl Timer {
    fn next(&self) -> Timer {
        let next = match self.0 {
            0 => 6,
            _ => self.0 - 1
        };
        return Timer(next);
    }
}

#[derive(Debug)]
struct Lanterfish {
    school: HashMap<Timer, u64>
}

impl Lanterfish {
    fn nextDay(&self) -> Lanterfish {
        let newFish = self.school.get(&Timer(0));
            
        let mut next: HashMap<Timer, u64> = self.school.iter()
            .filter(|(_k, v)| *v > &0)
            .map(|(k,v)| (k.next(), *v))
            .fold(HashMap::new(), |mut nextIteration, (k,v)| {
                let fish_count = nextIteration.entry(k).or_insert(0);
                *fish_count += v;
                return nextIteration;
            });
        match newFish {
            None => None,
            Some(nr_of_new_fish) => next.insert(Timer(8), *nr_of_new_fish)
        };
        
        return Lanterfish { school: next }
    }
}

impl FromStr for Lanterfish {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let fish_population: HashMap<Timer, u64> = input.lines()
            .map(|l| l.split(",").map(|nr| nr.parse()))
            .flatten()
            .filter(|nr| nr.is_ok())
            .map(|nr| nr.unwrap())
            .map(|nr| Timer(nr))
            .fold(HashMap::new(), |mut school, fish| {
                let fish_count = school.entry(fish).or_insert(0);
                *fish_count += 1;
                return school;
            });
        return Ok(Lanterfish { school: fish_population });
    }
}

pub fn partOne(input: &str) -> u64 {
    let fish_population: Result<Lanterfish, _> = input.parse();
    //println!("{:?}", fish_population);
    if fish_population.is_ok() {
        let mut fishes = fish_population.unwrap();
        for _day in 0..80 {
            //println!("fish count: {} {:?}", fishes.school.values().sum::<u32>(), fishes);
            fishes = fishes.nextDay();
        }
        
        return fishes.school.values().sum();
    }
    return 0;
}

pub fn partTwo(input: &str) -> u64 {
    let fish_population: Result<Lanterfish, _> = input.parse();
    //println!("{:?}", fish_population);
    if fish_population.is_ok() {
        let mut fishes = fish_population.unwrap();
        for _day in 0..256 {
            //println!("fish count: {} {:?}", fishes.school.values().sum::<u32>(), fishes);
            fishes = fishes.nextDay();
        }
        
        return fishes.school.values().sum();
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExample() {
        let input = "3,4,3,1,2";
        let res = partOne(input);
        assert_eq!(5934, res);
    }

    #[test]
    fn partTwoExample() {
        let input = "3,4,3,1,2";
        let res = partTwo(input);
        assert_eq!(26984457539, res);
    }
}