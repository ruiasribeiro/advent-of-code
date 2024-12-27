use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(number: u64) -> u64 {
    number % 16_777_216
}

fn evolve(number: u64) -> u64 {
    let step_1 = prune(mix(number * 64, number));
    let step_2 = prune(mix(step_1 / 32, step_1));
    prune(mix(step_2 * 2048, step_2))
}

pub fn solve_part1(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let iterations = 2000;

    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .map(|number| (0..iterations).fold(number, |acc, _i| evolve(acc)))
        .sum::<u64>()
        .to_string()
}

fn is_full_input(path: &Path) -> bool {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .contains("input")
}

pub fn solve_part2(path: &Path) -> String {
    if !is_full_input(path) {
        return String::from("N/A");
    }

    let file = File::open(path).unwrap();

    let iterations = 2000;

    let evolutions = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .map(|number| {
            (0..iterations).fold(
                {
                    let mut numbers = vec![number];
                    numbers.reserve(iterations);
                    numbers
                },
                |mut numbers, _i| {
                    let next = evolve(*numbers.last().unwrap());
                    numbers.push(next);
                    numbers
                },
            )
        })
        .collect::<Vec<_>>();

    let changes = evolutions
        .iter()
        .map(|evolution| {
            evolution
                .windows(2)
                .map(|window| (window[1] % 10) as i8 - (window[0] % 10) as i8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let all_buyer_sequences = changes
        .iter()
        .enumerate()
        .map(|(i_evolution, change)| {
            change.windows(4).enumerate().fold(
                HashMap::<&[i8], u32>::new(),
                |mut acc, (i, window)| {
                    acc.entry(window)
                        .or_insert((evolutions[i_evolution][i + 4] % 10) as u32);

                    acc
                },
            )
        })
        .collect::<Vec<_>>();

    all_buyer_sequences
        .iter()
        .fold(HashMap::<&[i8], u32>::new(), |mut acc, sequences| {
            for (sequence, value) in sequences {
                *acc.entry(sequence).or_default() += value;
            }

            acc
        })
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(_k, v)| v)
        .unwrap()
        .to_string()
}
