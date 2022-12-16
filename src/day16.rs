use anyhow::Result;
use bit_vec::BitVec;
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
    let distances = shortest_distances(&valves);
    let valves = valves
        .into_iter()
        .map(|(flow_rate, _)| flow_rate)
        .collect::<Vec<usize>>();
    let first_valve = valve_names["AA"];
    let filtered = (0..valves.len())
        .filter(|i| valves[*i] > 0)
        .collect::<Vec<usize>>();

    superluminal_perf::begin_event("Part a");
    let solution_a = solve(
        (first_valve, 1, BitVec::from_elem(filtered.len(), false)),
        &distances,
        &valves,
        &filtered,
        &mut HashMap::new(),
        false,
        first_valve,
    );
    let solution_b = solve(
        (first_valve, 5, BitVec::from_elem(filtered.len(), false)),
        &distances,
        &valves,
        &filtered,
        &mut HashMap::new(),
        true,
        first_valve,
    );
    Ok((solution_a, solution_b))
}

type State = (usize, usize, BitVec);

fn solve(
    (position, minute, mut opened): State,
    distances: &[Vec<usize>],
    valves: &[usize],
    filtered: &[usize],
    cache: &mut HashMap<State, usize>,
    first_pass: bool,
    first_valve: usize,
) -> usize {
    let state = (position, minute, opened.clone());
    if !first_pass && cache.contains_key(&state) {
        return cache[&state];
    }
    let mut sum = 0;
    for (i, valve) in filtered.iter().copied().enumerate() {
        if !opened[i] {
            let dist = distances[position][valve] + 1;
            if minute + dist > 30 {
                continue;
            }
            opened.set(i, true);
            let recursive_sum = solve(
                (valve, minute + dist, opened.clone()),
                distances,
                valves,
                filtered,
                cache,
                first_pass,
                first_valve,
            ) + (31 - minute - dist) * valves[valve];
            opened.set(i, false);
            if recursive_sum > sum {
                sum = recursive_sum;
            }
        }
    }
    if first_pass {
        let elephant_sum = solve(
            (first_valve, 5, opened),
            distances,
            valves,
            filtered,
            cache,
            false,
            first_valve,
        );
        if elephant_sum > sum {
            return elephant_sum;
        }
    } else {
        cache.insert(state, sum);
    }

    sum
}

fn shortest_distances(valves: &Vec<(usize, Vec<usize>)>) -> Vec<Vec<usize>> {
    let mut distances = vec![vec![usize::MAX - 1; valves.len()]; valves.len()];
    for (valve, (_, destinations)) in valves.iter().enumerate() {
        for d in destinations {
            distances[valve][*d] = 1;
        }
        distances[valve][valve] = 0;
    }

    let mut changed = true;
    while changed {
        changed = false;

        for (valve, (_, destinations)) in valves.iter().enumerate() {
            for d in destinations {
                for i in 0..distances.len() {
                    if distances[*d][i] + 1 < distances[valve][i] {
                        distances[valve][i] = distances[*d][i] + 1;
                        changed = true;
                    }
                }
            }
        }
    }

    distances
}
