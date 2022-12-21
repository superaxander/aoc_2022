use anyhow::Result;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/20.txt")?;

    // Could be improved further with memmoves (by removing at index and copying all between on index over)
    let mut numbers: Vec<_> = lines
        .into_iter()
        .filter_map(|l| l.ok()?.parse().ok())
        .enumerate()
        .collect();

    let len = numbers.len();
    let solution_a = {
        let mut numbers = numbers.clone();
        let mut original_indices: Vec<usize> = (0..len).collect();
        mix(&mut numbers, &mut original_indices);
        calculate_solution(&numbers)
    };
    let solution_b = {
        numbers.iter_mut().for_each(|(_, n)| *n *= 811_589_153);
        let mut original_indices: Vec<usize> = (0..len).collect();
        for _ in 0..10 {
            mix(&mut numbers, &mut original_indices);
        }
        calculate_solution(&numbers)
    };

    Ok((solution_a, solution_b))
}

fn calculate_solution(numbers: &[(usize, i64)]) -> i64 {
    let len = numbers.len();
    let zero_index = numbers.iter().position(|(_, n)| *n == 0).unwrap();
    let n1000 = numbers[(zero_index + 1000) % len].1;
    let n2000 = numbers[(zero_index + 2000) % len].1;
    let n3000 = numbers[(zero_index + 3000) % len].1;
    n1000 + n2000 + n3000
}

#[allow(clippy::cast_possible_truncation)]
fn mix(numbers: &mut [(usize, i64)], original_indices: &mut [usize]) {
    let len = numbers.len();
    for idx in 0..len {
        let i = original_indices[idx];
        let (_, n) = numbers[i];

        let n = n.rem_euclid(len as i64 - 1);

        let sign = n.signum();
        for offset in 1..=n.abs() {
            let old = (i as i64 + sign * (offset - 1)).rem_euclid(len as i64) as usize;
            let new = (i as i64 + sign * (offset)).rem_euclid(len as i64) as usize;
            original_indices.swap(numbers[old].0, numbers[new].0);
            numbers.swap(old, new);
        }
    }
}
