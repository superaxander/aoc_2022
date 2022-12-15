use anyhow::Result;
use regex::Regex;

use crate::common;

pub fn main() -> Result<(usize, usize)> {
    const ROW_NUMBER: isize = 2_000_000;
    const MAX_COORD: isize = 4_000_000;
    let re = Regex::new(
        "Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)",
    )?;

    let lines = common::read_lines("inputs/15.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    for line in lines {
        let line = line?;
        if let Some(captures) = re.captures(&line) {
            sensors.push((
                captures.get(1).unwrap().as_str().parse::<isize>()?,
                captures.get(2).unwrap().as_str().parse::<isize>()?,
            ));
            beacons.push((
                captures.get(3).unwrap().as_str().parse::<isize>()?,
                captures.get(4).unwrap().as_str().parse::<isize>()?,
            ));
        }
    }
    let mut excluded_ranges = Vec::new();

    for i in 0..sensors.len() {
        let sensor = sensors[i];
        let beacon = beacons[i];
        let distance = manhattan_distance(&sensor, &beacon) as isize;
        let distance_from_y = sensor.1.abs_diff(ROW_NUMBER) as isize;
        let half_distance = distance - distance_from_y;
        if half_distance >= 0 {
            excluded_ranges.push(((sensor.0 - half_distance), (sensor.0 + half_distance)));
        }
    }
    excluded_ranges.sort_unstable_by_key(|(start, _)| *start);
    let mut last = excluded_ranges[0].0 - 1;
    for range in &excluded_ranges {
        if range.1 > last {
            let diff = last - range.0;
            solution_a += range.1 - range.0;
            if diff > 0 {
                solution_a -= diff;
            }
            last = range.1;
        }
    }
    excluded_ranges.clear();

    for y in 0..=MAX_COORD {
        for i in 0..sensors.len() {
            let sensor = sensors[i];
            let beacon = beacons[i];
            let distance = manhattan_distance(&sensor, &beacon) as isize;
            let distance_from_y = sensor.1.abs_diff(y) as isize;
            let half_distance = distance - distance_from_y;
            if half_distance >= 0 {
                excluded_ranges.push((
                    (sensor.0 - half_distance).max(0),
                    (sensor.0 + half_distance).min(MAX_COORD),
                ));
            }
        }
        excluded_ranges.sort_unstable_by_key(|(start, _)| *start);
        let mut last = -1;
        for range in &excluded_ranges {
            if range.1 > last {
                if range.0 > last + 1 && last + 1 < MAX_COORD {
                    solution_b = (last + 1) * 4000000 + y;
                }
                last = range.1;
            }
        }
        excluded_ranges.clear();
    }

    Ok((solution_a as usize, solution_b as usize))
}

#[inline]
fn manhattan_distance(p0: &(isize, isize), p1: &(isize, isize)) -> usize {
    p0.0.abs_diff(p1.0) + p0.1.abs_diff(p1.1)
}
