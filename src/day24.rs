use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::mem::swap;

use crate::common;

type Point = (usize, usize);

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/24.txt")?;

    let mut blizzards: HashMap<Point, Vec<Direction>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for line in lines {
        let line = line?;
        width = line.len();
        for (x, c) in line.chars().enumerate() {
            match c {
                '>' => blizzards.entry((x, height)).or_default().push(Direction::Right),
                '<' => blizzards.entry((x, height)).or_default().push(Direction::Left),
                'v' => blizzards.entry((x, height)).or_default().push(Direction::Down),
                '^' => blizzards.entry((x, height)).or_default().push(Direction::Up),
                _ => {}
            }
        }
        height += 1;
    }

    let mut sets: (HashSet<Point>, HashSet<Point>) = (HashSet::new(), HashSet::new());
    let current = &mut sets.0;
    let next = &mut sets.1;
    let mut minute = 1 + find_path(
        (1, 0),
        (width - 2, height - 2),
        &mut blizzards,
        width,
        height,
        current,
        next,
    );
    let solution_a = minute;
    current.clear();
    minute += find_path(
        (width - 2, height - 1),
        (1, 1),
        &mut blizzards,
        width,
        height,
        current,
        next,
    );
    current.clear();
    minute += find_path(
        (1, 0),
        (width - 2, height - 2),
        &mut blizzards,
        width,
        height,
        current,
        next,
    );
    let solution_b = minute;

    Ok((solution_a, solution_b))
}

fn find_path(
    start: (usize, usize),
    goal: (usize, usize),
    blizzards: &mut HashMap<Point, Vec<Direction>>,
    width: usize,
    height: usize,
    current: &mut HashSet<Point>,
    next: &mut HashSet<Point>,
) -> usize {
    let mut minute = 0;
    current.insert(start);
    loop {
        let mut to_add = Vec::new();
        for pos in blizzards.keys().copied().collect::<Vec<_>>() {
            let (x, y) = pos;
            for dir in blizzards.get_mut(&pos).unwrap().drain(0..) {
                match dir {
                    Direction::Right => {
                        if x >= width - 2 {
                            to_add.push(((1, y), dir));
                        } else {
                            to_add.push(((x + 1, y), dir));
                        }
                    }
                    Direction::Left => {
                        if x <= 1 {
                            to_add.push(((width - 2, y), dir));
                        } else {
                            to_add.push(((x - 1, y), dir));
                        }
                    }
                    Direction::Down => {
                        if y >= height - 2 {
                            to_add.push(((x, 1), dir));
                        } else {
                            to_add.push(((x, y + 1), dir));
                        }
                    }
                    Direction::Up => {
                        if y <= 1 {
                            to_add.push(((x, height - 2), dir));
                        } else {
                            to_add.push(((x, y - 1), dir));
                        }
                    }
                }
            }
        }
        for (k, v) in to_add {
            blizzards.entry(k).or_default().push(v);
        }
        for (x, y) in current.drain() {
            for (x, y) in [
                (x - 1, y),
                (x + 1, y),
                (x, y.saturating_sub(1)),
                (x, y + 1),
                (x, y),
            ] {
                if (x, y) == start
                    || (0 < x
                    && 0 < y
                    && x < width - 1
                    && y < height - 1
                    && blizzards[&(x, y)].is_empty())
                {
                    next.insert((x, y));
                }
            }
        }
        if next.contains(&goal) {
            swap(current, next);
            return minute + 1;
        }
        swap(current, next);
        minute += 1;
    }
}
