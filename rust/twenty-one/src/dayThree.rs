

pub fn partOne(input: &str) -> u32 {
    let bitSize = input.split("\n").count();
    println!("Bit size {}", bitSize);
    
    let columnCount = match input.split("\n").next() {
        Some(line) => line.chars().count(),
        None => 0
    };
    println!("{} {}",columnCount, (bitSize / 2));
    let mut columns: Vec<u32> = vec![0; columnCount];

    columns = input.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars())
        .fold(columns, |mut cols, chars| {
            for (i, c) in chars.enumerate() {
                cols[i] = match c {
                    '1' => cols[i] + 1,
                    _ => cols[i]
                }
            }
            return cols;
        });

    let gamma = u32::from_str_radix(
        &columns.iter()
            .map(|nr_of_ones| if *nr_of_ones > (bitSize as u32 / 2) { '1' } else { '0' })
            .fold(String::from(""), |mut nr, bit| { nr.push(bit); nr}), 2).unwrap();
   
    let b = getBitMask(columnCount);
    println!("{} {}", gamma, !gamma & b);

    return gamma * (!gamma & b);
}

fn bitCount(lines: &[&str]) -> Vec<u32> {
    let len = lines[0].len();
    //println!("nr of lines {:?} {}", lines.len(), (lines.len() as u32 / 2));
    let x = lines.iter()
    .map(|line| line.chars())
    .fold(vec![0; len], |mut cols, chars| {
        for (i, c) in chars.enumerate() {
            cols[i] = match c {
                '1' => cols[i] + 1,
                _ => cols[i]
            }
        }

        return cols;
    });
    println!("ones count {:?}", x);
    return x;
}

pub fn partTwo(input: &str) -> u32 {
    
    let numbers: Vec<&str> = input.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let oxygen_rating = recurCounterOxygen(&numbers, 0);
    //println!("-------");
    let co2_rating = recurCounterCarbonDioxide(&numbers, 0);
    //println!("{} {}", oxygen_rating, co2_rating);
    return u32::from_str_radix(&oxygen_rating, 2).unwrap() * u32::from_str_radix(&co2_rating, 2).unwrap();
}

fn recurCounterOxygen(numbers: &[&str], pos: usize) -> String {
    if numbers.len() == 1 {
        return numbers[0].to_string();
    }
    //println!("{:?}", numbers);
    
    let ones: Vec<char> = bitCount(numbers)
        .iter()
        .map(|nr_of_ones| if *nr_of_ones >= (numbers.len() as u32 - nr_of_ones) { '1' } else { '0' })
        .collect();
    //println!("pos {} {:?}", pos, ones);

    
    let next: Vec<&str> = numbers.iter()
        .filter(|nr| nr.chars().nth(pos).unwrap() == ones[pos])
        .map(|nr| *nr)
        .collect();
    return recurCounterOxygen(&next, pos + 1);
}

fn recurCounterCarbonDioxide(numbers: &[&str], pos: usize) -> String {
    if numbers.len() == 1 {
        return numbers[0].to_string();
    }
    //println!("{:?}", numbers);
    
    let ones: Vec<char> = bitCount(numbers)
        .iter()
        .map(|nr_of_ones| {
            //println!("{} - {} = {}",numbers.len(), nr_of_ones, (numbers.len() as u32 - nr_of_ones));
            if *nr_of_ones >= (numbers.len() as u32 - nr_of_ones) { '0' } else { '1' }
        })
        .collect();
    //println!("pos {} {:?}", pos, ones);

    
    let next: Vec<&str> = numbers.iter()
        .filter(|nr| nr.chars().nth(pos).unwrap() == ones[pos])
        .map(|nr| *nr)
        .collect();
    return recurCounterCarbonDioxide(&next, pos + 1);
}

fn getBitMask(bitsize: usize) -> u32 {
    let mut zeros = vec!['0'; 32-bitsize];
    let mut ones = vec!['1'; bitsize];
    zeros.append(&mut ones);
    let bitmask: String = zeros.iter().collect();
    return u32::from_str_radix(&bitmask, 2).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExample() {
        let example = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        let res = partOne(example);
        assert_eq!(198, res);
    }

    #[test]
    fn partTwoExample() {
        let example = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        let res = partTwo(example);
        assert_eq!(230, res);
    }
}