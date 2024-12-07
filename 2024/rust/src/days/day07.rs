use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_part1(path: &str) -> String {
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

            for possible_value in possible_values.iter() {
                new_possible_values.push(possible_value.checked_add(*value).unwrap());
                new_possible_values.push(possible_value.checked_mul(*value).unwrap());
            }

            possible_values = new_possible_values;
        }

        if possible_values.iter().any(|value| *value == test_value) {
            sum += test_value;
        }
    }

    return sum.to_string();
}

pub fn solve_part2(path: &str) -> String {
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

            for possible_value in possible_values.iter() {
                new_possible_values.push(possible_value.checked_add(*value).unwrap());
                new_possible_values.push(possible_value.checked_mul(*value).unwrap());
                new_possible_values.push({
                    let mut temp = possible_value.to_string();
                    temp.push_str(&value.to_string());
                    temp.parse::<i64>().unwrap()
                });
            }

            possible_values = new_possible_values;
        }

        if possible_values.iter().any(|value| *value == test_value) {
            sum += test_value;
        }
    }

    return sum.to_string();
}
