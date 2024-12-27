use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum OperationType {
    And,
    Or,
    Xor,
}

impl OperationType {
    fn apply(self, lhs: bool, rhs: bool) -> bool {
        match self {
            OperationType::And => lhs && rhs,
            OperationType::Or => lhs || rhs,
            OperationType::Xor => lhs ^ rhs,
        }
    }
}

impl TryFrom<&str> for OperationType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "AND" => Ok(OperationType::And),
            "OR" => Ok(OperationType::Or),
            "XOR" => Ok(OperationType::Xor),
            _ => Err(anyhow::anyhow!("unexpected operation type: {}", value)),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Operation {
    lhs: String,
    rhs: String,
    op: OperationType,
    target: String,
}

fn parse_initial_values(lines: &[&str]) -> HashMap<String, bool> {
    lines
        .iter()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();

            let name = name.to_owned();
            let value = match value.parse::<u8>().unwrap() {
                0 => false,
                1 => true,
                _ => unreachable!(),
            };

            (name, value)
        })
        .collect()
}

fn parse_operations(lines: &[&str]) -> Vec<Operation> {
    lines
        .iter()
        .map(|line| {
            let (operation, target) = line.split_once(" -> ").unwrap();

            let chunks = operation.split_whitespace().collect::<Vec<_>>();

            let lhs = chunks[0].to_owned();
            let op = chunks[1].try_into().unwrap();
            let rhs = chunks[2].to_owned();
            let target = target.to_owned();

            Operation {
                lhs,
                rhs,
                op,
                target,
            }
        })
        .collect()
}

pub fn solve_part1(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let lines = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let input = lines.split(String::is_empty).collect::<Vec<_>>();

    let mut values = parse_initial_values(&input[0].iter().map(String::as_str).collect::<Vec<_>>());
    let mut operations = parse_operations(&input[1].iter().map(String::as_str).collect::<Vec<_>>());

    while !operations.is_empty() {
        let mut remaining_operations = vec![];

        for operation in operations {
            let lhs_value = values.get(&operation.lhs);
            let rhs_value = values.get(&operation.rhs);

            match (lhs_value, rhs_value) {
                (Some(lhs), Some(rhs)) => {
                    let result = operation.op.apply(*lhs, *rhs);
                    values.insert(operation.target, result);
                }

                (_, _) => remaining_operations.push(operation),
            }
        }

        operations = remaining_operations;
    }

    let mut zs = values
        .iter()
        .filter(|(key, _value)| key.starts_with('z'))
        .collect::<Vec<_>>();

    zs.sort_unstable_by_key(|(key, _value)| *key);

    zs.iter()
        .enumerate()
        .fold(0_usize, |acc, (i, (_key, value))| {
            if **value {
                acc + 2_usize.pow(i.try_into().unwrap())
            } else {
                acc
            }
        })
        .to_string()
}

pub fn solve_part2(_path: &Path) -> String {
    "to do".into()
}
