mod cli;
mod utils;
mod year2024;

use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, Write},
    path::Path,
    time::{Duration, Instant},
};

use clap::Parser;

use cli::{Cli, Commands};
use utils::Solver;

fn get_available_days(year: u16) -> Result<Vec<u8>, anyhow::Error> {
    match year {
        2024 => Ok(year2024::get_available_days()),
        _ => Err(anyhow::anyhow!("year {year} is not available")),
    }
}

fn get_day_solvers(year: u16, day: u8) -> Result<(Solver, Solver), anyhow::Error> {
    match year {
        2024 => year2024::get_day_solvers(day),
        _ => Err(anyhow::anyhow!("year {year} is not available")),
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();

    match args.command {
        Commands::Fetch { year, day } => fetch_day_input(year, day)?,
        Commands::Solve { year, day, input } => solve(year, day, &input)?,
    };

    Ok(())
}

fn fetch_day_input(year: u16, day: u8) -> Result<(), anyhow::Error> {
    let target_file_name = format!("../_inputs/year{year}/day{day:02}/input.txt");
    let target_file = Path::new(&target_file_name);

    if target_file.exists() {
        return Err(anyhow::anyhow!(
            "'{}' already exists; skipping...",
            target_file.display()
        ));
    }

    dotenvy::dotenv()?;

    let session = {
        let env = env::vars().collect::<HashMap<_, _>>();
        env.get("AOC_SESSION").map(ToOwned::to_owned)
    };

    match session {
        None => Err(anyhow::anyhow!("could not find AOC_SESSION in env")),

        Some(session) => {
            let mut request_headers = reqwest::header::HeaderMap::new();
            request_headers.insert(
                reqwest::header::COOKIE,
                reqwest::header::HeaderValue::from_str(&format!("session={session}"))?,
            );

            let client = reqwest::blocking::Client::builder()
                .default_headers(request_headers)
                .build()?;

            let input = client
                .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
                .send()?
                .bytes()?;

            fs::create_dir_all(
                target_file
                    .parent()
                    .ok_or(anyhow::anyhow!("could not get parent directory"))?,
            )?;

            File::create(target_file)?.write_all(input.trim_ascii())?;

            Ok(())
        }
    }
}

fn solve(year: u16, day: Option<u8>, input: &str) -> Result<(), anyhow::Error> {
    println!("solving year {year} using '{input}' for input");
    println!();

    if let Some(day) = day {
        solve_day(year, day, input)?;
    } else {
        for day in get_available_days(year)? {
            solve_day(year, day, input)?;
        }
    };

    Ok(())
}

fn solve_day(year: u16, day: u8, input: &str) -> Result<(), anyhow::Error> {
    let (part1_solver, part2_solver) = get_day_solvers(year, day)?;

    let path_name = format!("../_inputs/year{year}/day{day:02}/{input}");
    let path = Path::new(&path_name);

    print!("day{day:02}: ");

    let start = Instant::now();
    let part1_result = part1_solver(path);
    let part1_duration = start.elapsed();

    print_result(&part1_result, &part1_duration);

    io::stdout().flush().unwrap();

    let start = Instant::now();
    let part2_result = part2_solver(path);
    let part2_duration = start.elapsed();

    print!("  ");

    print_result(&part2_result, &part2_duration);

    println!();

    Ok(())
}

fn print_result(result: &str, duration: &Duration) {
    print!("{:>38} ({:>6.3}s)", result, duration.as_secs_f64());
}
