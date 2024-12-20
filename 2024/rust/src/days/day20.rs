use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

struct Simulation {
    space: Vec<Vec<bool>>,
    start: Position,
    end: Position,
}

impl Simulation {
    fn from_file(path: &Path) -> Self {
        let file = File::open(path).unwrap();

        let mut start = None;
        let mut end = None;

        let space = BufReader::new(file)
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.unwrap()
                    .char_indices()
                    .map(|(col, c)| match c {
                        '#' => false,
                        '.' => true,
                        'S' => {
                            start = Some(Position { row, col });
                            true
                        }
                        'E' => {
                            end = Some(Position { row, col });
                            true
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        let start = start.unwrap();
        let end = end.unwrap();

        Simulation { space, start, end }
    }

    fn get_successors(&self, position: Position) -> Vec<Position> {
        let mut successors = vec![];

        let row_count = self.space.len();
        let col_count = self.space[0].len();

        let Position { row, col } = position;

        if row > 0 && self.space[row - 1][col] {
            successors.push(Position { row: row - 1, col });
        }

        if col > 0 && self.space[row][col - 1] {
            successors.push(Position { row, col: col - 1 });
        }

        if row < row_count - 1 && self.space[row + 1][col] {
            successors.push(Position { row: row + 1, col });
        }

        if col < col_count - 1 && self.space[row][col + 1] {
            successors.push(Position { row, col: col + 1 });
        }

        successors
    }

    fn calculate_cheating_possibilities(&self, threshold: i32, allowed_skips: i32) -> usize {
        let regular_result = pathfinding::prelude::bfs(
            &self.start,
            |position| self.get_successors(*position),
            |position| *position == self.end,
        )
        .unwrap();

        let steps_per_position = regular_result
            .iter()
            .enumerate()
            .map(|(steps, position)| (*position, steps))
            .collect::<HashMap<_, _>>();

        regular_result
            .iter()
            .enumerate()
            .map(|(current_cost, Position { row, col })| {
                self.space
                    .iter()
                    .enumerate()
                    .flat_map(|(r, tiles)| {
                        tiles
                            .iter()
                            .enumerate()
                            .filter(|(_c, tile)| **tile)
                            .map(move |(c, _tile)| Position { row: r, col: c })
                    })
                    .filter(|position @ Position { row: r, col: c }| {
                        let steps = (i32::try_from(*row).unwrap() - i32::try_from(*r).unwrap())
                            .abs()
                            + (i32::try_from(*col).unwrap() - i32::try_from(*c).unwrap()).abs();

                        if steps > allowed_skips {
                            return false;
                        }

                        match steps_per_position.get(position) {
                            Some(distance) => {
                                (i32::try_from(*distance).unwrap()
                                    - i32::try_from(current_cost).unwrap())
                                    - steps
                                    >= threshold
                            }
                            None => false,
                        }
                    })
                    .count()
            })
            .sum()
    }
}

fn is_full_input(path: &Path) -> bool {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .contains("input")
}

pub fn solve_part1(path: &Path) -> String {
    let threshold = if is_full_input(path) { 100 } else { 2 };
    let skips = 2;

    Simulation::from_file(path)
        .calculate_cheating_possibilities(threshold, skips)
        .to_string()
}

pub fn solve_part2(path: &Path) -> String {
    let threshold = if is_full_input(path) { 100 } else { 50 };
    let skips = 20;

    Simulation::from_file(path)
        .calculate_cheating_possibilities(threshold, skips)
        .to_string()
}
