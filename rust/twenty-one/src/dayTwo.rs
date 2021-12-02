use std::convert::TryFrom;

#[derive(Debug)]
enum Instruction {
    FORWARD(i32),
    UP(i32),
    DOWN(i32)
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = input.split(" ").collect();
        return match split[0] {
            "forward" => Ok(Instruction::FORWARD(split[1].parse().unwrap())),
            "up" => Ok(Instruction::UP(split[1].parse().unwrap())),
            "down" => Ok(Instruction::DOWN(split[1].parse().unwrap())),
            _instr => Err("Unknown Instruction")
        }
    }
}


pub fn partOne(input: &str) -> i32 {
    let lines: Vec<Instruction> = input.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| Instruction::try_from(line))
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap())
        .collect();
    let forward: i32 = lines.iter()
        .filter(|instruction| match instruction {
            Instruction::FORWARD(_) => true,
            _ => false
        })
        .map(|fw| if let Instruction::FORWARD(v) = fw { v } else { &0 }).sum();    
    let depth: i32 = lines.iter()
        .map(|instruction| match instruction {
            Instruction::UP(v) => -v,
            Instruction::DOWN(v) => *v,
            _ => 0
        })
        .sum();  
    return forward * depth;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExample() {
        let example = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";
        let res = partOne(example);
        assert_eq!(150, res);
    }

}