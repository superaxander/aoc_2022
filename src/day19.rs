use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::common;

pub fn main() -> Result<(usize, usize)> {
    let re = Regex::new("Blueprint (\\d+): Each ore robot costs (\\d+) ore. Each clay robot costs (\\d+) ore. Each obsidian robot costs (\\d+) ore and (\\d+) clay. Each geode robot costs (\\d+) ore and (\\d+) obsidian.")?;

    let lines = common::read_lines("inputs/19.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 1;

    for line in lines {
        let line = line?;
        let matches = re.captures(&line).unwrap();

        let id: usize = matches.get(1).unwrap().as_str().parse()?;

        let costs = Costs {
            ore_costs_ore: matches.get(2).unwrap().as_str().parse()?,
            ore_costs_clay: matches.get(3).unwrap().as_str().parse()?,
            ore_costs_obsidian: matches.get(4).unwrap().as_str().parse()?,
            clay_costs_obsidian: matches.get(5).unwrap().as_str().parse()?,
            ore_costs_geode: matches.get(6).unwrap().as_str().parse()?,
            obsidian_costs_geode: matches.get(7).unwrap().as_str().parse()?,
        };
        let state = State {
            tick: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_production: 1,
            clay_production: 0,
            obsidian_production: 0,
            geode_production: 0,
        };
        println!("Solving for {costs:?}");
        // let geodes = solve(&state, &costs, 24,&mut HashMap::new());
        // println!("Geodes after 24: {}", geodes);
        // solution_a += id * geodes;
        if id < 4 {
            let geodes = solve(&state, &costs, 32, &mut HashMap::new(), 1);
            println!("Geodes after 32: {}", geodes);
            solution_b *= geodes;
        }
    }

    Ok((solution_a, solution_b))
}

#[derive(Debug)]
struct Costs {
    ore_costs_ore: usize,
    ore_costs_clay: usize,
    ore_costs_obsidian: usize,
    clay_costs_obsidian: usize,
    ore_costs_geode: usize,
    obsidian_costs_geode: usize,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Hash)]
struct State {
    tick: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
    ore_production: usize,
    clay_production: usize,
    obsidian_production: usize,
    geode_production: usize,
}

fn solve(
    state: &State,
    costs: &Costs,
    max_tick: usize,
    cache: &mut HashMap<State, usize>,
    best: usize,
) -> usize {
    if state.tick >= max_tick {
        return best;
    }

    if cache.contains_key(state) {
        return cache[state];
    }

    let mut best = best;
    if costs.ore_costs_geode <= state.ore && costs.obsidian_costs_geode <= state.obsidian {
        let mut new_state = state.clone();
        new_state.tick += 1;
        new_state.ore -= costs.ore_costs_geode;
        new_state.ore += state.ore_production;
        new_state.clay += state.clay_production;
        new_state.obsidian -= costs.obsidian_costs_geode;
        new_state.obsidian += state.obsidian_production;
        new_state.geodes += state.geode_production;
        new_state.geode_production += 1;
        best = solve(&new_state, costs, max_tick, cache, best)
    } else {
        if costs.ore_costs_obsidian <= state.ore && costs.clay_costs_obsidian <= state.clay {
            let mut new_state = state.clone();
            new_state.tick += 1;
            new_state.ore -= costs.ore_costs_obsidian;
            new_state.ore += state.ore_production;
            new_state.clay -= costs.clay_costs_obsidian;
            new_state.clay += state.clay_production;
            new_state.obsidian += state.obsidian_production;
            new_state.geodes += state.geode_production;
            new_state.obsidian_production += 1;
            best = solve(&new_state, costs, max_tick, cache, best);
        } else {
            if costs.ore_costs_clay <= state.ore {
                let mut new_state = state.clone();
                new_state.tick += 1;
                new_state.ore -= costs.ore_costs_clay;
                new_state.ore += state.ore_production;
                new_state.clay += state.clay_production;
                new_state.obsidian += state.obsidian_production;
                new_state.geodes += state.geode_production;
                new_state.clay_production += 1;
                best = best.max(solve(&new_state, costs, max_tick, cache, best));
            }
            if costs.ore_costs_ore <= state.ore {
                let mut new_state = state.clone();
                new_state.tick += 1;
                new_state.ore -= costs.ore_costs_ore;
                new_state.ore += state.ore_production;
                new_state.clay += state.clay_production;
                new_state.obsidian += state.obsidian_production;
                new_state.geodes += state.geode_production;
                new_state.ore_production += 1;
                best = best.max(solve(&new_state, costs, max_tick, cache, best));
            }
        }

        let mut new_state = state.clone();
        new_state.tick += 1;
        new_state.ore += state.ore_production;
        new_state.clay += state.clay_production;
        new_state.obsidian += state.obsidian_production;
        new_state.geodes += state.geode_production;
        best = best.max(solve(&new_state, costs, max_tick, cache, best));
    }

    cache.insert(state.clone(), best);

    best
}
