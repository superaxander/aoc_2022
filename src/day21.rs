use anyhow::Result;
use std::collections::HashMap;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/21.txt")?;

    let mut dependencies: HashMap<String, (String, char, String)> = HashMap::new();
    let mut known: HashMap<String, i64> = HashMap::new();

    for line in lines {
        let line = line?;
        let (monkey, rest) = line.split_once(": ").unwrap();
        if rest.contains(' ') {
            let (lhs, rest) = rest.split_once(' ').unwrap();
            let (operator, rhs) = rest.split_once(' ').unwrap();
            dependencies.insert(
                monkey.to_owned(),
                (
                    lhs.to_owned(),
                    operator.chars().next().unwrap(),
                    rhs.to_owned(),
                ),
            );
        } else {
            let number = rest.parse()?;
            known.insert(monkey.to_owned(), number);
        }
    }

    let solution_a = find(&dependencies["root"], &dependencies, &known);

    let lhs = find_with_unknown(
        &dependencies[&dependencies["root"].0],
        &dependencies,
        &known,
    );
    let rhs = find_with_unknown(
        &dependencies[&dependencies["root"].2],
        &dependencies,
        &known,
    );
    assert!(matches!(lhs, Variable::Known(_)) || matches!(rhs, Variable::Known(_)));
    let solution_b = find_x(lhs, rhs);

    Ok((solution_a, solution_b))
}

fn find_x(lhs: Variable, rhs: Variable) -> i64 {
    match (lhs, rhs) {
        (Variable::Known(_), Variable::Known(_)) => panic!("Didn't find X"),
        (Variable::X, Variable::Known(other)) | (Variable::Known(other), Variable::X) => other,
        (Variable::Unknown(t, o), Variable::Known(other))
        | (Variable::Known(other), Variable::Unknown(t, o)) => match o {
            '+' => match *t {
                (Variable::Known(lhs), rhs) => find_x(rhs, Variable::Known(other - lhs)),
                (lhs, Variable::Known(rhs)) => find_x(lhs, Variable::Known(other - rhs)),
                _ => panic!("Can't compute two unknowns"),
            },
            '-' => match *t {
                (Variable::Known(lhs), rhs) => find_x(rhs, Variable::Known(lhs - other)),
                (lhs, Variable::Known(rhs)) => find_x(lhs, Variable::Known(rhs + other)),
                _ => panic!("Can't compute two unknowns"),
            },
            '/' => match *t {
                (Variable::Known(lhs), rhs) => find_x(rhs, Variable::Known(lhs / other)),
                (lhs, Variable::Known(rhs)) => find_x(lhs, Variable::Known(rhs * other)),
                _ => panic!("Can't compute two unknowns"),
            },
            '*' => match *t {
                (Variable::Known(lhs), rhs) => find_x(rhs, Variable::Known(other / lhs)),
                (lhs, Variable::Known(rhs)) => find_x(lhs, Variable::Known(other / rhs)),
                _ => panic!("Can't compute two unknowns"),
            },
            _ => panic!("Unknown operation"),
        },
        (Variable::X, Variable::X) => 0,
        (Variable::Unknown(_, _) | Variable::X, Variable::Unknown(_, _) | Variable::X) => {
            panic!("Can't compute two unknowns")
        }
    }
}

fn find(
    operation: &(String, char, String),
    dependencies: &HashMap<String, (String, char, String)>,
    known: &HashMap<String, i64>,
) -> i64 {
    let lhs = if known.contains_key(&operation.0) {
        known[&operation.0]
    } else {
        find(&dependencies[&operation.0], dependencies, known)
    };

    let rhs = if known.contains_key(&operation.2) {
        known[&operation.2]
    } else {
        find(&dependencies[&operation.2], dependencies, known)
    };

    match operation.1 {
        '+' => lhs + rhs,
        '-' | '=' => lhs - rhs,
        '/' => lhs / rhs,
        '*' => lhs * rhs,
        _ => panic!(),
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Variable {
    Known(i64),
    Unknown(Box<(Variable, Variable)>, char),
    X,
}

fn find_with_unknown(
    operation: &(String, char, String),
    dependencies: &HashMap<String, (String, char, String)>,
    known: &HashMap<String, i64>,
) -> Variable {
    let lhs = if operation.0 == "humn" {
        Variable::X
    } else if known.contains_key(&operation.0) {
        Variable::Known(known[&operation.0])
    } else {
        find_with_unknown(&dependencies[&operation.0], dependencies, known)
    };

    let rhs = if operation.2 == "humn" {
        Variable::X
    } else if known.contains_key(&operation.2) {
        Variable::Known(known[&operation.2])
    } else {
        find_with_unknown(&dependencies[&operation.2], dependencies, known)
    };

    match (lhs, rhs) {
        (Variable::Known(lhs), Variable::Known(rhs)) => Variable::Known(match operation.1 {
            '+' => lhs + rhs,
            '-' | '=' => lhs - rhs,
            '/' => lhs / rhs,
            '*' => lhs * rhs,
            _ => panic!(),
        }),
        t => Variable::Unknown(Box::new(t), operation.1),
    }
}
