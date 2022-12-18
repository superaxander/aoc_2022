use anyhow::Result;
use std::collections::{HashMap, HashSet};

use crate::common;

const ITERATION_COUNT: i64 = 1_000_000_000_000;

const ROCK_SHAPES: &[&[(i64, i64)]] = &[
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    &[(0, 0), (0, 1), (1, 0), (1, 1)],
];

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/17.txt")?;

    let mut winds = Vec::new();

    for line in lines {
        let line = line?;
        winds.extend(line.chars().map(|c| c as i64 - '=' as i64));
    }

    let mut solution_a = 0;
    let mut combinations = HashMap::new();
    let mut order = Vec::new();
    let mut found = false;
    let mut did_reset = false;

    let mut wind_index = 0;
    let mut rock_index = 0;
    let mut highest = 0;
    let mut map = HashSet::new();
    let mut i = 0;
    'outer: while {
        let mut new_rock = ROCK_SHAPES[rock_index]
            .iter()
            .copied()
            .map(|(x, y)| (x + 2, y + highest + 4))
            .collect::<Vec<(i64, i64)>>();

        let key = (rock_index, wind_index);
        if combinations.contains_key(&key) {
            found = true;
            if did_reset {
                let offset_start = order.iter().position(|k| *k == key).unwrap();
                let loop_length = order.iter().rev().position(|k| *k == key).unwrap() + 1;
                let loop_start = (offset_start..=(offset_start + loop_length).min(order.len() - 1))
                    .filter_map(|j| order.iter().position(|k| *k == order[j]))
                    .min()
                    .unwrap();

                let remaining = ITERATION_COUNT - i;
                let mut loop_size = 0;
                for j in 0..loop_length {
                    loop_size += combinations[&order[loop_start + j]];
                }
                highest += loop_size * (remaining / (loop_length as i64));
                let offset = offset_start - loop_start;
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                for j in 0..remaining % (loop_length as i64) {
                    highest +=
                        combinations[&order[(offset + j as usize) % loop_length + loop_start]];
                }
                break 'outer;
            }
        } else if found {
            order.clear();
            combinations.clear();
            found = false;
            did_reset = true;
        }

        let old_rock = rock_index;
        let old_wind = wind_index;
        let old_highest = highest;
        rock_index = (rock_index + 1) % ROCK_SHAPES.len();
        while {
            // apply wind
            if new_rock.first().unwrap().0 + winds[wind_index] >= 0
                && new_rock.last().unwrap().0 + winds[wind_index] < 7
            {
                let mut collision = false;
                for (x, y) in &new_rock {
                    if map.contains(&(*x + winds[wind_index], *y)) {
                        collision = true;
                        break;
                    }
                }
                if !collision {
                    for (x, _) in &mut new_rock {
                        *x += winds[wind_index];
                    }
                }
            }
            wind_index = (wind_index + 1) % winds.len();

            // apply gravity
            let mut hit = false;
            for (x, y) in &mut new_rock {
                if *y == 1 || map.contains(&(*x, *y - 1)) {
                    hit = true;
                }
                *y -= 1;
            }

            !hit
        } {}

        for (x, y) in new_rock {
            map.insert((x, y + 1));
            if y + 1 > highest {
                highest = y + 1;
            }
        }

        combinations.insert((old_rock, old_wind), highest - old_highest);
        order.push((old_rock, old_wind));

        i += 1;
        if i == 2022 {
            solution_a = highest;
        }
        i < ITERATION_COUNT
    } {}
    Ok((solution_a, highest))
}
