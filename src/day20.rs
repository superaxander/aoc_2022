use anyhow::Result;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/20.txt")?;

    let mut numbers: Vec<_> = lines
        .into_iter()
        .filter_map(|l| l.ok()?.parse().ok())
        .collect();

    let len = numbers.len();
    let solution_a = {
        let mut numbers = numbers.clone();
        let mut indices: Vec<(usize, i64)> = numbers.iter().copied().enumerate().collect();
        let mut original_indices: Vec<usize> = (0..len).collect();
        mix(&mut numbers, &mut indices, &mut original_indices);
        calculate_solution(&numbers)
    };
    let solution_b = {
        numbers.iter_mut().for_each(|n| *n *= 811589153);
        let mut indices: Vec<(usize, i64)> = numbers.iter().copied().enumerate().collect();
        let mut original_indices: Vec<usize> = (0..len).collect();
        for _ in 0..10 {
            mix(&mut numbers, &mut indices, &mut original_indices);
        }
        calculate_solution(&numbers)
    };

    Ok((solution_a, solution_b))
}

fn calculate_solution(numbers: &[i64]) -> i64 {
    let len = numbers.len();
    let zero_index = numbers.iter().position(|n| *n == 0).unwrap();
    let n1000 = numbers[(zero_index + 1000) % len];
    let n2000 = numbers[(zero_index + 2000) % len];
    let n3000 = numbers[(zero_index + 3000) % len];
    n1000 + n2000 + n3000
}

fn mix(numbers: &mut [i64], indices: &mut [(usize, i64)], original_indices: &mut [usize]) {
    let len = numbers.len();
    for idx in 0..len {
        let (i, n) = indices[idx];

        let n = n.rem_euclid(len as i64 - 1);

        let sign = n.signum();
        for offset in 1..=n.abs() {
            let old = (i as i64 + sign * (offset - 1)).rem_euclid(len as i64) as usize;
            let new = (i as i64 + sign * (offset)).rem_euclid(len as i64) as usize;
            let rev_index = original_indices[old];
            indices[rev_index].0 = (indices[rev_index].0 + len + sign as usize) % len;
            let rev_index = original_indices[new];
            indices[rev_index].0 = (indices[rev_index].0 + len - sign as usize) % len;
            original_indices.swap(old, new);
            numbers.swap(old, new);
        }
    }
}
