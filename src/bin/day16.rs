use std::fs::read_to_string;

#[derive(Debug)]
struct Header {
    version: u32,
    type_id: u32,
}

impl Header {
    const BITS: usize = 6;

    fn is_literal_value(&self) -> bool {
        self.type_id == 4
    }

    fn new(s: &str) -> Header {
        Header {
            version: bin_to_dec(&s[..3]).unwrap(),
            type_id: bin_to_dec(&s[3..6]).unwrap(),
        }
    }
}

fn value_from_groups(s: &str) -> (u64, usize) {
    let mut bits = String::new();
    let mut n = 0;

    for group in s.chars().collect::<Vec<char>>().chunks(5) {
        for &c in &group[1..] {
            bits.push(c);
        }

        n += 1;

        if group[0] == '0' {
            break;
        }
    }

    (u64::from_str_radix(&bits, 2).unwrap(), bits.len() + n)
}

#[derive(Debug)]
struct LiteralValuePacket {
    header: Header,
    value: u64,
    bits: usize,
}

impl LiteralValuePacket {
    fn new(s: &str) -> LiteralValuePacket {
        let header = Header::new(&s);
        let (value, mut bits) = value_from_groups(&s[6..]);
        bits += Header::BITS;

        LiteralValuePacket {
            header,
            value,
            bits,
        }
    }
}

#[derive(Debug)]
enum PacketLength {
    Bits(usize),
    Number(usize),
}

impl PacketLength {
    fn new(s: &str) -> PacketLength {
        if s.chars().next().unwrap() == '0' {
            PacketLength::Bits(bin_to_dec(&s[1..16]).unwrap().try_into().unwrap())
        } else {
            PacketLength::Number(bin_to_dec(&s[1..12]).unwrap().try_into().unwrap())
        }
    }

    fn bits(&self) -> usize {
        match self {
            PacketLength::Bits(_) => 16,
            PacketLength::Number(_) => 12,
        }
    }
}

#[derive(Debug)]
struct OperatorPacket {
    header: Header,
    length: PacketLength,
    subpackets: Vec<Packet>,
    bits: usize,
}

impl OperatorPacket {
    fn new(s: &str) -> OperatorPacket {
        let header = Header::new(&s);
        let length = PacketLength::new(&s[6..]);
        let mut subpackets = Vec::new();
        let mut bits = Header::BITS + length.bits();

        let mut offset = 0;
        match length {
            PacketLength::Bits(n) => {
                while offset < n {
                    let packet = Packet::new(&s[bits + offset..]);
                    offset += packet.bits();
                    subpackets.push(packet);
                }
            }
            PacketLength::Number(n) => {
                for _ in 0..n {
                    let packet = Packet::new(&s[bits + offset..]);
                    offset += packet.bits();
                    subpackets.push(packet);
                }
            }
        }
        bits += offset;

        OperatorPacket {
            header,
            length,
            subpackets,
            bits,
        }
    }
}

#[derive(Debug)]
enum Packet {
    LiteralValue(LiteralValuePacket),
    Operator(OperatorPacket),
}

impl Packet {
    fn new(s: &str) -> Packet {
        let header = Header::new(&s);

        if header.is_literal_value() {
            Packet::LiteralValue(LiteralValuePacket::new(&s))
        } else {
            Packet::Operator(OperatorPacket::new(&s))
        }
    }

    fn bits(&self) -> usize {
        match self {
            Packet::LiteralValue(packet) => packet.bits,
            Packet::Operator(packet) => packet.bits,
        }
    }

    fn sum_of_version_numbers(&self) -> u32 {
        match self {
            Packet::Operator(packet) => {
                packet.header.version
                    + packet
                        .subpackets
                        .iter()
                        .map(|p| p.sum_of_version_numbers())
                        .sum::<u32>()
            }
            Packet::LiteralValue(packet) => packet.header.version,
        }
    }

    fn value(&self) -> u64 {
        match self {
            Packet::Operator(packet) => match packet.header.type_id {
                0 => packet.subpackets.iter().map(|p| p.value()).sum(),
                1 => packet.subpackets.iter().map(|p| p.value()).product(),
                2 => packet.subpackets.iter().map(|p| p.value()).min().unwrap(),
                3 => packet.subpackets.iter().map(|p| p.value()).max().unwrap(),
                5 => (packet.subpackets[0].value() > packet.subpackets[1].value()) as u64,
                6 => (packet.subpackets[0].value() < packet.subpackets[1].value()) as u64,
                7 => (packet.subpackets[0].value() == packet.subpackets[1].value()) as u64,
                _ => unreachable!(),
            },
            Packet::LiteralValue(packet) => packet.value,
        }
    }
}

fn hex_to_bin(c: char) -> Option<String> {
    Some(format!("{:04b}", c.to_digit(16)?))
}

fn bin_to_dec(s: &str) -> Option<u32> {
    u32::from_str_radix(s, 2).ok()
}

fn binary_representation(s: &str) -> String {
    s.chars().filter_map(hex_to_bin).collect()
}

fn main() {
    let input = read_to_string("input/day16/input").unwrap();
    let packet = Packet::new(&binary_representation(&input));
    println!("{} {}", packet.sum_of_version_numbers(), packet.value());
}
