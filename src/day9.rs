use anyhow::Result;
use std::cmp::Ordering;
use std::collections::HashSet;

use crate::common;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/9.txt")?;

    let mut knots = [(0, 0); 10];
    let mut tail_history_a = HashSet::new();
    let mut tail_history_b = HashSet::new();
    tail_history_a.insert((0, 0));
    tail_history_b.insert((0, 0));

    for line in lines {
        let line = line?;
        let (direction, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse::<usize>()?;
        for _ in 0..amount {
            match direction {
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                "L" => knots[0].0 -= 1,
                "R" => knots[0].0 += 1,
                _ => panic!(),
            }
            for i in 1..knots.len() {
                update_knot(knots[i - 1], &mut knots[i]);
            }
            tail_history_a.insert(knots[1]);
            tail_history_b.insert(knots[knots.len() - 1]);
        }
    }

    Ok((tail_history_a.len(), tail_history_b.len()))
}

fn update_knot(head_position: (i64, i64), tail_position: &mut (i64, i64)) {
    if head_position.0.abs_diff(tail_position.0) > 1 {
        match head_position.0.cmp(&tail_position.0) {
            Ordering::Less => tail_position.0 -= 1,
            Ordering::Greater => tail_position.0 += 1,
            Ordering::Equal => unreachable!(),
        }
        match head_position.1.cmp(&tail_position.1) {
            Ordering::Less => tail_position.1 -= 1,
            Ordering::Equal => {}
            Ordering::Greater => tail_position.1 += 1,
        }
    } else if head_position.1.abs_diff(tail_position.1) > 1 {
        match head_position.1.cmp(&tail_position.1) {
            Ordering::Less => tail_position.1 -= 1,
            Ordering::Greater => tail_position.1 += 1,
            Ordering::Equal => unreachable!(),
        }
        match head_position.0.cmp(&tail_position.0) {
            Ordering::Less => tail_position.0 -= 1,
            Ordering::Equal => {}
            Ordering::Greater => tail_position.0 += 1,
        }
    }
}
