use anyhow::Result;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

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
        let geodes = solve(&state, &costs, 24, 0, &mut HashMap::new());
        solution_a += id * geodes;
        if id < 4 {
            let geodes = solve(&state, &costs, 32, 0, &mut HashMap::new());
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
    mut best: usize,
    cache: &mut HashMap<State, usize>,
) -> usize {
    match state.tick.cmp(&max_tick) {
        Ordering::Equal => return state.geodes,
        Ordering::Greater => return 0,
        Ordering::Less => {}
    }

    if state.geodes
        + (state.tick..max_tick)
            .map(|i| (max_tick - i) * (state.geode_production + i - state.tick - 1))
            .sum::<usize>()
        < best
    {
        return 0;
    }

    if cache.contains_key(state) {
        return cache[state];
    }

    best = best.max(state.geodes + state.geode_production * (max_tick - state.tick - 1));

    if state.obsidian_production > 0 {
        let ore_ticks = (costs.ore_costs_geode - state.ore.min(costs.ore_costs_geode))
            .div_ceil(state.ore_production);
        let obsidian_ticks = (costs.obsidian_costs_geode
            - state.obsidian.min(costs.obsidian_costs_geode))
        .div_ceil(state.obsidian_production);
        let ticks = ore_ticks.max(obsidian_ticks) + 1;
        let mut new_state = state.clone();
        new_state.tick += ticks;
        new_state.ore += state.ore_production * ticks;
        new_state.ore -= costs.ore_costs_geode;
        new_state.clay += state.clay_production * ticks;
        new_state.obsidian += state.obsidian_production * ticks;
        new_state.obsidian -= costs.obsidian_costs_geode;
        new_state.geodes += state.geode_production * ticks;
        new_state.geode_production += 1;
        best = best.max(solve(&new_state, costs, max_tick, best, cache));
    }

    if state.ore < costs.ore_costs_geode || state.obsidian < costs.obsidian_costs_geode {
        if state.clay_production > 0 && costs.obsidian_costs_geode > state.obsidian_production {
            let ore_ticks = (costs.ore_costs_obsidian - state.ore.min(costs.ore_costs_obsidian))
                .div_ceil(state.ore_production);
            let clay_ticks = (costs.clay_costs_obsidian
                - state.clay.min(costs.clay_costs_obsidian))
            .div_ceil(state.clay_production);
            let ticks = ore_ticks.max(clay_ticks) + 1;
            let mut new_state = state.clone();
            new_state.tick += ticks;
            new_state.ore += state.ore_production * ticks;
            new_state.ore -= costs.ore_costs_obsidian;
            new_state.clay += state.clay_production * ticks;
            new_state.clay -= costs.clay_costs_obsidian;
            new_state.obsidian += state.obsidian_production * ticks;
            new_state.geodes += state.geode_production * ticks;
            new_state.obsidian_production += 1;
            best = best.max(solve(&new_state, costs, max_tick, best, cache));
        }
        if costs.clay_costs_obsidian > state.clay_production {
            let ticks = 1
                + (costs.ore_costs_clay - state.ore.min(costs.ore_costs_clay))
                    .div_ceil(state.ore_production);
            let mut new_state = state.clone();
            new_state.tick += ticks;
            new_state.ore += state.ore_production * ticks;
            new_state.ore -= costs.ore_costs_clay;
            new_state.clay += state.clay_production * ticks;
            new_state.obsidian += state.obsidian_production * ticks;
            new_state.geodes += state.geode_production * ticks;
            new_state.clay_production += 1;
            best = best.max(solve(&new_state, costs, max_tick, best, cache));
        }
        if costs
            .ore_costs_geode
            .max(costs.ore_costs_obsidian.max(costs.ore_costs_clay))
            > state.ore_production
        {
            let ticks = 1
                + (costs.ore_costs_ore - state.ore.min(costs.ore_costs_ore))
                    .div_ceil(state.ore_production);
            let mut new_state = state.clone();
            new_state.tick += ticks;
            new_state.ore += state.ore_production * ticks;
            new_state.ore -= costs.ore_costs_ore;
            new_state.clay += state.clay_production * ticks;
            new_state.obsidian += state.obsidian_production * ticks;
            new_state.geodes += state.geode_production * ticks;
            new_state.ore_production += 1;
            best = best.max(solve(&new_state, costs, max_tick, best, cache));
        }
    }

    cache.insert(state.clone(), best);

    best
}
