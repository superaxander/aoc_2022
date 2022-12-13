use anyhow::Result;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Write};

use crate::common;

#[derive(Eq, PartialEq, Clone, Debug)]
enum Packet {
    Integer(usize),
    List(Vec<Packet>),
}

impl PartialOrd<Packet> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.partial_cmp(b),
            (Packet::Integer(a), Packet::List(b)) => vec![Packet::Integer(*a)].partial_cmp(b),
            (Packet::List(a), Packet::Integer(b)) => a.partial_cmp(&vec![Packet::Integer(*b)]),
            (Packet::List(a), Packet::List(b)) => a.partial_cmp(b),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Integer(i) => Display::fmt(i, f),
            Packet::List(packets) => {
                f.write_char('[')?;
                if !packets.is_empty() {
                    for packet in &packets[..packets.len() - 1] {
                        Display::fmt(packet, f)?;
                        f.write_char(',')?;
                    }
                    Display::fmt(packets.last().unwrap(), f)?;
                }
                f.write_char(']')
            }
        }
    }
}

fn build_packet(line: &str) -> (Packet, usize) {
    let mut packets = Vec::new();
    let mut skip = 0;
    let mut number = String::new();
    for (i, c) in line.chars().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        match c {
            ']' => {
                if !number.is_empty() {
                    packets.push(Packet::Integer(number.parse().expect("Expected number")));
                }
                return (Packet::List(packets), i + 1);
            }
            '[' => {
                let (packet, to_skip) = build_packet(&line[i + 1..]);
                packets.push(packet);
                skip = to_skip;
            }
            '0'..='9' => number.push(c),
            ',' => {
                if !number.is_empty() {
                    packets.push(Packet::Integer(number.parse().expect("Expected number")));
                    number.clear();
                }
            }
            _ => panic!("Unexpected character: {c}"),
        }
    }
    (Packet::List(packets), line.len())
}

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/13.txt")?;
    let mut solution_a = 0;

    let mut packets = Vec::new();
    let mut pair_index = 0;

    for line in lines {
        let line = line?;
        if line.is_empty() {
            pair_index += 1;
            if packets[packets.len() - 2] < packets[packets.len() - 1] {
                solution_a += pair_index;
            }
        } else {
            let (packet, _) = build_packet(&line[1..]);
            packets.push(packet);
        }
    }
    let div_packet_a = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let div_packet_b = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
    packets.push(div_packet_a.clone());
    packets.push(div_packet_b.clone());
    packets.sort_unstable();
    let idx_a = packets.iter().position(|p| p == &div_packet_a).unwrap() + 1;
    let îdx_b = packets.iter().position(|p| p == &div_packet_b).unwrap() + 1;
    let solution_b = idx_a * îdx_b;

    Ok((solution_a, solution_b))
}
