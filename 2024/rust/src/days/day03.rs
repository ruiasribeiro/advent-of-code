use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

pub fn solve_part1(path: &str) -> String {
    let file = File::open(path).unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            re.captures_iter(&line)
                .map(|c| c.extract())
                .map(|(_, [lhs, rhs])| lhs.parse::<i32>().unwrap() * rhs.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sum::<i32>()
        .to_string()
}

pub fn solve_part2(path: &str) -> String {
    let file = File::open(path).unwrap();

    let re = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();

    let mut sum = 0;
    let mut is_enabled = true;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        for capture in re.captures_iter(&line) {
            if is_enabled {
                if let Some(_) = capture.get(1) {
                    let lhs = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    let rhs = capture.get(3).unwrap().as_str().parse::<i32>().unwrap();

                    sum += lhs * rhs;
                }

                if let Some(_) = capture.get(5) {
                    is_enabled = false;
                    continue;
                }
            } else if let Some(_) = capture.get(4) {
                is_enabled = true;
                continue;
            }
        }
    }

    sum.to_string()
}
