
use std::str::FromStr;

enum Packet {
    LITERAL(Literal),
    OP(Operator)
}

type PacketErr = &'static str;

fn from_str<'a>(input: &str) -> Result<Packet, PacketErr> {
    println!("{}", input);
    let mut bits = input.chars();
    let mut version = String::new();
    for _i in 0..3 {
        match bits.next() {
            Some(c) if c == '0' || c == '1' => version.push(c),
            _ => return Err("Unable to parse Packet, version must be 3 bits")
        }
    }
    let mut typeId = String::new();
    for _i in 0..3 {
        match bits.next() {
            Some(c) if c == '0' || c == '1' => typeId.push(c),
            _ => return Err("Unable to parse Packet, type ID must be 3 bits")
        }
    }
    let version = u8::from_str_radix(&version, 2).unwrap();
    let typeId = u8::from_str_radix(&typeId, 2).unwrap();
    if typeId == 4 {
        let value: String = bits.collect::<Vec<char>>().chunks(5)
            .map_while(|bit_group| if bit_group.len() == 5 {
                Some(bit_group.iter().skip(1).collect::<String>())
                } else { None }
            ).collect();
        println!("{}", value);
        return Ok(Packet::LITERAL(Literal {
            version: version,
            typeId: typeId, 
            value: u32::from_str_radix(&value, 2).unwrap() 
        }));
    }
    
    let mode = match bits.next() {
        Some('0') => OperatorMode::FIFTEEN,
        Some('1') => OperatorMode::ELEVEN,
        _ => return Err("Unable to parse Packet, OperatorMode must be a bit")
    };
    let mut length_bits = Vec::new();
    for _i in 0..mode.getLength() {
        match bits.next() {
            Some(c) if c == '0' || c == '1' => length_bits.push(c),
            _ => return Err("Unable to parse Packet, nr of length bits is too small")
        }
    }
    let length = usize::from_str_radix(&length_bits.iter().collect::<String>(), 2).unwrap();
    let subPackets = bits.take(length).collect();
    return Ok(Packet::OP(Operator {
        mode: mode,
        version: version,
        typeId: typeId,
        length: length,
        packets: subPackets
     }));
}

#[derive(Debug)]
struct Literal {
    version: u8,
    typeId: u8,
    value: u32
}

// impl Packet for Literal {
//     fn version(&self) -> u8 {
//         return self.version;
//     }

//     fn typeId(&self) -> u8 {
//         return self.typeId;
//     }
// }

#[derive(Debug)]
enum OperatorMode {
    ELEVEN,
    FIFTEEN
}

impl OperatorMode {
    fn getLength(&self) -> usize {
        return match self {
            OperatorMode::ELEVEN => 11,
            OperatorMode::FIFTEEN => 15
        }
    }
}

struct Operator {
    version: u8,
    typeId: u8,
    length: usize,
    packets: String,
    mode: OperatorMode
}

// impl Packet for Operator<'_> {
//     fn version(&self) -> u8 {
//         return self.version;
//     }

//     fn typeId(&self) -> u8 {
//         return self.typeId;
//     }
// }

fn hex_to_binary(hex: &str) -> String {
    return hex.chars()
        .filter_map(|c| match c {
            '0' => Some("0000"),
            '1' => Some("0001"),
            '2' => Some("0010"),
            '3' => Some("0011"),
            '4' => Some("0100"),
            '5' => Some("0101"),
            '6' => Some("0110"),
            '7' => Some("0111"),
            '8' => Some("1000"),
            '9' => Some("1001"),
            'A' => Some("1010"),
            'B' => Some("1011"),
            'C' => Some("1100"),
            'D' => Some("1101"),
            'E' => Some("1110"),
            'F' => Some("1111"),
            _ => None
        }
    ).collect::<String>();
}

pub fn partOne(input: &str) -> u32 {
    input.lines()
        .map(|hex_str| hex_to_binary(hex_str)
        ).for_each(|packet| println!("{}", packet));
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneLiteralExampleTest() {
        let INPUT = hex_to_binary("D2FE28");
        let result = match from_str(&INPUT).unwrap() {
            Packet::LITERAL(literal) => Some(literal),
            Packet::OP(_) => None
        };
        let literal = result.unwrap();
        assert_eq!(6, literal.version);
        assert_eq!(4, literal.typeId);
        assert_eq!(2021, literal.value);

    }

    #[test]
    fn partOneOperatorExampleTest() {
        let INPUT = hex_to_binary("38006F45291200");
        let result = match from_str(&INPUT).unwrap() {
            Packet::LITERAL(_) => None,
            Packet::OP(op) => Some(op)
        };
        let operator = result.unwrap();
        assert_eq!(1, operator.version);
        assert_eq!(6, operator.typeId);
        assert_eq!(27, operator.length);

    }
}