use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::utils::is_full_input;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

pub fn solve_part1(path: &Path) -> String {
    if !is_full_input(path) {
        return String::from("N/A");
    }

    let file = File::open(path).unwrap();

    let mut current_position = Position { x: 0, y: 0 };

    let mut houses: HashSet<Position> = HashSet::new();
    houses.insert(current_position);

    for c in BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
    {
        match c {
            '^' => current_position.y += 1,
            'v' => current_position.y -= 1,
            '>' => current_position.x += 1,
            '<' => current_position.x -= 1,
            _ => unreachable!(),
        };

        houses.insert(current_position);
    }

    houses.len().to_string()
}

pub fn solve_part2(path: &Path) -> String {
    if !is_full_input(path) {
        return String::from("N/A");
    }

    let file = File::open(path).unwrap();

    let mut santa_position = Position { x: 0, y: 0 };
    let mut robot_position = Position { x: 0, y: 0 };

    let mut is_santa_turn = true;

    let mut houses: HashSet<Position> = HashSet::new();
    houses.insert(santa_position);

    for c in BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
    {
        let position = if is_santa_turn {
            &mut santa_position
        } else {
            &mut robot_position
        };

        match c {
            '^' => position.y += 1,
            'v' => position.y -= 1,
            '>' => position.x += 1,
            '<' => position.x -= 1,
            _ => unreachable!(),
        };

        houses.insert(*position);
        is_santa_turn = !is_santa_turn;
    }

    houses.len().to_string()
}
