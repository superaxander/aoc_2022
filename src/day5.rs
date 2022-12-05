use anyhow::Result;

use crate::common;

pub fn main() -> Result<(String, String)> {
    let lines = common::read_lines("inputs/5.txt")?;

    let mut columns_a: Vec<Vec<String>> = Vec::new();
    let mut columns_b: Vec<Vec<String>> = Vec::new();

    let mut command_mode = false;
    for line in lines {
        let line = line?;
        if command_mode {
            let (count, loc) = line[5..].split_once(" from ").expect("Invalid_string format");
            let (src, dst) = loc.split_once(" to ").expect("Invalid string format");
            let count = count.parse::<usize>()?;
            let src = src.parse::<usize>()? - 1;
            let dst = dst.parse::<usize>()? - 1;

            for _ in 0..count {
                // Take top-most element
                let element = columns_a[src].iter_mut().find(|s| !s.trim().is_empty()).expect("Column empty");
                let string = element.to_owned();
                *element = "   ".to_owned();

                // Find bottom-most empty space
                if let Some(element) = columns_a[dst].iter_mut().filter(|s| s.trim().is_empty()).last() {
                    *element = string;
                } else {
                    columns_a[dst].insert(0, string);
                }
            }

            // Take top-most elements
            let src_elements = columns_b[src].iter_mut().filter(|s| !s.trim().is_empty()).take(count).collect::<Vec<&mut String>>();
            let mut elements = Vec::new();
            for element in src_elements {
                elements.push(element.to_owned());
                *element = "   ".to_owned();
            }

            // Find bottom-most empty spaces
            let mut dst_elements = columns_b[dst].iter_mut().filter(|s| s.trim().is_empty()).rev().take(count).collect::<Vec<&mut String>>();
            let expansion_size = count - dst_elements.len();
            for (i, element) in dst_elements.iter_mut().rev().enumerate() {
                **element = elements[i + expansion_size].to_owned();
            }
            // Expand columns if necessary
            for i in (0..expansion_size).rev() {
                columns_b[dst].insert(0, elements[i].to_owned());
            }
        } else if line.is_empty() {
            command_mode = true;
            columns_a.iter_mut().for_each(|c| { c.pop(); });
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
                    columns_a[i].push(l.to_owned());
                } else {
                    columns_a.push(vec![l.to_owned()])
                }
            }
        }
    }

    let solution_a = columns_a.iter().map(|c| &c.iter().find(|s| !s.trim().is_empty()).unwrap()[1..2]).collect::<String>();
    let solution_b = columns_b.iter().map(|c| &c.iter().find(|s| !s.trim().is_empty()).unwrap()[1..2]).collect::<String>();

    Ok((solution_a, solution_b))
}
