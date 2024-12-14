mod days;

use std::{
    env,
    io::{self, Write},
    path::Path,
    time::{Duration, Instant},
};

type Solver = fn(&Path) -> String;

const SOLVERS: [(&str, Solver, Solver); 14] = [
    ("day01", days::day01::solve_part1, days::day01::solve_part2),
    ("day02", days::day02::solve_part1, days::day02::solve_part2),
    ("day03", days::day03::solve_part1, days::day03::solve_part2),
    ("day04", days::day04::solve_part1, days::day04::solve_part2),
    ("day05", days::day05::solve_part1, days::day05::solve_part2),
    ("day06", days::day06::solve_part1, days::day06::solve_part2),
    ("day07", days::day07::solve_part1, days::day07::solve_part2),
    ("day08", days::day08::solve_part1, days::day08::solve_part2),
    ("day09", days::day09::solve_part1, days::day09::solve_part2),
    ("day10", days::day10::solve_part1, days::day10::solve_part2),
    ("day11", days::day11::solve_part1, days::day11::solve_part2),
    ("day12", days::day12::solve_part1, days::day12::solve_part2),
    ("day13", days::day13::solve_part1, days::day13::solve_part2),
    ("day14", days::day14::solve_part1, days::day14::solve_part2),
];

const DEFAULT_FILE_NAME: &str = "input.txt";

fn print_result(result: &str, duration: &Duration) {
    print!("{:>15} ({:>6.3}s)", result, duration.as_secs_f64());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = if args.len() >= 2 {
        println!("using '{}' for the input data", args[1]);
        &args[1]
    } else {
        println!("using the default file name ('{DEFAULT_FILE_NAME}')");
        DEFAULT_FILE_NAME
    };

    for (day, part1, part2) in SOLVERS {
        let path_name = format!("inputs/{day}/{file_name}");
        let path = Path::new(&path_name);

        print!("{day}: ");

        let start = Instant::now();
        let part1_result = part1(path);
        let part1_duration = start.elapsed();

        print_result(&part1_result, &part1_duration);

        io::stdout().flush().unwrap();

        let start = Instant::now();
        let part2_result = part2(path);
        let part2_duration = start.elapsed();

        print!("  ");

        print_result(&part2_result, &part2_duration);

        println!();
    }
}
