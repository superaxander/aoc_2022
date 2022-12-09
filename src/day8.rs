use anyhow::Result;

use crate::common;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/8.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut grid = Vec::new();
    let mut width = 0;
    for line in lines {
        let line = line?;
        width = line.len();
        grid.extend(line.chars().map(|c| c as usize - '0' as usize));
    }
    let height = grid.len() / width;

    for y in 0..height {
        for x in 0..width {
            let h = grid[y * width + x];
            let (left, c_l) = check_range_x(&grid, width, y, h, (0..x).rev());
            let (right, c_r) = check_range_x(&grid, width, y, h, x + 1..width);
            let (top, c_t) = check_range_y(&grid, width, x, h, (0..y).rev());
            let (bottom, c_b) = check_range_y(&grid, width, x, h, y + 1..height);
            if left || right || top || bottom {
                solution_a += 1;
            }
            let scenic_score = c_l * c_r * c_t * c_b;
            if scenic_score > solution_b {
                solution_b = scenic_score;
            }
        }
    }

    Ok((solution_a, solution_b))
}

fn check_range_x<T: Iterator<Item = usize>>(
    grid: &[usize],
    width: usize,
    y: usize,
    h: usize,
    range: T,
) -> (bool, usize) {
    let mut visible = true;
    let mut visible_trees = 0;
    for x in range {
        visible_trees += 1;
        if grid[y * width + x] >= h {
            visible = false;
            break;
        }
    }
    (visible, visible_trees)
}

fn check_range_y<T: Iterator<Item = usize>>(
    grid: &[usize],
    width: usize,
    x: usize,
    h: usize,
    range: T,
) -> (bool, usize) {
    let mut visible = true;
    let mut visible_trees = 0;
    for y in range {
        visible_trees += 1;
        if grid[y * width + x] >= h {
            visible = false;
            break;
        }
    }
    (visible, visible_trees)
}
