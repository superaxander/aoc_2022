use std::cmp::Ordering;
use anyhow::Result;
use std::collections::{BinaryHeap, BTreeSet, HashMap, HashSet};
use std::collections::hash_map::Entry;
use bit_vec::BitVec;
use itertools::Itertools;

use crate::common;

#[derive(Eq, PartialEq, Debug)]
struct Node<const N: usize>(usize, usize, [usize; N], BitVec);

impl<const N: usize> PartialOrd<Node<N>> for Node<N> {
    fn partial_cmp(&self, other: &Node<N>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<const N: usize> Ord for Node<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/test.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;

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
            let destinations = list.split(", ").map(|s| s.to_owned()).collect::<Vec<String>>();
            valve_names.insert(id, valves.len());
            valves.push((flow_rate, destinations));
        }
    }
    let valves: Vec<(usize, Vec<usize>)> = valves.into_iter().map(|(flow_rate, destinations)| (flow_rate, destinations.into_iter().map(|d| valve_names[&d]).collect())).collect();
    let first_valve = valve_names["AA"];
    let mut frontier = BinaryHeap::new();
    let mut g_scores: HashMap<(usize, usize, BitVec), usize> = HashMap::new();
    let potential = valves.iter().map(|(flow, _)| *flow * 30).sum::<usize>();
    frontier.push(Node(potential, 1, [first_valve], BitVec::from_elem(valves.len(), false)));
    g_scores.insert((1, first_valve, BitVec::from_elem(valves.len(), false)), 0);
    while let Some(Node(_, minute, [valve], opened)) = frontier.pop() {
        let our_score = g_scores[&(minute, valve, opened.clone())];
        if minute == 30 {
            solution_a = our_score;
            break;
        }

        let (flow_rate, destinations) = &valves[valve];

        if minute < 29 && !opened[valve] {
            let new_score = our_score + flow_rate * (30 - minute);
            let mut new_opened = opened.clone();
            new_opened.set(valve, true);
            let potential = new_opened.iter().enumerate().filter(|(_, b)| !*b).map(|(i, _)| valves[i].0 * (28 - minute)).sum::<usize>() / 2;
            let new_entry = Node(new_score + potential, minute + 1, [valve], new_opened.clone());
            match g_scores.entry((minute + 1, valve, new_opened.clone())) {
                Entry::Occupied(mut e) =>
                    if *e.get() < new_score {
                        e.insert(new_score);
                        frontier.push(new_entry);
                    }
                Entry::Vacant(e) => {
                    e.insert(new_score);
                    frontier.push(new_entry);
                }
            }
        }

        let potential = opened.iter().enumerate().filter(|(_, b)| !*b).map(|(i, _)| valves[i].0 * (29 - minute)).sum::<usize>() / 2;
        for destination in destinations {
            let new_entry = Node(our_score + potential, minute + 1, [*destination], opened.clone());
            match g_scores.entry((minute + 1, *destination, opened.clone())) {
                Entry::Occupied(mut e) => if *e.get() < our_score {
                    e.insert(our_score);
                    frontier.push(new_entry);
                }
                Entry::Vacant(e) => {
                    e.insert(our_score);
                    frontier.push(new_entry);
                }
            }
        }
    }

    let mut frontier = BinaryHeap::new();
    let mut g_scores: HashMap<(usize, [usize; 2], BitVec), usize> = HashMap::new();
    let potential = valves.iter().map(|(flow, _)| *flow * 26).sum::<usize>();
    frontier.push(Node(potential, 5, [first_valve, first_valve], BitVec::from_elem(valves.len(), false)));
    g_scores.insert((5, [first_valve, first_valve], BitVec::from_elem(valves.len(), false)), 0);
    while let Some(Node(_, minute, players, opened)) = frontier.pop() {
        println!("{g_scores:?}");
        let our_score = g_scores[&(minute, players, opened.clone())];
        if minute == 30 {
            solution_b = our_score;
            break;
        }


        let mut entries = [Vec::new(), Vec::new()];
        for i in 0..2 {
            let valve = players[i];
            let (flow_rate, destinations) = &valves[valve];
            if minute < 29 && !opened[valve] {
                let new_score = flow_rate * (30 - minute);
                let mut new_opened = opened.clone();
                new_opened.set(valve, true);
                entries[i].push((new_score, valve, new_opened))
            }
            
            for destination in destinations {
                entries[i].push((0, *destination, opened.clone()))
            }
        }
        let [entries_a, entries_b] = entries;
        for ((score_a, destination_a, opened_a), (score_b, destination_b, opened_b)) in entries_a.into_iter().cartesian_product(entries_b.into_iter()) {
            let mut orred = opened_a;
            orred.or(&opened_b);
            let potential = orred.iter().enumerate().filter(|(_, b)| !*b).map(|(i, _)| valves[i].0 * (28 - minute.min(28))).sum::<usize>() / 2;
            let entry = Node(our_score + potential + score_a + score_b, minute + 1, [destination_a, destination_b], orred.clone());
            match g_scores.entry((minute + 1, [destination_a, destination_b], orred))) {
                Entry::Occupied(mut e) => if *e.get() < our_score {
                    e.insert(our_score + score_a + score_b);
                    frontier.push(entry);
                }
                Entry::Vacant(e) => {
                    e.insert(our_score + score_a + score_b);
                    frontier.push(entry);
                }
            }
        }
        
    }
    Ok((solution_a, solution_b))
}
