use anyhow::Result;

use crate::common;

const CHARS: [char; 5] = ['=', '-', '0', '1', '2'];
const REV_CHARS: [char; 5] = ['0', '1', '2', '-', '='];

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn main() -> Result<(String, String)> {
    let lines = common::read_lines("inputs/25.txt")?;

    let mut sum = 0;
    for line in lines {
        let line = line?;
        sum += line
            .chars()
            .enumerate()
            .map(|(i, c)| {
                5i64.pow((line.len() - i) as u32 - 1)
                    * (CHARS
                        .iter()
                        .position(|d| *d == c)
                        .expect("Expected valid SNAFU character") as i64
                        - 2)
            })
            .sum::<i64>();
    }

    let mut snafu = String::new();
    while sum > 0 {
        let rem = sum % 5;

        snafu.insert(0, REV_CHARS[rem as usize]);
        sum = (sum + 2) / 5;
    }

    Ok((snafu, "No part two!".to_owned()))
}
