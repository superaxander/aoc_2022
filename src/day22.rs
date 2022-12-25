use anyhow::Result;
use std::collections::HashMap;

use crate::common;

#[derive(Eq, PartialEq, Debug)]
enum Tile {
    Wall,
    Empty,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

type Region = (usize, usize, usize, usize);

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/22.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut map_is_done = false;
    let mut map = HashMap::new();
    let mut y = 1;

    let mut start = None;

    for line in lines {
        let line = line?;
        if map_is_done {
            // Figure out segments
            y -= 1;
            let x = map.keys().map(|(x, _)| *x).max().unwrap();

            let (segments, connections) = fold_cube(&map, x, y);

            // Do steps
            let mut facing = Direction::Right;
            let (mut x, mut y) = start.unwrap();
            for action in line.split_inclusive(|c| c == 'L' || c == 'R') {
                if action.ends_with(|c| c == 'L' || c == 'R') {
                    let movement = action[..action.len() - 1].parse()?;
                    do_move_wrapping(&map, &mut facing, &mut x, &mut y, movement);
                    facing = do_turn(facing, action.chars().last().unwrap());
                } else {
                    let movement = action.parse()?;
                    do_move_wrapping(&map, &mut facing, &mut x, &mut y, movement);
                }
            }
            solution_a = y * 1000 + x * 4 + facing as usize;

            let mut facing = Direction::Right;
            let (mut x, mut y) = start.unwrap();
            for action in line.split_inclusive(|c| c == 'L' || c == 'R') {
                if action.ends_with(|c| c == 'L' || c == 'R') {
                    let movement = action[..action.len() - 1].parse()?;
                    do_move_cube(
                        &map,
                        &segments,
                        &connections,
                        &mut facing,
                        &mut x,
                        &mut y,
                        movement,
                    );
                    facing = do_turn(facing, action.chars().last().unwrap());
                } else {
                    let movement = action.parse()?;
                    do_move_cube(
                        &map,
                        &segments,
                        &connections,
                        &mut facing,
                        &mut x,
                        &mut y,
                        movement,
                    );
                }
            }
            solution_b = y * 1000 + x * 4 + facing as usize;
        } else if line.is_empty() {
            map_is_done = true;
        } else {
            let mut x = 1;
            for c in line.chars() {
                match c {
                    '#' => {
                        map.insert((x, y), Tile::Wall);
                    }
                    '.' => {
                        if start.is_none() {
                            start = Some((x, y));
                        }
                        map.insert((x, y), Tile::Empty);
                    }
                    _ => {}
                }
                x += 1;
            }

            y += 1;
        }
    }

    Ok((solution_a, solution_b))
}

fn do_turn(facing: Direction, dir: char) -> Direction {
    match dir {
        'R' => match facing {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        },
        'L' => match facing {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        },
        _ => panic!(),
    }
}

fn fold_cube(
    map: &HashMap<(usize, usize), Tile>,
    x: usize,
    y: usize,
) -> ([Region; 12], [[usize; 5]; 12]) {
    let (width, height, segments) = get_segments(x, y);
    let mut populated = [false; 12];
    for (i, (x_min, _, y_min, _)) in segments.iter().enumerate() {
        if map.contains_key(&(*x_min, *y_min)) {
            populated[i] = true;
        }
    }

    let mut connections = [[usize::MAX; 5]; 12];

    for seg_y in 0..height {
        for seg_x in 0..width {
            if populated[seg_y * width + seg_x] {
                if seg_y > 0 && populated[(seg_y - 1) * width + seg_x] {
                    connections[seg_y * width + seg_x][Direction::Up as usize] =
                        (seg_y - 1) * width + seg_x;
                    connections[(seg_y - 1) * width + seg_x][Direction::Down as usize] =
                        seg_y * width + seg_x;
                }
                if seg_y < 2 && populated[(seg_y + 1) * width + seg_x] {
                    connections[seg_y * width + seg_x][Direction::Down as usize] =
                        (seg_y + 1) * width + seg_x;
                    connections[(seg_y + 1) * width + seg_x][Direction::Up as usize] =
                        seg_y * width + seg_x;
                }
                if seg_x > 0 && populated[seg_y * width + seg_x - 1] {
                    connections[seg_y * width + seg_x][Direction::Left as usize] =
                        seg_y * width + seg_x - 1;
                    connections[seg_y * width + seg_x - 1][Direction::Right as usize] =
                        seg_y * width + seg_x;
                }
                if seg_x < 3 && populated[seg_y * width + seg_x + 1] {
                    connections[seg_y * width + seg_x][Direction::Right as usize] =
                        seg_y * width + seg_x + 1;
                    connections[seg_y * width + seg_x + 1][Direction::Left as usize] =
                        seg_y * width + seg_x;
                }
            }
        }
    }

    iterate_connections(width, height, &populated, &mut connections);
    (segments, connections)
}

