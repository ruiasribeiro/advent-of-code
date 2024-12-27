use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    sync::OnceLock,
};

use cached::proc_macro::cached;

#[derive(Debug)]
struct Problem {
    towels: Vec<String>,
    designs: Vec<String>,
}

impl Problem {
    fn from_file(path: &Path) -> Self {
        let file = File::open(path).unwrap();

        let lines = BufReader::new(file)
            .lines()
            .map(Result::unwrap)
            .collect::<Vec<_>>();

        let (towels, designs) = lines.split_at(1);

        let towels = towels[0]
            .split(", ")
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        let designs = designs[1..]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        Self { towels, designs }
    }
}

static PROBLEM: OnceLock<Problem> = OnceLock::new();

#[cached]
fn count_combinations(design: String) -> u64 {
    if design.is_empty() {
        return 1;
    }

    PROBLEM
        .get()
        .unwrap()
        .towels
        .iter()
        .map(|towel| {
            if !design.starts_with(towel) {
                return 0;
            }

            count_combinations(design[towel.len()..].to_string())
        })
        .sum()
}

pub fn solve_part1(path: &Path) -> String {
    PROBLEM
        .get_or_init(|| Problem::from_file(path))
        .designs
        .iter()
        .filter(|design| count_combinations((*design).to_string()) > 1)
        .count()
        .to_string()
}

pub fn solve_part2(path: &Path) -> String {
    PROBLEM
        .get_or_init(|| Problem::from_file(path))
        .designs
        .iter()
        .map(|design| count_combinations(design.to_string()))
        .sum::<u64>()
        .to_string()
}
