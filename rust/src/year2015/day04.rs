use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use md5::Digest;

use crate::utils::is_full_input;

pub fn solve_part1(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let secret_key = BufReader::new(file).lines().next().unwrap().unwrap();
    let mut number = 0_usize;

    loop {
        let mut hasher = md5::Md5::new();
        hasher.update(format!("{}{}", secret_key, number));
        let result = hasher.finalize();

        if hex::encode(result).starts_with("00000") {
            return number.to_string();
        }

        number += 1;
    }
}

pub fn solve_part2(path: &Path) -> String {
    if !is_full_input(path) {
        return String::from("N/A");
    }

    let file = File::open(path).unwrap();

    let secret_key = BufReader::new(file).lines().next().unwrap().unwrap();
    let mut number = 0_usize;

    loop {
        let mut hasher = md5::Md5::new();
        hasher.update(format!("{}{}", secret_key, number));
        let result = hasher.finalize();

        if hex::encode(result).starts_with("000000") {
            return number.to_string();
        }

        number += 1;
    }
}