fn iterate_connections(
    width: usize,
    height: usize,
    populated: &[bool; 12],
    connections: &mut [[usize; 5]; 12],
) {
    while populated
        .iter()
        .enumerate()
        .any(|(i, b)| *b && connections[i].contains(&usize::MAX))
    {
        for seg_y in 0..height {
            for seg_x in 0..width {
                let this = seg_y * width + seg_x;
                if populated[this] {
                    let up = this - width;
                    let down = this + width;
                    let left = this - 1;
                    let right = this + 1;
                    if seg_y > 0 && populated[up] {
                        connections[this][0] = connections[up][0].min(connections[this][0]);
                        connections[this][2] = connections[up][2].min(connections[this][2]);
                        connections[this][1] = connections[up][4].min(connections[this][1]);
                        connections[this][4] = connections[up][3].min(connections[this][4]);
                    }
                    if seg_y < 2 && populated[down] {
                        connections[this][0] = connections[down][0].min(connections[this][0]);
                        connections[this][2] = connections[down][2].min(connections[this][2]);
                        connections[this][3] = connections[down][4].min(connections[this][3]);
                        connections[this][4] = connections[down][1].min(connections[this][4]);
                    }
                    if seg_x > 0 && populated[left] {
                        connections[this][1] = connections[left][1].min(connections[this][1]);
                        connections[this][3] = connections[left][3].min(connections[this][3]);
                        connections[this][0] = connections[left][4].min(connections[this][0]);
                        connections[this][4] = connections[left][2].min(connections[this][4]);
                    }
                    if seg_x < 3 && populated[right] {
                        connections[this][1] = connections[right][1].min(connections[this][1]);
                        connections[this][3] = connections[right][3].min(connections[this][3]);
                        connections[this][2] = connections[right][4].min(connections[this][2]);
                        connections[this][4] = connections[right][0].min(connections[this][4]);
                    }
                }
            }
        }
    }
}

fn get_segments(x: usize, y: usize) -> (usize, usize, [Region; 12]) {
    if x > y {
        let l = x / 4;
        (
            4,
            3,
            [
                (1, l, 1, l),
                (l + 1, l * 2, 1, l),
                (l * 2 + 1, l * 3, 1, l),
                (l * 3 + 1, l * 4, 1, l),
                (1, l, l + 1, l * 2),
                (l + 1, l * 2, l + 1, l * 2),
                (l * 2 + 1, l * 3, l + 1, l * 2),
                (l * 3 + 1, l * 4, l + 1, l * 2),
                (1, l, l * 2 + 1, l * 3),
                (l + 1, l * 2, l * 2 + 1, l * 3),
                (l * 2 + 1, l * 3, l * 2 + 1, l * 3),
                (l * 3 + 1, l * 4, l * 2 + 1, l * 3),
            ],
        )
    } else {
        let l = y / 4;
        (
            3,
            4,
            [
                (1, l, 1, l),
                (l + 1, l * 2, 1, l),
                (l * 2 + 1, l * 3, 1, l),
                (1, l, l + 1, l * 2),
                (l + 1, l * 2, l + 1, l * 2),
                (l * 2 + 1, l * 3, l + 1, l * 2),
                (1, l, l * 2 + 1, l * 3),
                (l + 1, l * 2, l * 2 + 1, l * 3),
                (l * 2 + 1, l * 3, l * 2 + 1, l * 3),
                (1, l, l * 3 + 1, l * 4),
                (l + 1, l * 2, l * 3 + 1, l * 4),
                (l * 2 + 1, l * 3, l * 3 + 1, l * 4),
            ],
        )
    }
}

