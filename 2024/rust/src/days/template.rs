use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn _solve_part1(path: &Path) -> String {
    let file = File::open(path).unwrap();

    for _line in BufReader::new(file).lines() {
        // do something
    }

    String::new()
}

pub fn _solve_part2(path: &Path) -> String {
    let file = File::open(path).unwrap();

    for _line in BufReader::new(file).lines() {
        // do something
    }

    String::new()
}
