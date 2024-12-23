mod cli;
mod days;

use std::{
    io::{self, Write},
    path::Path,
    time::{Duration, Instant},
};

use clap::Parser;

use cli::{Cli, Commands};

type Solver = fn(&Path) -> String;

const SOLVERS: [(Solver, Solver); 23] = [
    (days::day01::solve_part1, days::day01::solve_part2),
    (days::day02::solve_part1, days::day02::solve_part2),
    (days::day03::solve_part1, days::day03::solve_part2),
    (days::day04::solve_part1, days::day04::solve_part2),
    (days::day05::solve_part1, days::day05::solve_part2),
    (days::day06::solve_part1, days::day06::solve_part2),
    (days::day07::solve_part1, days::day07::solve_part2),
    (days::day08::solve_part1, days::day08::solve_part2),
    (days::day09::solve_part1, days::day09::solve_part2),
    (days::day10::solve_part1, days::day10::solve_part2),
    (days::day11::solve_part1, days::day11::solve_part2),
    (days::day12::solve_part1, days::day12::solve_part2),
    (days::day13::solve_part1, days::day13::solve_part2),
    (days::day14::solve_part1, days::day14::solve_part2),
    (days::day15::solve_part1, days::day15::solve_part2),
    (days::day16::solve_part1, days::day16::solve_part2),
    (days::day17::solve_part1, days::day17::solve_part2),
    (days::day18::solve_part1, days::day18::solve_part2),
    (days::day19::solve_part1, days::day19::solve_part2),
    (days::day20::solve_part1, days::day20::solve_part2),
    (days::day21::solve_part1, days::day21::solve_part2),
    (days::day22::solve_part1, days::day22::solve_part2),
    (days::day23::solve_part1, days::day23::solve_part2),
];

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Fetch { .. } => todo!(),
        Commands::Solve { day, input } => solve(day, input),
    }
}

fn solve(day: Option<u8>, input: String) {
    println!("using '{}' for input", input);
    println!();

    match day {
        Some(day) => solve_day(usize::from(day), &input).unwrap(),
        None => (1..=SOLVERS.len()).for_each(|day| solve_day(day, input.as_ref()).unwrap()),
    }
}

fn solve_day(day: usize, input: &str) -> Result<(), anyhow::Error> {
    let (solve_part1, solve_part2) = SOLVERS.get(day - 1).ok_or(anyhow::anyhow!(
        "got invalid day ({}), please pick between 1 and {}",
        day,
        SOLVERS.len()
    ))?;

    let path_name = format!("inputs/day{day:02}/{input}");
    let path = Path::new(&path_name);

    print!("day{day:02}: ");

    let start = Instant::now();
    let part1_result = solve_part1(path);
    let part1_duration = start.elapsed();

    print_result(&part1_result, &part1_duration);

    io::stdout().flush().unwrap();

    let start = Instant::now();
    let part2_result = solve_part2(path);
    let part2_duration = start.elapsed();

    print!("  ");

    print_result(&part2_result, &part2_duration);

    println!();

    Ok(())
}

fn print_result(result: &str, duration: &Duration) {
    print!("{:>38} ({:>6.3}s)", result, duration.as_secs_f64());
}
