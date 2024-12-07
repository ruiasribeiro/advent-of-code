use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn _solve_part1(path: &str) -> String {
    let file = File::open(path).unwrap();

    for _line in BufReader::new(file).lines() {
        // do something
    }

    String::new()
}

pub fn _solve_part2(path: &str) -> String {
    let file = File::open(path).unwrap();

    for _line in BufReader::new(file).lines() {
        // do something
    }

    String::new()
}
