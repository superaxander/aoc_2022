use anyhow::Result;
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use crate::common;

type Point = (i64, i64);

const DIRECTIONS: [Point; 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (-1, 1),
    (0, 1),
    (-1, 0),
];
const NORTH_OFFSETS: [Point; 3] = [(-1, -1), (0, -1), (1, -1)];
const EAST_OFFSETS: [Point; 3] = [(1, -1), (1, 0), (1, 1)];
const SOUTH_OFFSETS: [Point; 3] = [(-1, 1), (0, 1), (1, 1)];
const WEST_OFFSETS: [Point; 3] = [(-1, -1), (-1, 0), (-1, 1)];

const OFFSETS: [(Point, &[Point; 3]); 4] = [
    ((0, -1), &NORTH_OFFSETS),
    ((0, 1), &SOUTH_OFFSETS),
    ((-1, 0), &WEST_OFFSETS),
    ((1, 0), &EAST_OFFSETS),
];

pub fn main() -> Result<Point> {
    let lines = common::read_lines("inputs/23.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut map = HashSet::new();

    for (y, line) in lines.enumerate() {
        let line = line?;
        map.extend(line.chars().enumerate().filter_map(|(x, c)| {
            if c == '#' {
                Some((x as i64, y as i64))
            } else {
                None
            }
        }));
    }

    for round in 0.. {
        let first = OFFSETS[round % 4];
        let second = OFFSETS[(round + 1) % 4];
        let third = OFFSETS[(round + 2) % 4];
        let fourth = OFFSETS[(round + 3) % 4];

        let mut proposals = HashMap::new();
        let mut to_be_skipped = HashSet::new();
        let copy = map.clone();
        map.retain(|pair| {
            let (x, y) = *pair;
            if !DIRECTIONS
                .iter()
                .any(|(x_offset, y_offset)| copy.contains(&(x + x_offset, y + y_offset)))
            {
                return true;
            }

            !(add_proposal(x, y, &mut proposals, &copy, &mut to_be_skipped, first)
                || add_proposal(x, y, &mut proposals, &copy, &mut to_be_skipped, second)
                || add_proposal(x, y, &mut proposals, &copy, &mut to_be_skipped, third)
                || add_proposal(x, y, &mut proposals, &copy, &mut to_be_skipped, fourth))
        });
        let mut moved = false;
        for pos in &to_be_skipped {
            map.insert(*pos);
        }
        for (dst, src) in &proposals {
            if to_be_skipped.contains(src) {
                continue;
            }
            moved = true;
            map.insert(*dst);
        }

        if !moved {
            solution_b = round as i64 + 1;
            break;
        }

        if round == 9 {
            let (min_x, max_x) = map.iter().minmax_by_key(|(x, _)| *x).into_option().unwrap();
            let (min_y, max_y) = map.iter().minmax_by_key(|(_, y)| *y).into_option().unwrap();
            solution_a = (max_x.0 - min_x.0 + 1) * (max_y.1 - min_y.1 + 1) - map.len() as i64;
        }
    }

    Ok((solution_a, solution_b))
}

#[inline]
fn add_proposal(
    x: i64,
    y: i64,
    proposals: &mut HashMap<Point, Point>,
    map: &HashSet<Point>,
    to_be_skipped: &mut HashSet<Point>,
    (offset, directions): (Point, &[Point; 3]),
) -> bool {
    if directions
        .iter()
        .any(|(x_offset, y_offset)| map.contains(&(x + x_offset, y + y_offset)))
    {
        false
    } else {
        match proposals.entry((x + offset.0, y + offset.1)) {
            Entry::Occupied(e) => {
                to_be_skipped.insert(*e.get());
                to_be_skipped.insert((x, y));
            }
            Entry::Vacant(e) => {
                e.insert((x, y));
            }
        }
        true
    }
}