fn do_move_wrapping(
    map: &HashMap<(usize, usize), Tile>,
    facing: &mut Direction,
    x: &mut usize,
    y: &mut usize,
    movement: usize,
) {
    for _ in 0..movement {
        match facing {
            Direction::Up => match map.get(&(*x, *y - 1)) {
                None => {
                    let (_, wrap_y) = map
                        .keys()
                        .filter(|(j, _)| j == x)
                        .max_by_key(|(_, y)| *y)
                        .unwrap();
                    if map[&(*x, *wrap_y)] == Tile::Wall {
                        break;
                    }
                    *y = *wrap_y;
                }
                Some(Tile::Wall) => {
                    break;
                }
                Some(Tile::Empty) => {
                    *y -= 1;
                }
            },
            Direction::Right => match map.get(&(*x + 1, *y)) {
                None => {
                    let (wrap_x, _) = map
                        .keys()
                        .filter(|(_, j)| j == y)
                        .min_by_key(|(x, _)| *x)
                        .unwrap();
                    if map[&(*wrap_x, *y)] == Tile::Wall {
                        break;
                    }
                    *x = *wrap_x;
                }
                Some(Tile::Wall) => {
                    break;
                }
                Some(Tile::Empty) => {
                    *x += 1;
                }
            },
            Direction::Down => match map.get(&(*x, *y + 1)) {
                None => {
                    let (_, wrap_y) = map
                        .keys()
                        .filter(|(j, _)| j == x)
                        .min_by_key(|(_, y)| *y)
                        .unwrap();
                    if map[&(*x, *wrap_y)] == Tile::Wall {
                        break;
                    }
                    *y = *wrap_y;
                }
                Some(Tile::Wall) => {
                    break;
                }
                Some(Tile::Empty) => {
                    *y += 1;
                }
            },
            Direction::Left => match map.get(&(*x - 1, *y)) {
                None => {
                    let (wrap_x, _) = map
                        .keys()
                        .filter(|(_, j)| j == y)
                        .max_by_key(|(x, _)| *x)
                        .unwrap();
                    if map[&(*wrap_x, *y)] == Tile::Wall {
                        break;
                    }
                    *x = *wrap_x;
                }
                Some(Tile::Wall) => {
                    break;
                }
                Some(Tile::Empty) => {
                    *x -= 1;
                }
            },
        }
    }
}

fn do_move_cube(
    map: &HashMap<(usize, usize), Tile>,
    segments: &[Region; 12],
    connections: &[[usize; 5]; 12],
    facing: &mut Direction,
    x: &mut usize,
    y: &mut usize,
    movement: usize,
) {
    for _ in 0..movement {
        let segment = segments
            .iter()
            .position(|(min_x, max_x, min_y, max_y)| {
                *min_x <= *x && *x <= *max_x && *min_y <= *y && *y <= *max_y
            })
            .unwrap();
        let new_segment = connections[segment][*facing as usize];
        let dir = connections[new_segment]
            .iter()
            .position(|d| *d == segment)
            .unwrap();
        let pos = match facing {
            Direction::Up => map.get(&(*x, *y - 1)),
            Direction::Right => map.get(&(*x + 1, *y)),
            Direction::Down => map.get(&(*x, *y + 1)),
            Direction::Left => map.get(&(*x - 1, *y)),
        };
        let (nx, px, ny, py) = segments[new_segment];
        match pos {
            None => {
                let new_pos = match facing {
                    Direction::Up => {
                        let rel_x = *x - segments[segment].0;
                        match dir {
                            0 => (px, py - rel_x),
                            1 => (nx + rel_x, py),
                            2 => (nx, ny + rel_x),
                            3 => (px - rel_x, ny),
                            _ => panic!(),
                        }
                    }
                    Direction::Down => {
                        let rel_x = *x - segments[segment].0;
                        match dir {
                            0 => (px, ny + rel_x),
                            1 => (px - rel_x, py),
                            2 => (nx, py - rel_x),
                            3 => (nx + rel_x, ny),
                            _ => panic!(),
                        }
                    }
                    Direction::Right => {
                        let rel_y = *y - segments[segment].2;
                        match dir {
                            0 => (px, py - rel_y),
                            1 => (nx + rel_y, py),
                            2 => (nx, ny + rel_y),
                            3 => (px - rel_y, ny),
                            _ => panic!(),
                        }
                    }
                    Direction::Left => {
                        let rel_y = *y - segments[segment].2;
                        match dir {
                            0 => (px, ny + rel_y),
                            1 => (px - rel_y, py),
                            2 => (nx, py - rel_y),
                            3 => (nx + rel_y, ny),
                            _ => panic!(),
                        }
                    }
                };
                if map[&new_pos] == Tile::Wall {
                    break;
                }
                *facing = match dir {
                    0 => Direction::Left,
                    1 => Direction::Up,
                    2 => Direction::Right,
                    3 => Direction::Down,
                    _ => panic!(),
                };
                (*x, *y) = new_pos;
            }
            Some(Tile::Wall) => {
                break;
            }
            Some(Tile::Empty) => match facing {
                Direction::Right => *x += 1,
                Direction::Down => *y += 1,
                Direction::Left => *x -= 1,
                Direction::Up => *y -= 1,
            },
        }
    }
}
