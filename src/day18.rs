use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

use crate::common;

type Pos3 = (i64, i64, i64);

pub fn main() -> Result<(i64, i64)> {
    const CUBE_DIRECTIONS: [Pos3; 6] = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];

    let lines = common::read_lines("inputs/18.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut cubes: HashSet<Pos3> = HashSet::new();

    let (mut x_min, mut y_min, mut z_min) = (i64::MAX, i64::MAX, i64::MAX);
    let (mut x_max, mut y_max, mut z_max) = (i64::MIN, i64::MIN, i64::MIN);

    for line in lines {
        let line = line?;
        let pos = line
            .split(',')
            .filter_map(|n| n.parse::<i64>().ok())
            .collect_tuple()
            .expect("Invalid input");
        cubes.insert(pos);
        x_min = x_min.min(pos.0);
        x_max = x_max.max(pos.0);
        y_min = y_min.min(pos.1);
        y_max = y_max.max(pos.1);
        z_min = z_min.min(pos.2);
        z_max = z_max.max(pos.2);
    }

    for (x, y, z) in &cubes {
        for (x_offset, y_offset, z_offset) in CUBE_DIRECTIONS {
            if !cubes.contains(&(x + x_offset, y + y_offset, z + z_offset)) {
                solution_a += 1;
            }
        }
    }

    let mut air_cubes = HashSet::new();
    let mut frontier = VecDeque::new();

    let (x_min, y_min, z_min) = (x_min - 1, y_min - 1, z_min - 1);
    let (x_max, y_max, z_max) = (x_max + 1, y_max + 1, z_max + 1);

    for z in z_min..=z_max {
        for x in x_min..=x_max {
            frontier.push_back((x, y_min, z));
            air_cubes.insert((x, y_min, z));
        }
    }

    while let Some((x, y, z)) = frontier.pop_front() {
        for (x_offset, y_offset, z_offset) in CUBE_DIRECTIONS {
            let (x, y, z) = (x + x_offset, y + y_offset, z + z_offset);
            if x_min <= x && x <= x_max && y_min <= y && y <= y_max && z_min <= z && z <= z_max {
                if cubes.contains(&(x, y, z)) {
                    solution_b += 1;
                } else if air_cubes.insert((x, y, z)) {
                    frontier.push_back((x, y, z));
                }
            }
        }
    }

    Ok((solution_a, solution_b))
}
