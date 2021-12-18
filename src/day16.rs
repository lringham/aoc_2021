use super::helpers;
// use std::cmp::Ordering;
// use std::cmp::Reverse;
// use std::collections::BinaryHeap;
// use std::collections::HashMap;
// use std::collections::HashSet;
#[derive(Debug)]
enum Packet {
    Operator((char, Vec<Packet>)),
    Literal(usize),
}

pub fn day16() {
    let lines = helpers::read_file("inputs/day16.txt");
    let numbers: Vec<u8> = lines[0]
        .chars()
        .map(|s| s.to_digit(16).unwrap().try_into().unwrap())
        .collect();
    let mut bits: Vec<u8> = Vec::new();
    for hex in numbers {
        for i in 0..4 {
            bits.push(if hex & (0x8 >> i) > 0 { 1 } else { 0 });
        }
    }
    println!("{:?}", bits);

    let mut parent_packet: Packet = Packet::Operator(('r', Vec::new()));
    let mut ver_sum = 0;
    parse_packet(&bits, &mut parent_packet, &mut ver_sum);

    println!("# part 1 - Ans: {:?}", ver_sum); // expected 960
    println!("# part 2 - Ans: {:?}", get_value(&parent_packet)); // expected >9423060
}

fn get_value(packet: &Packet) -> usize {
    let mut ans = 0;
    if let Packet::Operator((op, childern)) = packet {
        ans = match op {
            '+' => {
                let mut ans = 0;
                for c in childern {
                    ans += get_value(c);
                }
                ans
            }
            '*' => {
                let mut ans = 1;
                for c in childern {
                    ans *= get_value(c);
                }
                ans
            }
            'm' => {
                let mut min = usize::MAX;
                print!("min(");
                for c in childern {
                    let v = get_value(c);
                    min = if v < min { v } else { min };
                    print!(" {}", v);
                }
                println!(")");
                min
            }
            'M' => {
                let mut max = usize::MIN;
                print!("max(");
                for c in childern {
                    let v = get_value(c);
                    max = if v > max { v } else { max };
                    print!(" {}", v);
                }
                println!(")");
                max
            }
            '>' => {
                let v1 = get_value(&childern[0]);
                let v2 = get_value(&childern[1]);
                println!("{} {} {}", v1, op, v2);
                if v1 > v2 {
                    1
                } else {
                    0
                }
            }
            '<' => {
                let v1 = get_value(&childern[0]);
                let v2 = get_value(&childern[1]);
                println!("{} {} {}", v1, op, v2);
                if v1 < v2 {
                    1
                } else {
                    0
                }
            }
            '=' => {
                let v1 = get_value(&childern[0]);
                let v2 = get_value(&childern[1]);
                println!("{} {} {}", v1, op, v2);
                if v1 == v2 {
                    1
                } else {
                    0
                }
            }
            'r' => get_value(&childern[0]),
            _ => 0,
        }
    } else if let Packet::Literal(lit) = packet {
        ans = *lit;
    }
    ans
}

fn parse_packet(bits: &[u8], parent_packet: &mut Packet, ver_sum: &mut usize) -> usize {
    let mut i = 0;
    let ver = join_bits(&bits[i..i + 3]);
    i += 3;
    let type_id = join_bits(&bits[i..i + 3]);
    i += 3;
    print!("ver: {:?}, type: {:?}, [", ver, type_id);
    *ver_sum += ver;
    match type_id {
        0x4 => {
            // literal seen
            print!("literal], ");

            let mut parsing_literal = true;
            let mut literal_bits = Vec::new();
            while parsing_literal {
                if bits[i] == 0 {
                    parsing_literal = false;
                }
                i += 1;

                for j in 0..4 {
                    literal_bits.push(bits[i + j]);
                }
                i += 4;
            }
            let literal = join_bits(&literal_bits);

            if let Packet::Operator((_, childern)) = parent_packet {
                childern.push(Packet::Literal(literal));
            }

            println!("value: {:?}", literal);
        }
        _ => {
            // operator seen
            print!("operator], ");

            let length_type = match bits[i] {
                0 => 15,
                1 => 11,
                _ => 0,
            };
            i += 1;
            let mut length = join_bits(&bits[i..i + length_type]);
            i += length_type;

            print!("[{}], ", length_type);
            if let Packet::Operator((_, childern)) = parent_packet {
                match type_id {
                    0 => {
                        print!("+");
                        childern.push(Packet::Operator(('+', Vec::new())))
                    }
                    1 => {
                        print!("*");
                        childern.push(Packet::Operator(('*', Vec::new())))
                    }
                    2 => {
                        print!("m");
                        childern.push(Packet::Operator(('m', Vec::new())))
                    }
                    3 => {
                        print!("M");
                        childern.push(Packet::Operator(('M', Vec::new())))
                    }
                    5 => {
                        print!(">");
                        childern.push(Packet::Operator(('>', Vec::new())))
                    }
                    6 => {
                        print!("<");
                        childern.push(Packet::Operator(('<', Vec::new())))
                    }
                    7 => {
                        print!("=");
                        childern.push(Packet::Operator(('=', Vec::new())))
                    }
                    _ => {}
                }
                println!(", length: {}", length);

                if length_type == 15 {
                    while length > 0 {
                        let bits_parsed =
                            parse_packet(&bits[i..], childern.last_mut().unwrap(), ver_sum);
                        i += bits_parsed;
                        length -= bits_parsed;
                    }
                } else if length_type == 11 {
                    for _ in 0..length {
                        let bits_parsed =
                            parse_packet(&bits[i..], childern.last_mut().unwrap(), ver_sum); // parse packet call here may parse more than one packet!
                        i += bits_parsed;
                    }
                }
            }
        }
    }
    i
}

fn join_bits(bits: &[u8]) -> usize {
    let mut val: usize = 0;
    for (i, byte) in bits.iter().enumerate() {
        val = val | (*byte as usize) << (bits.len() - i - 1)
    }
    val
}

fn join_bytes(bytes: &[u8]) -> u8 {
    let mut val = 0;
    for (i, byte) in bytes.iter().enumerate() {
        val = val | byte << (bytes.len() - i)
    }
    val
}

fn print_packets(packet: &Packet) {
    if let Packet::Operator((op, childern)) = packet {
        println!("{:?}", op);
        for c in childern {
            print_packets(c);
        }
    } else if let Packet::Literal(lit) = packet {
        println!("{:?}", lit);
    }
}
