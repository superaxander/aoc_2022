use anyhow::Result;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/2.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;

    for line in lines {
        let line = line?;
        let split = line.trim().chars().collect::<Vec<char>>();
        let them = split[0] as i64 - 'A' as i64;
        let you = split[2] as i64 - 'X' as i64;

        solution_a += if them == you {
            3 + you + 1
        } else if (3 + you - them) % 3 == 1 { // Difference is +1
            6 + you + 1
        } else {
            you + 1
        };

        solution_b += match you {
            0 => (3 + them - 1) % 3 + 1,
            1 => them + 1 + 3,
            2 => (them + 1) % 3 + 1 + 6,
            _ => panic!()
        };
    }

    Ok((solution_a, solution_b))
}
