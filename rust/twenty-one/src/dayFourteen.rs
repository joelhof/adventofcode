use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct PolymerFactory {
    mapping: HashMap<String, String>,
    polymer: String
}

impl FromStr for PolymerFactory {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let template: Option<&str> = input.lines().next();
        let foldInstructionPattern: Regex = Regex::new(r"([A-Z]{2}) -> ([A-Z]{1})").unwrap();
        let caps = foldInstructionPattern.captures_iter(input);
        if template.is_none() {
            return Err("Unable to parse into polymer factory");
        }
        let mapping = caps.map(|m| (m[1].to_string(), {
            let mut s = m[1].to_string();
            s.insert_str(1, &m[2]);
            s
        })).collect::<HashMap<String, String>>();
        return Ok(PolymerFactory { polymer: template.unwrap().to_string(), mapping: mapping });
    }
}

impl PolymerFactory {
    fn polymerize(&mut self, iterations: u32) -> String {
       return (0..iterations).fold(String::from(&self.polymer), |res, _input| {
            let mut sr = String::new();
            let chars = res.chars().collect::<Vec<char>>();
            let pair_count = chars.windows(2).count() - 1;
            let pairs = chars.windows(2);
            for (i,pair) in pairs.enumerate() {
                let mut mapped = match self.mapping.get(&pair.iter().collect::<String>()) {
                    Some(s) => String::from(s),
                    None => String::new()
                };
                if i < pair_count {
                    mapped.pop();
                }
                sr.push_str(&mapped);
            }
            sr
        });
    }
}

pub fn partOne(input: &str) -> u32 {
    let mut factory: PolymerFactory = input.parse().unwrap();
    //println!("{:?}", factory);
    let polymer = factory.polymerize(10);
    let mut freq = HashMap::new();
    for small_cave in polymer.chars() {
        let f = freq.entry(small_cave).or_insert(0);
        *f += 1;
    };
    //println!("{:?}", freq);
    let mostCommon = freq.values().max();
    let leastCommon = freq.values().min();
    return mostCommon.unwrap() - leastCommon.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneSmallExample() {
        let input = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";
        let res = partOne(input);
        assert_eq!(1588, res);
    }
}