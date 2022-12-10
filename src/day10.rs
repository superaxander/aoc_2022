use anyhow::Result;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/10.txt")?;

    let mut solution_a = 0;
    let mut cycle_count = 0;
    let mut value_x = 1;
    let mut crt_rows = vec![];

    for line in lines {
        let line = line?;
        cycle_count += 1;
        solution_a += update_cycle(cycle_count, value_x, &mut crt_rows);
        if line == "noop" {
        } else if let ("addx", offset) = line.split_once(' ').expect("Invalid command format") {
            cycle_count += 1;
            solution_a += update_cycle(cycle_count, value_x, &mut crt_rows);
            value_x += offset.parse::<i64>()?;
        }
    }
    for row in crt_rows {
        println!("{row}");
    }
    Ok((solution_a, 0))
}

fn update_cycle(cycle_count: i64, value_x: i64, crt_rows: &mut Vec<String>) -> i64 {
    let crt_position = (cycle_count - 1) % 40;
    if crt_position == 0 {
        crt_rows.push(String::new());
    }
    if value_x.abs_diff(crt_position) > 1 {
        crt_rows.last_mut().unwrap().push('░');
    } else {
        crt_rows.last_mut().unwrap().push('▓');
    }

    if cycle_count == 20 || (cycle_count - 20) % 40 == 0 {
        cycle_count * value_x
    } else {
        0
    }
}
