

pub fn partOne(input: &str) -> u32 {
    let bitSize = input.split("\n").count();
    println!("Bit size {}", input.split("\n").count());
    
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

    let mut zeros = vec!['0'; 32-columnCount];
    let mut ones = vec!['1'; columnCount];
    zeros.append(&mut ones);
    let bitmask: String = zeros.iter().collect();
    println!("{}", bitmask);
    println!("{} {}", gamma, !gamma & u32::from_str_radix(&bitmask, 2).unwrap());

    return gamma * (!gamma & u32::from_str_radix(&bitmask, 2).unwrap());
}

pub fn partTwo(input: &str) -> u32 {
    return 0;
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