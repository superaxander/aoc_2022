use std::cmp::Ordering;
use anyhow::Result;
use std::collections::{BinaryHeap, BTreeSet, HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::process::exit;
use std::sync::atomic::AtomicUsize;
use std::time::Instant;
use bit_vec::BitVec;
use itertools::Itertools;
use rayon::prelude::*;

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

#[derive(Eq, PartialEq, Debug)]
struct Node2<const N: usize>(usize, usize, usize, BitVec);

impl<const N: usize> PartialOrd<Self> for Node2<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<const N: usize> Ord for Node2<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/16.txt")?;
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
    let mut valves: Vec<(usize, Vec<usize>)> = valves.into_iter().map(|(flow_rate, destinations)| (flow_rate, destinations.into_iter().map(|d| valve_names[&d]).collect())).collect();
    let distances = shortest_distances(&valves);
    let valves = valves.into_iter().map(|(flow_rate,_)|flow_rate).collect::<Vec<usize>>();
    let first_valve = valve_names["AA"];
    let filtered = (0..valves.len()).filter(|i| valves[*i] > 0).collect::<Vec<usize>>();
    
    superluminal_perf::begin_event("Part a");
    let solution_a = solve((first_valve, 1, BitVec::from_elem(filtered.len(), false)), &distances, &valves,&filtered, &mut HashMap::new(), false, first_valve);
    let solution_b = solve((first_valve, 5, BitVec::from_elem(filtered.len(), false)), &distances, &valves,&filtered, &mut HashMap::new(), true, first_valve);
    // let mut frontier = BinaryHeap::new();
    // let mut g_scores: HashMap<(usize, usize, BitVec), usize> = HashMap::new();
    // let potential = valves.iter().map(|(flow, _)| *flow * 30).sum::<usize>();
    // frontier.push(Node(potential, 1, [first_valve], BitVec::from_elem(valves.len(), false)));
    // g_scores.insert((1, first_valve, BitVec::from_elem(valves.len(), false)), 0);
    // while let Some(Node(_, minute, [valve], opened)) = frontier.pop() {
    //     let our_score = g_scores[&(minute, valve, opened.clone())];
    //     if minute == 30 {
    //         solution_a = our_score;
    //         break;
    //     }
    // 
    //     let (flow_rate, destinations) = &valves[valve];
    // 
    //     if minute < 29 && !opened[valve] {
    //         let new_score = our_score + flow_rate * (30 - minute);
    //         let mut new_opened = opened.clone();
    //         new_opened.set(valve, true);
    //         let potential = new_opened.iter().enumerate().filter(|(_, b)| !*b).map(|(i, _)| valves[i].0 * (28 - minute)).sum::<usize>() / 2;
    //         let new_entry = Node(new_score + potential, minute + 1, [valve], new_opened.clone());
    //         match g_scores.entry((minute + 1, valve, new_opened.clone())) {
    //             Entry::Occupied(mut e) =>
    //                 if *e.get() < new_score {
    //                     e.insert(new_score);
    //                     frontier.push(new_entry);
    //                 }
    //             Entry::Vacant(e) => {
    //                 e.insert(new_score);
    //                 frontier.push(new_entry);
    //             }
    //         }
    //     }
    // 
    //     let potential = opened.iter().enumerate().filter(|(_, b)| !*b).map(|(i, _)| valves[i].0 * (29 - minute)).sum::<usize>() / 2;
    //     for destination in destinations {
    //         let new_entry = Node(our_score + potential, minute + 1, [*destination], opened.clone());
    //         match g_scores.entry((minute + 1, *destination, opened.clone())) {
    //             Entry::Occupied(mut e) => if *e.get() < our_score {
    //                 e.insert(our_score);
    //                 frontier.push(new_entry);
    //             }
    //             Entry::Vacant(e) => {
    //                 e.insert(our_score);
    //                 frontier.push(new_entry);
    //             }
    //         }
    //     }
    // }
    // superluminal_perf::end_event();
    // let mut start = Instant::now();
    // let mut count = 0;
    // let mut scores = HashMap::new();
    // let solution_b = filtered.iter().copied().permutations(5).map(|you| {
    //     if scores.contains_key(&you) {
    //         return scores[&you];
    //     }
    //     let mut you = &you[0..];
    //     let mut last = first_valve;
    //     let mut distance = 4;
    //     let mut sum = 0;
    //     for i in 0..you.len() {
    //         distance += distances[last][you[i]] + 1;
    //         if distance > 30 {
    //             scores.insert(you.to_vec(), sum);
    //             you = &you[..i];
    //             break;
    //         }
    //         sum += valves[you[i]].0 * (30 - distance);
    //         last = you[i];
    //     }
    //     
    //     sum += filtered.iter().copied().filter(|i|!you.contains(i)).permutations(5).map(|elephant| {
    //         if scores.contains_key(&elephant) {
    //             return scores[&elephant];
    //         }
    //         let mut sum = 0;
    //         distance = 4;
    //         last = first_valve;
    //         for i in 0..elephant.len() {
    //             distance += distances[last][elephant[i]] + 1;
    //             if distance > 30 {
    //                 break;
    //             }
    //             sum += valves[elephant[i]].0 * (30 - distance);
    //             last = elephant[i];
    //         }
    //         scores.insert(elephant.to_vec(), sum);
    //         sum
    //     }).max().unwrap();
    //     
    //     count += 1;
    //     if count % 1000 == 0 {
    //         let elapsed = start.elapsed().as_secs_f64();
    //         println!("{} p/s", 1000.0 / elapsed);
    //         start = Instant::now();
    //     } 
    //     sum
    // }).max().unwrap();
    // 
    // println!("{:?}", g_scores.iter().max_by_key(|(_,v)| **v));

    // let mut frontier = BinaryHeap::new();
    // let mut g_scores: HashMap<(usize, [usize; 2], BitVec), usize> = HashMap::new();
    // let potential = valves.iter().map(|(flow, _)| *flow * 26).sum::<usize>();
    // frontier.push(Node(potential, 5, [first_valve, first_valve], BitVec::from_elem(valves.len(), false)));
    // g_scores.insert((5, [first_valve, first_valve], BitVec::from_elem(valves.len(), false)), 0);
    // while let Some(Node(_, minute, players, opened)) = frontier.pop() {
    //     let our_score = g_scores[&(minute, players, opened.clone())];
    //     if minute == 30 {
    //         solution_b = our_score;
    //         break;
    //     }
    // 
    // 
    //     let mut entries = [Vec::new(), Vec::new()];
    //     let mut newly_opened = None;
    //     for i in 0..2 {
    //         let valve = players[i];
    //         let (flow_rate, destinations) = &valves[valve];
    //         if !opened[valve] && newly_opened != Some(valve) {
    //             let new_score = flow_rate * (30 - minute);
    //             let mut new_opened = opened.clone();
    //             new_opened.set(valve, true);
    //             newly_opened = Some(valve);
    //             entries[i].push((new_score, valve, new_opened))
    //         }
    //         
    //         for destination in destinations {
    //             entries[i].push((0, *destination, opened.clone()))
    //         }
    //     }
    //     let [entries_a, entries_b] = entries;
    //     for ((score_a, destination_a, opened_a), (score_b, destination_b, opened_b)) in entries_a.into_iter().cartesian_product(entries_b.into_iter()) {
    //         let mut orred = opened_a;
    //         orred.or(&opened_b);
    //         let potential = orred.iter().enumerate().filter(|(_, b)| !*b).map(|(i, _)| valves[i].0 * (29 - minute)).sum::<usize>();
    //         let entry = Node(our_score + potential + score_a + score_b, minute + 1, [destination_a, destination_b], orred.clone());
    //         match g_scores.entry((minute + 1, [destination_a, destination_b], orred)) {
    //             Entry::Occupied(mut e) => if *e.get() < our_score {
    //                 e.insert(our_score + score_a + score_b);
    //                 frontier.push(entry);
    //             }
    //             Entry::Vacant(e) => {
    //                 e.insert(our_score + score_a + score_b);
    //                 frontier.push(entry);
    //             }
    //         }
    //     }
    //     
    // }
    Ok((solution_a, solution_b))
}

type State = (usize, usize, BitVec);

fn solve((position, minute, mut opened): State, distances: &[Vec<usize>], valves: &[usize], filtered: &[usize], cache: &mut HashMap<State, usize>,  first_pass: bool, first_valve: usize) -> usize {
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
            let recursive_sum = solve((valve, minute + dist, opened.clone()), distances, valves, filtered, cache, first_pass, first_valve) + (31 - minute - dist) * valves[valve];
            opened.set(i, false);
            if recursive_sum > sum {
                sum = recursive_sum;
            }
        }
    }
    if first_pass {
        let elephant_sum = solve((first_valve, 5, opened), distances, valves, filtered, cache, false, first_valve);
        if elephant_sum > sum { return elephant_sum; }
    } else {
        cache.insert(state, sum);
    }
    
    sum
}

fn shortest_distances(valves: &Vec<(usize, Vec<usize>)>) -> Vec<Vec<usize>> {
    let mut distances = vec![vec![usize::MAX - 1 ; valves.len()] ; valves.len()];
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
