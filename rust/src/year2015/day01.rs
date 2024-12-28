use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::utils::is_full_input;

pub fn solve_part1(path: &Path) -> String {
    if !is_full_input(path) {
        return String::from("N/A");
    }

    let file = File::open(path).unwrap();

    BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .fold(0, |acc, c| if c == '(' { acc + 1 } else { acc - 1 })
        .to_string()
}

pub fn solve_part2(path: &Path) -> String {
    if !is_full_input(path) {
        return String::from("N/A");
    }

    let file = File::open(path).unwrap();
    let line = BufReader::new(file).lines().next().unwrap().unwrap();

    let mut floor = 0;

    for (i, c) in line.chars().enumerate() {
        if floor == 0 && c == ')' {
            return (i + 1).to_string();
        }

        floor += if c == '(' { 1 } else { -1 };
    }

    unreachable!();
}
