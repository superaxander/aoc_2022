use std::collections::HashSet;
use anyhow::Result;

use crate::common;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/14.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;
    
    let mut map_positions = HashSet::new();
    let mut largest_height = 0;

    for line in lines {
        let line = line?;
        let mut last_point = None;
        for point in line.split(" -> ") {
            let (x, y) = point.split_once(',').expect("Invalid format");
            let x = x.parse::<usize>()?;
            let y = y.parse::<usize>()?;
            if y > largest_height {
                largest_height = y;
            }
            map_positions.insert((x,y));
            if let Some((previous_x, previous_y)) = last_point {
                if x < previous_x {
                    for x in x..previous_x {
                        map_positions.insert((x,y));
                    }
                } else if x > previous_x {
                    for x in previous_x..x {
                        map_positions.insert((x,y));
                    }
                } else if y < previous_y {
                    for y in y..previous_y {
                        map_positions.insert((x,y));
                    }
                } else {
                    for y in previous_y..y {
                        map_positions.insert((x,y));
                    }
                }
            }
            last_point = Some((x,y));
        }
    }
    
    loop {
        if move_sand(&mut map_positions, largest_height) {
            break;
        }
        solution_a += 1;
    }
    
    largest_height += 2;
    loop {
        solution_b += 1;
        if let Some((500, 0)) = move_sand_with_floor(&mut map_positions, largest_height) {
            break;
        }
    }
    
    Ok((solution_a, solution_a + solution_b))
}

#[inline]
fn move_sand(map: &mut HashSet<(usize,usize)>, largest_height: usize) -> bool {
    let (mut sand_x, mut sand_y) = (500, 0);
    loop {
        while !map.contains(&(sand_x,sand_y+1)) {
            sand_y += 1;
            if sand_y > largest_height {
                return true;
            }
        }
        if!map.contains(&(sand_x-1, sand_y+1)) {
            sand_y += 1;
            sand_x -= 1;
            if sand_y > largest_height {
                return true;
            }
        } else if !map.contains(&(sand_x + 1, sand_y + 1)) {
            sand_y += 1;
            sand_x += 1;
            if sand_y > largest_height {
                return true;
            }
        } else {
            map.insert((sand_x, sand_y));
            break;
        }
    }
    false
}

#[inline]
fn move_sand_with_floor(map: &mut HashSet<(usize,usize)>, largest_height: usize) -> Option<(usize, usize)> {
    let (mut sand_x, mut sand_y) = (500, 0);
    loop {
        while sand_y + 1 < largest_height && !map.contains(&(sand_x,sand_y+1)) {
            sand_y += 1;
        }
        if sand_y + 1 < largest_height && !map.contains(&(sand_x-1, sand_y+1)) {
            sand_y += 1;
            sand_x -= 1;
        } else if sand_y + 1 < largest_height && !map.contains(&(sand_x + 1, sand_y + 1)) {
            sand_y += 1;
            sand_x += 1;
        } else {
            map.insert((sand_x, sand_y));
            break;
        }
    }
    Some((sand_x, sand_y))
}
