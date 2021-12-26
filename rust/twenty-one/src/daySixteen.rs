
use std::collections::VecDeque;

#[derive(Debug)]
enum Packet {
    LITERAL(Literal),
    OP_MODE_0(Operator),
    OP_MODE_1(Operator),
}

type PacketErr = &'static str;

fn parse_literal_value(bits: &mut VecDeque<char>) -> Result<u64, std::num::ParseIntError> {
    let mut value = String::new();
    while true {
        let mut valueBits = bits.drain(0..5);
        let controlBit = valueBits.next();
        valueBits.for_each(|b| value.push(b));
        match controlBit {
            None => break,
            Some(b) if b == '0' => break,
            _ => ()
        }
    }
    //println!("literal value bits {}", value);
    return u64::from_str_radix(&value, 2);
}

fn packet_parser(input: &mut VecDeque<char>) -> Result<Packet, PacketErr> {
    //println!("input {:?}", input);
    let mut version = String::new();
    for _i in 0..3 {
        let bit = input.pop_front();
        match bit {
            Some(c) if c == '0' || c == '1' => version.push(c),
            _ => return Err("Unable to parse Packet, version must be 3 bits long")
        }
    }
    let mut typeId = String::new();
    for _i in 0..3 {
        match input.pop_front() {
            Some(c) if c == '0' || c == '1' => typeId.push(c),
            _ => return Err("Unable to parse Packet, type ID must be 3 bits long")
        }
    }
    //println!("version bits {:?} typeID bits: {:?}", version, typeId);
    let version = u8::from_str_radix(&version, 2).unwrap();
    let typeId = u8::from_str_radix(&typeId, 2).unwrap();
    if typeId == 4 {
        return Ok(Packet::LITERAL(Literal {
            version: version,
            typeId: typeId, 
            value: parse_literal_value(input).unwrap()
        }));
    }
    
    let mode = match input.pop_front() {
        Some('0') => OperatorMode::BitLength,
        Some('1') => OperatorMode::SubpacketLength,
        _ => return Err("Unable to parse Packet, OperatorMode must be a bit")
    };
    let mut length_bits = Vec::new();
    for _i in 0..mode.getLength() {
        match input.pop_front() {
            Some(c) if c == '0' || c == '1' => length_bits.push(c),
            _ => return Err("Unable to parse Packet, nr of length bits is too small")
        }
    }
    let length = usize::from_str_radix(&length_bits.iter().collect::<String>(), 2).unwrap();

    // while length is not exhausted, keep parsing
    let mut subPackets = Vec::new();
    let mut keepParsing = true;
    let startCount = input.len();
    while keepParsing {
        let packet = packet_parser(input).unwrap();
        //println!("subpacket {:?}", packet);
        subPackets.push(packet);
        keepParsing = match mode {
            OperatorMode::SubpacketLength => subPackets.len() < length,
            OperatorMode::BitLength => startCount - input.len() < length
        }
    }

    return Ok(Packet::OP_MODE_0(Operator {
        mode: mode,
        version: version,
        typeId: typeId,
        length: length,
        packets: subPackets
     }));
}

fn from_str<'a>(input: &str) -> Result<Packet, PacketErr> {
    //println!("{}", input);
    return packet_parser(&mut input.chars().collect::<VecDeque<char>>());
}

#[derive(Debug)]
struct Literal {
    version: u8,
    typeId: u8,
    value: u64
}

#[derive(Debug)]
enum OperatorMode {
    SubpacketLength,
    BitLength
}

impl OperatorMode {
    fn getLength(&self) -> usize {
        return match self {
            OperatorMode::SubpacketLength => 11,
            OperatorMode::BitLength => 15
        }
    }
}

#[derive(Debug)]
struct Operator {
    version: u8,
    typeId: u8,
    length: usize,
    packets: Vec<Packet>,
    mode: OperatorMode
}

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

fn version_sum(packet: &Packet) -> u32 {
    return match packet {
        Packet::LITERAL(literal) => literal.version as u32,
        Packet::OP_MODE_0(op) => (op.version as u32) + op.packets.iter().map(|p| version_sum(p)).sum::<u32>(),
        Packet::OP_MODE_1(op) => op.version as u32 + op.packets.iter().map(|p| version_sum(p)).sum::<u32>()
    };
}

pub fn partOne(input: &str) -> u32 {
    let binary: String = input.lines()
            .map(|hex_str| hex_to_binary(hex_str))
            .collect();

    let packet = from_str(&binary).unwrap();
    return version_sum(&packet);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneLiteralExampleTest() {
        let INPUT = hex_to_binary("D2FE28");
        let result = match from_str(&INPUT).unwrap() {
            Packet::LITERAL(literal) => Some(literal),
            _ => None
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
            Packet::OP_MODE_0(op) => Some(op),
            Packet::OP_MODE_1(op) => Some(op),
        };
        let operator = result.unwrap();
        assert_eq!(1, operator.version);
        assert_eq!(6, operator.typeId);
        assert_eq!(27, operator.length);
    }

    #[test]
    fn partOneExampleOneTest() {
        let INPUT1: &str = "8A004A801A8002F478";
        let result = partOne(INPUT1);
        assert_eq!(16, result);

        let INPUT1: &str = "620080001611562C8802118E34";
        let result = partOne(INPUT1);
        assert_eq!(12, result);

        let INPUT1: &str = "C0015000016115A2E0802F182340";
        let result = partOne(INPUT1);
        assert_eq!(23, result);

        let INPUT1: &str = "A0016C880162017C3686B18A3D4780";
        let result = partOne(INPUT1);
        assert_eq!(31, result);
    }
}