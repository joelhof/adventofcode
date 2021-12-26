
use std::collections::VecDeque;

#[derive(Debug)]
enum Packet {
    Value(Literal),
    SumOp(Operator),
    ProductOp(Operator),
    MinOp(Operator),
    MaxOp(Operator),
    GreaterThan(Operator),
    LessThan(Operator),
    Equal(Operator)
}

type PacketErr = &'static str;

fn parse_literal_value(bits: &mut VecDeque<char>) -> Result<u64, std::num::ParseIntError> {
    let mut value = String::new();
    loop {
        let mut valueBits = bits.drain(0..5);
        let controlBit = valueBits.next();
        valueBits.for_each(|b| value.push(b));
        match controlBit {
            None => break,
            Some(b) if b == '0' => break,
            _ => ()
        }
    }
    return u64::from_str_radix(&value, 2);
}

fn packet_parser(input: &mut VecDeque<char>) -> Result<Packet, PacketErr> {
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
    let version = u8::from_str_radix(&version, 2).unwrap();
    let typeId = u8::from_str_radix(&typeId, 2).unwrap();
    if typeId == 4 {
        return Ok(Packet::Value(Literal {
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

    let mut subPackets = Vec::new();
    let mut keepParsing = true;
    let startCount = input.len();
    while keepParsing {
        let packet = packet_parser(input).unwrap();
        subPackets.push(packet);
        keepParsing = match mode {
            OperatorMode::SubpacketLength => subPackets.len() < length,
            OperatorMode::BitLength => startCount - input.len() < length
        }
    }
    let op = Operator {
        mode: mode,
        version: version,
        typeId: typeId,
        length: length,
        packets: subPackets
     };
    return match typeId {
        0 => Ok(Packet::SumOp(op)),
        1 => Ok(Packet::ProductOp(op)),
        2 => Ok(Packet::MinOp(op)),
        3 => Ok(Packet::MaxOp(op)),
        5 => Ok(Packet::GreaterThan(op)),
        6 => Ok(Packet::LessThan(op)),
        7 => Ok(Packet::Equal(op)),
        _ => Err("Unsupported Type Id")
    }
}

fn from_str<'a>(input: &str) -> Result<Packet, PacketErr> {
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
        Packet::Value(literal) => literal.version as u32,
        Packet::SumOp(op) => (op.version as u32) + op.packets.iter().map(|p| version_sum(p)).sum::<u32>(),
        Packet::ProductOp(op) => op.version as u32 + op.packets.iter().map(|p| version_sum(p)).sum::<u32>(),
        Packet::MinOp(op) => op.version as u32 + op.packets.iter().map(|p| version_sum(p)).sum::<u32>(),
        Packet::MaxOp(op) => op.version as u32 + op.packets.iter().map(|p| version_sum(p)).sum::<u32>(),
        Packet::GreaterThan(op) => op.version as u32 + op.packets.iter().map(|p| version_sum(p)).sum::<u32>(),
        Packet::LessThan(op) => op.version as u32 + op.packets.iter().map(|p| version_sum(p)).sum::<u32>(),
        Packet::Equal(op) => op.version as u32 + op.packets.iter().map(|p| version_sum(p)).sum::<u32>(),
    };
}

pub fn partOne(input: &str) -> u32 {
    let binary: String = input.lines()
            .map(|hex_str| hex_to_binary(hex_str))
            .collect();

    let packet = from_str(&binary).unwrap();
    return version_sum(&packet);
}

pub fn partTwo(input: &str) -> u64 {
    let binary: String = input.lines()
            .map(|hex_str| hex_to_binary(hex_str))
            .collect();

    let packet = from_str(&binary).unwrap();
    return packet.value();
}

impl Packet {
    fn value(&self) -> u64 {
        return match self {
            Packet::Value(literal) => literal.value,
            Packet::SumOp(op) => op.packets.iter().map(|p| p.value()).sum::<u64>(),
            Packet::ProductOp(op) => op.packets.iter().map(|p| p.value()).product::<u64>(),
            Packet::MinOp(op) => match op.packets.iter().map(|p| p.value()).min() { Some(min) => min, None => 0 },
            Packet::MaxOp(op) => match op.packets.iter().map(|p| p.value()).max() { Some(max) => max, None => 0 },
            Packet::GreaterThan(op) => if op.packets[0].value() > op.packets[1].value() { 1 } else { 0 },
            Packet::LessThan(op) => if op.packets[0].value() < op.packets[1].value() { 1 } else { 0 },
            Packet::Equal(op) => if op.packets[0].value() == op.packets[1].value() { 1 } else { 0 },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneLiteralExampleTest() {
        let INPUT = hex_to_binary("D2FE28");
        let result = match from_str(&INPUT).unwrap() {
            Packet::Value(literal) => Some(literal),
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
            Packet::Value(_) => None,
            Packet::SumOp(op) => Some(op),
            Packet::ProductOp(op) => Some(op),
            Packet::MinOp(op) => Some(op),
            Packet::MaxOp(op) => Some(op),
            Packet::GreaterThan(op) => Some(op),
            Packet::LessThan(op) => Some(op),
            Packet::Equal(op) => Some(op),
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

    #[test]
    fn partTwoExamplesTest() {
        let INPUT1: &str = "C200B40A82";
        let result = partTwo(INPUT1);
        assert_eq!(3, result);

        let INPUT1: &str = "04005AC33890";
        let result = partTwo(INPUT1);
        assert_eq!(54, result);

        let INPUT1: &str = "880086C3E88112";
        let result = partTwo(INPUT1);
        assert_eq!(7, result);

        let INPUT1: &str = "CE00C43D881120";
        let result = partTwo(INPUT1);
        assert_eq!(9, result);

        let INPUT1: &str = "D8005AC2A8F0";
        let result = partTwo(INPUT1);
        assert_eq!(1, result);
        
        let INPUT1: &str = "F600BC2D8F";
        let result = partTwo(INPUT1);
        assert_eq!(0, result);

        let INPUT1: &str = "9C005AC2F8F0";
        let result = partTwo(INPUT1);
        assert_eq!(0, result);

        let INPUT1: &str = "9C0141080250320F1802104A08";
        let result = partTwo(INPUT1);
        assert_eq!(1, result);
    }
}