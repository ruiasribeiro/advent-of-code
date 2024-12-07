mod days;

use std::{
    io::{stdout, Write},
    path::Path,
    time::{Duration, Instant},
};

use days::{day01, day02, day03, day04, day05, day06, day07};

type SolveDay = fn(&Path) -> String;

const SOLVERS: [(&str, SolveDay, SolveDay); 7] = [
    ("day01", day01::solve_part1, day01::solve_part2),
    ("day02", day02::solve_part1, day02::solve_part2),
    ("day03", day03::solve_part1, day03::solve_part2),
    ("day04", day04::solve_part1, day04::solve_part2),
    ("day05", day05::solve_part1, day05::solve_part2),
    ("day06", day06::solve_part1, day06::solve_part2),
    ("day07", day07::solve_part1, day07::solve_part2),
];

const FILE_NAME: &str = "input.txt";

fn print_result(result: &str, duration: &Duration) {
    print!("{:>14} ({:>6.3}s)", result, duration.as_secs_f64());
}

fn main() {
    for (day, part1, part2) in SOLVERS {
        let path_name = format!("inputs/{day}/{FILE_NAME}");
        let path = Path::new(&path_name);

        print!("{day}: ");

        let start = Instant::now();
        let part1_result = part1(path);
        let part1_duration = start.elapsed();

        print_result(&part1_result, &part1_duration);

        stdout().flush().unwrap();

        let start = Instant::now();
        let part2_result = part2(path);
        let part2_duration = start.elapsed();

        print!(" / ");

        print_result(&part2_result, &part2_duration);

        println!();
    }
}
