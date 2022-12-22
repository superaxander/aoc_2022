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
    let (width, height, segments) = if x > y {
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
    };
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

    while populated
        .iter()
        .enumerate()
        .any(|(i, b)| *b && connections[i].contains(&usize::MAX))
    {
        for seg_y in 0..3 {
            for seg_x in 0..4 {
                if populated[seg_y * width + seg_x] {
                    if seg_y > 0 && populated[(seg_y - 1) * width + seg_x] {
                        connections[seg_y * width + seg_x][0] = connections
                            [(seg_y - 1) * width + seg_x][0]
                            .min(connections[seg_y * width + seg_x][0]);
                        connections[seg_y * width + seg_x][2] = connections
                            [(seg_y - 1) * width + seg_x][2]
                            .min(connections[seg_y * width + seg_x][2]);
                        connections[seg_y * width + seg_x][1] = connections
                            [(seg_y - 1) * width + seg_x][4]
                            .min(connections[seg_y * width + seg_x][1]);
                        connections[seg_y * width + seg_x][4] = connections
                            [(seg_y - 1) * width + seg_x][3]
                            .min(connections[seg_y * width + seg_x][4]);
                    }
                    if seg_y < 2 && populated[(seg_y + 1) * width + seg_x] {
                        connections[seg_y * width + seg_x][0] = connections
                            [(seg_y + 1) * width + seg_x][0]
                            .min(connections[seg_y * width + seg_x][0]);
                        connections[seg_y * width + seg_x][2] = connections
                            [(seg_y + 1) * width + seg_x][2]
                            .min(connections[seg_y * width + seg_x][2]);
                        connections[seg_y * width + seg_x][3] = connections
                            [(seg_y + 1) * width + seg_x][4]
                            .min(connections[seg_y * width + seg_x][3]);
                        connections[seg_y * width + seg_x][4] = connections
                            [(seg_y + 1) * width + seg_x][1]
                            .min(connections[seg_y * width + seg_x][4]);
                    }
                    if seg_x > 0 && populated[seg_y * width + seg_x - 1] {
                        connections[seg_y * width + seg_x][1] = connections
                            [seg_y * width + seg_x - 1][1]
                            .min(connections[seg_y * width + seg_x][1]);
                        connections[seg_y * width + seg_x][3] = connections
                            [seg_y * width + seg_x - 1][3]
                            .min(connections[seg_y * width + seg_x][3]);
                        connections[seg_y * width + seg_x][0] = connections
                            [seg_y * width + seg_x - 1][4]
                            .min(connections[seg_y * width + seg_x][0]);
                        connections[seg_y * width + seg_x][4] = connections
                            [seg_y * width + seg_x - 1][2]
                            .min(connections[seg_y * width + seg_x][4]);
                    }
                    if seg_x < 3 && populated[seg_y * width + seg_x + 1] {
                        connections[seg_y * width + seg_x][1] = connections
                            [seg_y * width + seg_x + 1][1]
                            .min(connections[seg_y * width + seg_x][1]);
                        connections[seg_y * width + seg_x][3] = connections
                            [seg_y * width + seg_x + 1][3]
                            .min(connections[seg_y * width + seg_x][3]);
                        connections[seg_y * width + seg_x][2] = connections
                            [seg_y * width + seg_x + 1][4]
                            .min(connections[seg_y * width + seg_x][2]);
                        connections[seg_y * width + seg_x][4] = connections
                            [seg_y * width + seg_x + 1][0]
                            .min(connections[seg_y * width + seg_x][4]);
                    }
                }
            }
        }
    }
    (segments, connections)
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
    segments: &[(usize, usize, usize, usize); 12],
    connections: &[[usize; 5]; 12],
    facing: &mut Direction,
    x: &mut usize,
    y: &mut usize,
    movement: usize,
) {
    for _ in 0..movement {
        match facing {
            Direction::Up => match map.get(&(*x, *y - 1)) {
                None => {
                    let segment = segments
                        .iter()
                        .position(|(min_x, max_x, min_y, max_y)| {
                            *min_x <= *x && *x <= *max_x && *min_y <= *y && *y <= *max_y
                        })
                        .unwrap();
                    let rel_x = *x - segments[segment].0;
                    let new_segment = connections[segment][Direction::Up as usize];
                    let old_facing = *facing;
                    let new_pos = match connections[new_segment]
                        .iter()
                        .position(|d| *d == segment)
                        .unwrap()
                    {
                        0 => {
                            *facing = Direction::Left;
                            (segments[new_segment].1, segments[new_segment].3 - rel_x)
                        }
                        1 => {
                            *facing = Direction::Up;
                            (segments[new_segment].0 + rel_x, segments[new_segment].3)
                        }
                        2 => {
                            *facing = Direction::Right;
                            (segments[new_segment].0, segments[new_segment].2 + rel_x)
                        }
                        3 => {
                            *facing = Direction::Down;
                            (segments[new_segment].1 - rel_x, segments[new_segment].2)
                        }
                        _ => panic!(),
                    };
                    if map[&new_pos] == Tile::Wall {
                        *facing = old_facing;
                        break;
                    }
                    (*x, *y) = new_pos;
                }
                Some(Tile::Wall) => {
                    break;
                }
                Some(Tile::Empty) => {
                    *y -= 1;
                }
            },
            Direction::Down => match map.get(&(*x, *y + 1)) {
                None => {
                    let segment = segments
                        .iter()
                        .position(|(min_x, max_x, min_y, max_y)| {
                            *min_x <= *x && *x <= *max_x && *min_y <= *y && *y <= *max_y
                        })
                        .unwrap();
                    let new_segment = connections[segment][Direction::Down as usize];
                    let rel_x = *x - segments[segment].0;
                    let old_facing = *facing;
                    let new_pos = match connections[new_segment]
                        .iter()
                        .position(|d| *d == segment)
                        .unwrap()
                    {
                        0 => {
                            *facing = Direction::Left;
                            (segments[new_segment].1, segments[new_segment].2 + rel_x)
                        }
                        1 => {
                            *facing = Direction::Up;
                            (segments[new_segment].1 - rel_x, segments[new_segment].3)
                        }
                        2 => {
                            *facing = Direction::Right;
                            (segments[new_segment].0, segments[new_segment].3 - rel_x)
                        }
                        3 => {
                            *facing = Direction::Down;
                            (segments[new_segment].0 + rel_x, segments[new_segment].2)
                        }
                        _ => panic!(),
                    };
                    if map[&new_pos] == Tile::Wall {
                        *facing = old_facing;
                        break;
                    }
                    (*x, *y) = new_pos;
                }
                Some(Tile::Wall) => {
                    break;
                }
                Some(Tile::Empty) => {
                    *y += 1;
                }
            },
            Direction::Right => match map.get(&(*x + 1, *y)) {
                None => {
                    let segment = segments
                        .iter()
                        .position(|(min_x, max_x, min_y, max_y)| {
                            *min_x <= *x && *x <= *max_x && *min_y <= *y && *y <= *max_y
                        })
                        .unwrap();
                    let new_segment = connections[segment][Direction::Right as usize];
                    let rel_y = *y - segments[segment].2;
                    let old_facing = *facing;
                    let new_pos = match connections[new_segment]
                        .iter()
                        .position(|d| *d == segment)
                        .unwrap()
                    {
                        0 => {
                            *facing = Direction::Left;
                            (segments[new_segment].1, segments[new_segment].3 - rel_y)
                        }
                        1 => {
                            *facing = Direction::Up;
                            (segments[new_segment].0 + rel_y, segments[new_segment].3)
                        }
                        2 => {
                            *facing = Direction::Right;
                            (segments[new_segment].0, segments[new_segment].2 + rel_y)
                        }
                        3 => {
                            *facing = Direction::Down;
                            (segments[new_segment].1 - rel_y, segments[new_segment].2)
                        }
                        _ => panic!(),
                    };
                    if map[&new_pos] == Tile::Wall {
                        *facing = old_facing;
                        break;
                    }
                    (*x, *y) = new_pos;
                }
                Some(Tile::Wall) => {
                    break;
                }
                Some(Tile::Empty) => {
                    *x += 1;
                }
            },
            Direction::Left => match map.get(&(*x - 1, *y)) {
                None => {
                    let segment = segments
                        .iter()
                        .position(|(min_x, max_x, min_y, max_y)| {
                            *min_x <= *x && *x <= *max_x && *min_y <= *y && *y <= *max_y
                        })
                        .unwrap();
                    let new_segment = connections[segment][Direction::Left as usize];
                    let rel_y = *y - segments[segment].2;
                    let old_facing = *facing;
                    let new_pos = match connections[new_segment]
                        .iter()
                        .position(|d| *d == segment)
                        .unwrap()
                    {
                        0 => {
                            *facing = Direction::Left;
                            (segments[new_segment].1, segments[new_segment].2 + rel_y)
                        }
                        1 => {
                            *facing = Direction::Up;
                            (segments[new_segment].1 - rel_y, segments[new_segment].3)
                        }
                        2 => {
                            *facing = Direction::Right;
                            (segments[new_segment].0, segments[new_segment].3 - rel_y)
                        }
                        3 => {
                            *facing = Direction::Down;
                            (segments[new_segment].0 + rel_y, segments[new_segment].2)
                        }
                        _ => panic!(),
                    };
                    if map[&new_pos] == Tile::Wall {
                        *facing = old_facing;
                        break;
                    }
                    (*x, *y) = new_pos;
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
