pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

use crate::interface::Solver;

pub fn get_day_solvers(day: u8) -> Result<(Solver, Solver), anyhow::Error> {
    match day {
        1 => Ok((day01::solve_part1, day01::solve_part2)),
        2 => Ok((day02::solve_part1, day02::solve_part2)),
        3 => Ok((day03::solve_part1, day03::solve_part2)),
        4 => Ok((day04::solve_part1, day04::solve_part2)),
        5 => Ok((day05::solve_part1, day05::solve_part2)),
        6 => Ok((day06::solve_part1, day06::solve_part2)),
        7 => Ok((day07::solve_part1, day07::solve_part2)),
        8 => Ok((day08::solve_part1, day08::solve_part2)),
        9 => Ok((day09::solve_part1, day09::solve_part2)),
        10 => Ok((day10::solve_part1, day10::solve_part2)),
        11 => Ok((day11::solve_part1, day11::solve_part2)),
        12 => Ok((day12::solve_part1, day12::solve_part2)),
        13 => Ok((day13::solve_part1, day13::solve_part2)),
        14 => Ok((day14::solve_part1, day14::solve_part2)),
        15 => Ok((day15::solve_part1, day15::solve_part2)),
        16 => Ok((day16::solve_part1, day16::solve_part2)),
        17 => Ok((day17::solve_part1, day17::solve_part2)),
        18 => Ok((day18::solve_part1, day18::solve_part2)),
        19 => Ok((day19::solve_part1, day19::solve_part2)),
        20 => Ok((day20::solve_part1, day20::solve_part2)),
        21 => Ok((day21::solve_part1, day21::solve_part2)),
        22 => Ok((day22::solve_part1, day22::solve_part2)),
        23 => Ok((day23::solve_part1, day23::solve_part2)),
        24 => Ok((day24::solve_part1, day24::solve_part2)),
        25 => Ok((day25::solve_part1, day25::solve_part2)),
        _ => Err(anyhow::anyhow!("could not find solver for 2024/{day}")),
    }
}

pub fn get_available_days() -> Vec<u8> {
    (1..=25).collect()
}
