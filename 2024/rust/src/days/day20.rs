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

        let Position { row, col } = position;

        if self.space[row - 1][col] {
            successors.push(Position { row: row - 1, col });
        }

        if self.space[row][col - 1] {
            successors.push(Position { row, col: col - 1 });
        }

        if self.space[row + 1][col] {
            successors.push(Position { row: row + 1, col });
        }

        if self.space[row][col + 1] {
            successors.push(Position { row, col: col + 1 });
        }

        successors
    }

    fn calculate_cheating_possibilities(&self, threshold: i32, allowed_skips: i32) -> usize {
        // The problem prompt states that there is only a single path to be
        // taken, so using pathfinding here is basically overkill.
        let path = pathfinding::prelude::dfs(
            self.start,
            |position| self.get_successors(*position),
            |position| *position == self.end,
        )
        .unwrap();

        let steps_taken_per_position = path
            .iter()
            .enumerate()
            .map(|(steps, position)| (*position, steps))
            .collect::<HashMap<_, _>>();

        let mut cheating_possibilities = 0;

        for (current_cost, Position { row, col }) in path.iter().enumerate() {
            cheating_possibilities += path[current_cost + 1..]
                .iter()
                .filter(|to| {
                    let from_row = i32::try_from(*row).unwrap();
                    let from_col = i32::try_from(*col).unwrap();

                    let to_row = i32::try_from(to.row).unwrap();
                    let to_col = i32::try_from(to.col).unwrap();

                    let steps_between = (from_row - to_row).abs() + (from_col - to_col).abs();

                    if steps_between > allowed_skips {
                        return false;
                    }

                    if let Some(to_distance) = steps_taken_per_position.get(to) {
                        let from_distance = i32::try_from(current_cost).unwrap();
                        let to_distance = i32::try_from(*to_distance).unwrap();

                        (to_distance - from_distance) - steps_between >= threshold
                    } else {
                        false
                    }
                })
                .count();
        }

        cheating_possibilities
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
