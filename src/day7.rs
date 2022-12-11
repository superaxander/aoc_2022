use anyhow::Result;
use std::collections::HashMap;

use crate::common;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/7.txt")?;

    let mut directories = HashMap::new();
    directories.insert(String::new(), HashMap::new());
    let mut current_path = String::new();

    for line in lines {
        let line = line?;
        if line.starts_with('$') {
            if let Some(("cd", argument)) = line[2..].split_once(' ') {
                match argument {
                    "/" => current_path.clear(),
                    ".." => {
                        while !current_path.ends_with('/') {
                            current_path.pop();
                        }
                        current_path.pop();
                    }
                    _ => {
                        current_path.push('/');
                        current_path.push_str(argument);
                    }
                }
            }
        } else {
            let (size, name) = line
                .split_once(' ')
                .expect("ls output lines need to consist of two parts");
            if size == "dir" {
                let full_name = [&current_path, "/", name].concat();
                directories.entry(full_name).or_insert_with(HashMap::new);
            } else {
                directories
                    .entry(current_path.clone())
                    .or_insert_with(HashMap::new)
                    .insert(name.to_owned(), size.parse::<usize>()?);
            }
        }
    }

    let mut sizes = Vec::new();
    get_directory_sizes(&directories, "", &mut sizes);
    let solution_a = sizes.iter().filter(|n| **n <= 100_000).sum();

    let remaining_space = 70_000_000 - sizes.last().unwrap();
    let to_delete = 30_000_000 - remaining_space;
    let solution_b = *sizes.iter().filter(|n| **n >= to_delete).min().unwrap();

    Ok((solution_a, solution_b))
}

fn get_directory_sizes(
    directories: &HashMap<String, HashMap<String, usize>>,
    path: &str,
    sizes: &mut Vec<usize>,
) -> usize {
    let mut size = 0;
    size += directories[path].values().sum::<usize>();

    let prefix = [path, "/"].concat();
    size += directories
        .keys()
        .filter(|k| {
            if let Some(index) = k.rfind('/') {
                prefix == k[..=index]
            } else {
                false
            }
        })
        .map(|k| get_directory_sizes(directories, k, sizes))
        .sum::<usize>();

    sizes.push(size);

    size
}
