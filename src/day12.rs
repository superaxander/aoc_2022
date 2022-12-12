use anyhow::Result;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use crate::common;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Location {
    idx: usize,
    cost: usize,
}

impl Location {
    fn new(idx: usize, cost: usize) -> Location {
        Location { idx, cost }
    }
}

impl PartialOrd<Location> for Location {
    fn partial_cmp(&self, other: &Location) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Location) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/12.txt")?;

    let mut heights = Vec::new();
    let mut start = 0;
    let mut goal = 0;
    let mut width = 0;

    for line in lines {
        let line = line?;
        width = line.len();
        let start_idx = heights.len();
        heights.extend(line.chars().enumerate().map(|(i, c)| match c {
            'a'..='z' => c as usize - 'a' as usize,
            'S' => {
                goal = start_idx + i;
                0
            }
            'E' => {
                start = start_idx + i;
                25
            }
            _ => panic!(),
        }));
    }

    let height = heights.len() / width;

    let goals = heights
        .iter()
        .enumerate()
        .filter(|(_, h)| **h == 0)
        .map(|(idx, _)| idx)
        .collect();
    let scores = find_shortest_paths(&heights, start, &goals, width, height);

    let solution_a = scores[goal];
    let solution_b = goals.iter().map(|idx| scores[*idx]).min().unwrap();

    Ok((solution_a, solution_b))
}

fn find_shortest_paths(
    heights: &Vec<usize>,
    start: usize,
    goals: &HashSet<usize>,
    width: usize,
    height: usize,
) -> Vec<usize> {
    let mut found_goals = HashSet::new();
    let mut frontier = BinaryHeap::new();
    let mut g_scores = vec![usize::MAX; heights.len()];
    frontier.push(Location::new(start, 0));

    while let Some(location) = frontier.pop() {
        let idx = location.idx;
        let cost = location.cost;
        if goals.contains(&idx) && found_goals.insert(idx) && found_goals.len() == goals.len() {
            return g_scores;
        }
        let elevation = heights[idx];
        if idx > width {
            add_direction(
                heights,
                &mut frontier,
                &mut g_scores,
                idx - width,
                cost + 1,
                elevation,
            );
        }
        if location.idx % width > 0 {
            add_direction(
                heights,
                &mut frontier,
                &mut g_scores,
                idx - 1,
                cost + 1,
                elevation,
            );
        }
        if location.idx / width < height - 1 {
            add_direction(
                heights,
                &mut frontier,
                &mut g_scores,
                idx + width,
                cost + 1,
                elevation,
            );
        }
        if location.idx % width < width - 1 {
            add_direction(
                heights,
                &mut frontier,
                &mut g_scores,
                idx + 1,
                cost + 1,
                elevation,
            );
        }
    }
    g_scores
}

fn add_direction(
    heights: &[usize],
    frontier: &mut BinaryHeap<Location>,
    g_scores: &mut [usize],
    idx: usize,
    cost: usize,
    elevation: usize,
) -> bool {
    if (elevation < heights[idx] || elevation - heights[idx] <= 1) && g_scores[idx] > cost {
        g_scores[idx] = cost;
        frontier.push(Location { idx, cost });
        true
    } else {
        false
    }
}
