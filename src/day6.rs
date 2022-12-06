use std::collections::HashSet;
use anyhow::Result;

use crate::common;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/6.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;

    for line in lines {
        let line = line?;
        let chars = line.chars().collect::<Vec<char>>();
        solution_a = chars.array_windows::<4>().enumerate().find(|(_, slice)| {
            slice.iter().collect::<HashSet<&char>>().len() == 4
        }).map(|(i, _)| i).unwrap() + 4;
        solution_b = chars.array_windows::<14>().enumerate().find(|(_, slice)| {
            slice.iter().collect::<HashSet<&char>>().len() == 14
        }).map(|(i, _)| i).unwrap() + 14;
    }

    Ok((solution_a, solution_b))
}
