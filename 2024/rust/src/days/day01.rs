use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
    path::Path,
};

pub fn solve_part1(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in BufReader::new(file).lines() {
        let numbers = line
            .unwrap()
            .split_whitespace()
            .map(|chunk| chunk.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    left.sort_unstable();
    right.sort_unstable();

    zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>()
        .to_string()
}

pub fn solve_part2(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let mut left: Vec<i32> = vec![];
    let mut right: HashMap<i32, i32> = HashMap::new();

    for line in BufReader::new(file).lines() {
        let numbers = line
            .unwrap()
            .split_whitespace()
            .map(|chunk| chunk.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        left.push(numbers[0]);

        let current_count = right.get_mut(&numbers[1]);

        if let Some(value) = current_count {
            *value += 1;
        } else {
            right.insert(numbers[1], 1);
        }
    }

    left.iter()
        .map(|num| {
            if let Some(val) = right.get(num) {
                num * (*val)
            } else {
                0
            }
        })
        .sum::<i32>()
        .to_string()
}
