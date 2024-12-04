use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_part1(path: &str) -> i32 {
    let file = File::open(path).unwrap();

    for line in BufReader::new(file).lines() {
        // do something
    }

    0
}

pub fn solve_part2(path: &str) -> i32 {
    let file = File::open(path).unwrap();

    for line in BufReader::new(file).lines() {
        // do something
    }

    0
}
