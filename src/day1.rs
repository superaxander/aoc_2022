use anyhow::Result;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    // One-liner:
    // let solution_b = BufReader::new(File::open("inputs/1.txt")?).lines().map(Result::unwrap).collect::<Vec<String>>().split(String::is_empty).map(|e|e.iter().map(|l|l.parse::<i64>().unwrap()).sum::<i64>()).collect::<BinaryHeap<_>>().drain_sorted().take(3).sum());

    let lines = common::read_lines("inputs/1.txt")?;
    let mut elf_1 = 0;
    let mut elf_2 = 0;
    let mut elf_3 = 0;

    let mut current_elf = 0;

    for line in lines {
        if let Ok(num) = line?.trim().parse::<i64>() {
            current_elf += num;
        } else {
            if current_elf > elf_1 {
                elf_3 = elf_2;
                elf_2 = elf_1;
                elf_1 = current_elf;
            } else if current_elf > elf_2 {
                elf_3 = elf_2;
                elf_2 = current_elf;
            } else if current_elf > elf_3 {
                elf_3 = current_elf;
            }
            current_elf = 0;
        }
    }

    if current_elf > elf_1 {
        elf_3 = elf_2;
        elf_2 = elf_1;
        elf_1 = current_elf;
    } else if current_elf > elf_2 {
        elf_3 = elf_2;
        elf_2 = current_elf;
    } else if current_elf > elf_3 {
        elf_3 = current_elf;
    }

    Ok((elf_1, elf_1 + elf_2 + elf_3))
}
