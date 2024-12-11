use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn calculate_digits(number: i64) -> u32 {
    number.checked_ilog10().unwrap_or(0) + 1
}

fn solve(path: &Path, iterations: usize) -> String {
    let file = File::open(path).unwrap();

    let mut stones = BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|stone| stone.parse::<i64>().unwrap())
        .fold(HashMap::new(), |mut map, stone| {
            *map.entry(stone).or_default() += 1;
            map
        });

    for _ in 0..iterations {
        let mut next_stones = HashMap::new();

        for (stone, count) in stones {
            if stone == 0 {
                *next_stones.entry(1).or_default() += count;
                continue;
            }

            let digits = calculate_digits(stone);

            if digits % 2 == 0 {
                let factor = 10_i64.pow(digits / 2);

                let left = stone / factor;
                let right = stone % factor;

                *next_stones.entry(left).or_default() += count;
                *next_stones.entry(right).or_default() += count;

                continue;
            }

            *next_stones.entry(stone * 2024).or_default() += count;
        }

        stones = next_stones;
    }

    stones.values().sum::<i64>().to_string()
}

pub fn solve_part1(path: &Path) -> String {
    solve(path, 25)
}

pub fn solve_part2(path: &Path) -> String {
    solve(path, 75)
}
