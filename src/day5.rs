use anyhow::Result;

use crate::common;

pub fn main() -> Result<(String, String)> {
    let lines = common::read_lines("inputs/5.txt")?;

    let mut columns_a: Vec<Vec<String>> = Vec::new();
    let mut columns_b: Vec<Vec<String>> = Vec::new();

    let mut parse_commands = false;
    for line in lines {
        let line = line?;
        if parse_commands {
            let (count, loc) = line[5..].split_once(" from ").expect("Invalid_string format");
            let (src, dst) = loc.split_once(" to ").expect("Invalid string format");
            let count = count.parse::<usize>()?;
            let src = src.parse::<usize>()? - 1;
            let dst = dst.parse::<usize>()? - 1;

            // Part a
            for _ in 0..count {
                let value = columns_a[src].pop().expect("Column was empty");
                columns_a[dst].push(value);
            }
            
            // Part b
            let split_index = columns_b[src].len() - count;
            let src_elements = columns_b[src].split_off(split_index);
            columns_b[dst].extend(src_elements);
        } else if line.is_empty() {
            parse_commands = true;
            columns_a.iter_mut().for_each(|c| { c.pop(); c.reverse(); });
            columns_b = columns_a.clone();
        } else {
            let width = (line.len() + 1) / 4;
            let mut left = line.as_str();
            for i in 0..width {
                let (l, r) = left.split_at(3);
                if !r.is_empty() {
                    left = &r[1..];
                }
                if i < columns_a.len() {
                    if !l.trim().is_empty() {
                        columns_a[i].push(l.to_owned());
                    }
                } else if l.trim().is_empty() {
                    columns_a.push(vec![])
                }else {
                    columns_a.push(vec![l.to_owned()])
                }
            }
        }
    }

    let solution_a = columns_a.iter().map(|c| &c.last().unwrap()[1..2]).collect::<String>();
    let solution_b = columns_b.iter().map(|c| &c.last().unwrap()[1..2]).collect::<String>();

    Ok((solution_a, solution_b))
}
