use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn calculate_digits(number: i64) -> u32 {
    number.checked_ilog10().unwrap_or(0) + 1
}

pub fn solve_part1(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let mut sum = 0;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        let (test_value, equation) = line.split_once(": ").unwrap();

        let test_value = test_value.parse::<i64>().unwrap();

        let values = equation
            .split_whitespace()
            .map(|value| value.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut possible_values = vec![values[0]];

        for value in values.iter().skip(1) {
            let mut new_possible_values = vec![];

            for possible_value in possible_values {
                new_possible_values.push(possible_value.checked_add(*value).unwrap());
                new_possible_values.push(possible_value.checked_mul(*value).unwrap());
            }

            possible_values = new_possible_values;
        }

        if possible_values.iter().any(|value| *value == test_value) {
            sum += test_value;
        }
    }

    sum.to_string()
}

pub fn solve_part2(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let mut sum = 0;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        let (test_value, equation) = line.split_once(": ").unwrap();

        let test_value = test_value.parse::<i64>().unwrap();

        let values = equation
            .split_whitespace()
            .map(|value| value.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut possible_values = vec![values[0]];

        for value in values.iter().skip(1) {
            let mut new_possible_values = vec![];

            for possible_value in possible_values {
                new_possible_values.push(possible_value.checked_add(*value).unwrap());
                new_possible_values.push(possible_value.checked_mul(*value).unwrap());
                new_possible_values.push({
                    let digits = calculate_digits(*value);
                    possible_value * 10_i64.pow(digits) + value
                });
            }

            possible_values = new_possible_values;
        }

        if possible_values.iter().any(|value| *value == test_value) {
            sum += test_value;
        }
    }

    sum.to_string()
}
