use commons::file_loading;


#[derive(Debug)]
enum Packet {
    Literal(u64, u64, usize),
    Operator(u64, OperatorType, Vec<Packet>, usize)
}


#[derive(Debug)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal
}


fn is_padding(bits: &str) -> bool {
    for c in bits.chars() {
        if c == '1' {
            return false
        }
    }
    true
}



fn parse_literal(version: u64, bits: &str) -> Packet {
    // println!("literal version {}", version);
    
    let mut packet_length = 0;
    let mut literal = 0;
    
    loop {
        let next_part = u64::from_str_radix(&bits[packet_length..packet_length + 5], 2).unwrap();
        packet_length += 5;
        literal = literal * 16 + next_part % 16;
        if next_part < 16 {
            break;
        }
    }
    
    Packet::Literal(version, literal, packet_length + 6)
}


fn parse_operator(version: u64, operator_type: OperatorType, bits: &str) -> Packet {
    // println!("operator version {}", version);

    let (sub_packets, packet_len) = match &bits[0..1] {
        "0" => {
            let sub_packets_len = usize::from_str_radix(&bits[1..16], 2).unwrap();
            let mut sub_packet_bits = &bits[16..16 + sub_packets_len];
            let mut sub_packets = Vec::new();

            while !is_padding(sub_packet_bits) {
                let sub_packet = parse_packet(sub_packet_bits);
                match sub_packet {
                    Packet::Literal(_, _, plen) => sub_packet_bits = &sub_packet_bits[plen..],
                    Packet::Operator(_, _, _, plen) => sub_packet_bits = &sub_packet_bits[plen..]
                };
                sub_packets.push(sub_packet);
            }

            (sub_packets, 6 + 1 + 15 + sub_packets_len)
        },
        "1" => {
            let sub_packet_count = usize::from_str_radix(&bits[1..12], 2).unwrap();
            let mut sub_packets_len = 12;
            let mut sub_packets = Vec::new();

            for _ in 0..sub_packet_count {
                let sub_packet = parse_packet(&bits[sub_packets_len..]);
                match sub_packet {
                    Packet::Literal(_, _, plen) => sub_packets_len += plen,
                    Packet::Operator(_, _, _, plen) => sub_packets_len += plen
                };
                sub_packets.push(sub_packet);
            }

            (sub_packets, 6 + sub_packets_len)
        },
        _ => panic!("Invalid opertor length type!")
    };

    Packet::Operator(version, operator_type, sub_packets, packet_len)
}


fn parse_packet(bits: &str) -> Packet {
    let version = u64::from_str_radix(&bits[..3], 2).unwrap();
    match u64::from_str_radix(&bits[3..6], 2) {
        Ok(0) => parse_operator(version, OperatorType::Sum, &bits[6..]),
        Ok(1) => parse_operator(version, OperatorType::Product, &bits[6..]),
        Ok(2) => parse_operator(version, OperatorType::Minimum, &bits[6..]),
        Ok(3) => parse_operator(version, OperatorType::Maximum, &bits[6..]),
        Ok(4) => parse_literal(version, &bits[6..]),
        Ok(5) => parse_operator(version, OperatorType::GreaterThan, &bits[6..]),
        Ok(6) => parse_operator(version, OperatorType::LessThan, &bits[6..]),
        Ok(7) => parse_operator(version, OperatorType::Equal, &bits[6..]),
        _ => panic!("Error occured!")
    }
}


fn sum_packet_versions(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(ver, _, _) => *ver,
        Packet::Operator(ver, _, sub_packets, _) => *ver + sub_packets.iter().map(sum_packet_versions).sum::<u64>()
    }
}


fn evaluate_packet(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(_, val, _) => *val,
        Packet::Operator(_, OperatorType::Sum, sub_packets, _) => sub_packets.iter().map(evaluate_packet).sum(),
        Packet::Operator(_, OperatorType::Product, sub_packets, _) => sub_packets.iter().map(evaluate_packet).product(),
        Packet::Operator(_, OperatorType::Minimum, sub_packets, _) => sub_packets.iter().map(evaluate_packet).min().unwrap(),
        Packet::Operator(_, OperatorType::Maximum, sub_packets, _) => sub_packets.iter().map(evaluate_packet).max().unwrap(),
        Packet::Operator(_, OperatorType::GreaterThan, sub_packets, _) => {
            (evaluate_packet(sub_packets.iter().nth(0).unwrap()) > evaluate_packet(sub_packets.iter().nth(1).unwrap())) as u64
        },
        Packet::Operator(_, OperatorType::LessThan, sub_packets, _) => {
            (evaluate_packet(sub_packets.iter().nth(0).unwrap()) < evaluate_packet(sub_packets.iter().nth(1).unwrap())) as u64
        },
        Packet::Operator(_, OperatorType::Equal, sub_packets, _) => {
            (evaluate_packet(sub_packets.iter().nth(0).unwrap()) == evaluate_packet(sub_packets.iter().nth(1).unwrap())) as u64
        }
    }
}


pub fn main() {
    let content = file_loading::read_file_to_string(2021, 16).unwrap();
    let bits = content.chars().map(|c| format!("{:04b}", c.to_digit(16).unwrap())).collect::<String>();
    let packet = parse_packet(bits.as_str());
    
    let sum_of_versions = sum_packet_versions(&packet);
    println!("{}", sum_of_versions);

    let packet_value = evaluate_packet(&packet);
    println!("{}", packet_value);
}
