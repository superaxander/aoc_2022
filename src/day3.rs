use std::collections::HashSet;
use std::io;

use crate::common;

fn value(char: char) -> i64 {
    match char {
        ('A'..='Z') => 27 + char as i64 - 'A' as i64,
        ('a'..='z') => 1 + char as i64 - 'a' as i64,
        _ => panic!(),
    }
}

pub fn main() -> io::Result<(i64, i64)> {
    let lines = common::read_lines("inputs/3.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut i = 0;
    let mut badge_set = HashSet::new();
    let mut compartment_set = HashSet::new();

    for line in lines {
        let line = line?;
        let mut chars = line.chars();
        for _ in 0..line.len() / 2 {
            compartment_set.insert(chars.next().unwrap());
        }
        for char in chars {
            if compartment_set.contains(&char) {
                solution_a += value(char);
                break;
            }
        }
        compartment_set.clear();

        match i {
            0 => badge_set.extend(line.chars()),
            1 => badge_set.retain(|c| line.contains(*c)),
            2 => solution_b += value(badge_set.drain().find(|c: &char| line.contains(*c)).expect("No common item found")),
            _ => unreachable!(),
        }
        i = (i + 1) % 3;
    }

    Ok((solution_a, solution_b))
}
