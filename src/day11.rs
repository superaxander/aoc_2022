use anyhow::{Context, Result};

use crate::common;

#[derive(Copy, Clone)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Copy, Clone)]
enum Operand {
    Num(usize),
    Old,
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn main(do_b: bool) -> Result<usize> {
    let mut lines = common::read_lines("inputs/11.txt")?;

    let mut monkey_items: Vec<Vec<usize>> = Vec::new();
    let mut monkey_parameters: Vec<(Operation, Operand, usize, usize, usize)> = Vec::new();

    while let Some(Ok(_)) = lines.next() {
        let line = lines.next().context("Invalid format")??;
        let (_, item_list) = line.split_once(": ").context("Invalid format")?;
        monkey_items.push(
            item_list
                .split(", ")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect(),
        );
        let line = lines.next().context("Invalid format")??;
        let (_, operation) = line.split_once("old ").context("Invalid format")?;
        let (operation, operand) = operation.split_once(' ').context("Invalid format")?;
        let operand = if operand == "old" {
            Operand::Old
        } else {
            Operand::Num(operand.parse()?)
        };
        let line = lines.next().context("Invalid format")??;
        let (_, divisibility) = line.split_once("by ").context("Invalid format")?;
        let divisor = divisibility.parse()?;
        let line = lines.next().context("Invalid format")??;
        let (_, true_target) = line.split_once("monkey ").context("Invalid format")?;
        let true_target = true_target.parse()?;
        let line = lines.next().context("Invalid format")??;
        let (_, false_target) = line.split_once("monkey ").context("Invalid format")?;
        let false_target = false_target.parse()?;
        if operation == "+" {
            monkey_parameters.push((Operation::Add, operand, divisor, true_target, false_target));
        } else {
            monkey_parameters.push((
                Operation::Multiply,
                operand,
                divisor,
                true_target,
                false_target,
            ));
        }
        lines.next();
    }

    let lcm = monkey_parameters
        .iter()
        .map(|(_, _, divisor, _, _)| divisor)
        .copied()
        .reduce(lcm)
        .unwrap();

    let mut monkey_inspections: Vec<usize> = vec![0; monkey_items.len()];
    let mut true_items = Vec::new();
    let mut false_items = Vec::new();

    for _ in 0..if do_b { 10000 } else { 20 } {
        for monkey in 0..monkey_items.len() {
            let (operator, operand, divisor, true_target, false_target) = monkey_parameters[monkey];
            for item in monkey_items[monkey].drain(0..) {
                monkey_inspections[monkey] += 1;
                let mut worry_level = item;
                match (operator, operand) {
                    (Operation::Add, Operand::Num(operand)) => worry_level += operand,
                    (Operation::Multiply, Operand::Num(operand)) => worry_level *= operand,
                    (Operation::Add, Operand::Old) => worry_level += worry_level,
                    (Operation::Multiply, Operand::Old) => worry_level *= worry_level,
                }
                if !do_b {
                    worry_level /= 3;
                }
                if worry_level % divisor == 0 {
                    true_items.push(worry_level % lcm);
                } else {
                    false_items.push(worry_level % lcm);
                }
            }
            monkey_items[true_target].extend(true_items.drain(0..));
            monkey_items[false_target].extend(false_items.drain(0..));
        }
    }
    monkey_inspections.sort_unstable();
    let solution_a = monkey_inspections[monkey_inspections.len() - 2..]
        .iter()
        .product();

    Ok(solution_a)
}
