pub mod day01;

use crate::utils::Solver;

pub fn get_day_solvers(day: u8) -> Result<(Solver, Solver), anyhow::Error> {
    match day {
        1 => Ok((day01::solve_part1, day01::solve_part2)),
        _ => Err(anyhow::anyhow!("could not find solver for 2015/{day}")),
    }
}

pub fn get_available_days() -> Vec<u8> {
    (1..=1).collect()
}
