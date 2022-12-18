use anyhow::Result;
use std::collections::HashMap;

use crate::common;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/16.txt")?;

    let mut valves = Vec::new();
    let mut valve_names = HashMap::new();

    for line in lines {
        let line = line?;
        let id = line[6..8].to_owned();
        if let Some((flow_rate, destination)) = line[23..].split_once("; tunnel leads to valve ") {
            let flow_rate = flow_rate.parse::<usize>()?;
            valve_names.insert(id, valves.len());
            valves.push((flow_rate, vec![destination.to_owned()]));
        } else if let Some((flow_rate, list)) = line[23..].split_once("; tunnels lead to valves ") {
            let flow_rate = flow_rate.parse::<usize>()?;
            let destinations = list
                .split(", ")
                .map(ToOwned::to_owned)
                .collect::<Vec<String>>();
            valve_names.insert(id, valves.len());
            valves.push((flow_rate, destinations));
        }
    }
    let valves: Vec<(usize, Vec<usize>)> = valves
        .into_iter()
        .map(|(flow_rate, destinations)| {
            (
                flow_rate,
                destinations.into_iter().map(|d| valve_names[&d]).collect(),
            )
        })
        .collect();
    let init_distances = shortest_distances(&valves);
    let valves = valves
        .into_iter()
        .map(|(flow_rate, _)| flow_rate)
        .collect::<Vec<usize>>();
    let count = valves.len();

    let first_valve = valve_names["AA"];
    let filtered = (0..count)
        .filter(|i| valves[*i] > 0)
        .map(|i| valves[i])
        .collect::<Vec<usize>>();
    let mut distances: Vec<usize> = (0..count)
        .filter(|i| valves[*i] > 0)
        .flat_map(|i| {
            let start = &init_distances[i * count..];
            (0..count)
                .filter(|j| valves[*j] > 0)
                .map(|j| start[j])
                .chain([start[first_valve]])
        })
        .collect();
    let start = &init_distances[first_valve * count..];
    distances.extend((0..count).filter(|j| valves[*j] > 0).map(|j| start[j]));
    distances.push(0);

    let solution_a = solve(
        (filtered.len(), 1, 0),
        &distances,
        &filtered,
        &mut HashMap::new(),
        false,
        0,
    );
    let solution_b = solve(
        (filtered.len(), 5, 0),
        &distances,
        &filtered,
        &mut HashMap::new(),
        true,
        0,
    );
    Ok((solution_a, solution_b))
}

type State = (usize, usize, usize);

#[inline]
fn compress(state: State) -> usize {
    state.0 | state.1 << 8 | state.2 << 16
}

fn solve(
    (position, minute, mut opened): State,
    distances: &[usize],
    valves: &[usize],
    cache: &mut HashMap<usize, usize>,
    first_pass: bool,
    current_best: usize,
) -> usize {
    let len = valves.len() + 1;
    let state = compress((position, minute, opened));
    if !first_pass {
        if cache.contains_key(&state) {
            return cache[&state];
        }
        let mut potential = 0;
        for (i, flow_rate) in valves.iter().copied().enumerate() {
            if opened & (1 << i) == 0 {
                potential += (30 - minute - distances[position * len + i]) * flow_rate;
            }
        }
        if potential < current_best {
            return 0;
        }
    }
    let mut sum: usize = 0;
    for (i, flow_rate) in valves.iter().copied().enumerate() {
        if opened & (1 << i) == 0 {
            let dist = distances[position * len + i] + 1;
            if minute + dist > 30 {
                continue;
            }
            opened |= 1 << i;
            let extra_flow = (31 - minute - dist) * flow_rate;
            let recursive_sum = solve(
                (i, minute + dist, opened),
                distances,
                valves,
                cache,
                first_pass,
                sum.saturating_sub(extra_flow),
            ) + extra_flow;
            opened &= !(1 << i);
            if recursive_sum > sum {
                sum = recursive_sum;
            }
        }
    }
    if first_pass {
        let elephant_sum = solve((len - 1, 5, opened), distances, valves, cache, false, sum);
        if elephant_sum > sum {
            return elephant_sum;
        }
    } else {
        cache.insert(state, sum);
    }

    sum
}

fn shortest_distances(valves: &Vec<(usize, Vec<usize>)>) -> Vec<usize> {
    let len = valves.len();
    let mut distances = vec![usize::MAX - 1; len * len];
    for (valve, (_, destinations)) in valves.iter().enumerate() {
        for d in destinations {
            distances[valve * len + *d] = 1;
        }
        distances[valve * len + valve] = 0;
    }

    let mut changed = true;
    while changed {
        changed = false;

        for (valve, (_, destinations)) in valves.iter().enumerate() {
            for d in destinations {
                for i in 0..len {
                    if distances[*d * len + i] + 1 < distances[valve * len + i] {
                        distances[valve * len + i] = distances[*d * len + i] + 1;
                        changed = true;
                    }
                }
            }
        }
    }

    distances
}
