pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

use crate::utils::Solver;

pub fn get_day_solvers(day: u8) -> Result<(Solver, Solver), anyhow::Error> {
    match day {
        1 => Ok((day01::solve_part1, day01::solve_part2)),
        2 => Ok((day02::solve_part1, day02::solve_part2)),
        3 => Ok((day03::solve_part1, day03::solve_part2)),
        4 => Ok((day04::solve_part1, day04::solve_part2)),
        _ => Err(anyhow::anyhow!("could not find solver for 2015/{day}")),
    }
}

pub fn get_available_days() -> Vec<u8> {
    (1..=4).collect()
}
