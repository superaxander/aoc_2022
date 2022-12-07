use anyhow::{Context, Result};

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/4.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;

    for line in lines {
        let string = line?;
        let (left, right) = string.trim().split_once(',').context(", split failed")?;
        let (left_start, left_end) = left.split_once('-').context("- split failed")?;
        let (right_start, right_end) = right.split_once('-').context("- split failed")?;
        let left_start = left_start.parse::<i64>()?;
        let left_end = left_end.parse::<i64>()?;
        let right_start = right_start.parse::<i64>()?;
        let right_end = right_end.parse::<i64>()?;

        let left_range = left_start..=left_end;
        let mut right_range = right_start..=right_end;
        if left_range.clone().all(|i| right_range.contains(&i))
            || right_range.all(|i| left_range.contains(&i))
        {
            solution_a += 1;
        }

        if left_start.max(right_start) <= left_end.min(right_end) {
            solution_b += 1;
        }
    }

    Ok((solution_a, solution_b))
}
