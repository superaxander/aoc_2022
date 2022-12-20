use anyhow::Result;
use regex::Regex;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    const ROW_NUMBER: i64 = 2_000_000;
    const MAX_COORD: i64 = 4_000_000;
    let re = Regex::new(
        "Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)",
    )?;

    let lines = common::read_lines("inputs/15.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut sensors = Vec::new();

    for line in lines {
        let line = line?;
        if let Some(captures) = re.captures(&line) {
            let sensor = (
                captures.get(1).unwrap().as_str().parse::<i64>()?,
                captures.get(2).unwrap().as_str().parse::<i64>()?,
            );
            #[allow(clippy::cast_possible_wrap)]
            let distance = manhattan_distance(
                &sensor,
                &(
                    captures.get(3).unwrap().as_str().parse::<i64>()?,
                    captures.get(4).unwrap().as_str().parse::<i64>()?,
                ),
            ) as i64;
            sensors.push((sensor, distance));
        }
    }

    sensors.sort_unstable_by_key(|((from, _), _)| *from);

    let mut excluded_ranges = Vec::new();

    for (sensor, distance) in &sensors {
        let half_distance = distance.wrapping_sub_unsigned(sensor.1.abs_diff(ROW_NUMBER));
        if half_distance >= 0 {
            excluded_ranges.push(((sensor.0 - half_distance), (sensor.0 + half_distance)));
        }
    }
    excluded_ranges.sort_unstable_by_key(|(start, _)| *start);
    let mut last = excluded_ranges[0].0 - 1;
    for range in excluded_ranges.drain(0..) {
        if range.1 > last {
            let diff = last - range.0;
            solution_a += range.1 - range.0;
            if diff > 0 {
                solution_a -= diff;
            }
            last = range.1;
        }
    }

    'lines: for y in 0..=MAX_COORD {
        for (sensor, distance) in &sensors {
            let half_distance = distance.wrapping_sub_unsigned(sensor.1.abs_diff(y));
            if half_distance >= 0 {
                excluded_ranges.push((
                    (sensor.0 - half_distance).max(0),
                    (sensor.0 + half_distance).min(MAX_COORD),
                ));
            }
        }
        excluded_ranges.sort_unstable_by_key(|(start, _)| *start);
        let mut last = -1;
        for range in excluded_ranges.drain(0..) {
            if range.1 > last {
                if range.0 > last + 1 && last + 1 < MAX_COORD {
                    solution_b = (last + 1) * 4_000_000 + y;
                    break 'lines;
                }
                last = range.1;
            }
        }
    }

    Ok((solution_a, solution_b))
}

#[inline]
fn manhattan_distance(p0: &(i64, i64), p1: &(i64, i64)) -> u64 {
    p0.0.abs_diff(p1.0) + p0.1.abs_diff(p1.1)
}
