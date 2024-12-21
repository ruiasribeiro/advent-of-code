use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use cached::proc_macro::cached;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    row: isize,
    col: isize,
}

fn get_numeric_key_position(key: char) -> Position {
    match key {
        'A' => Position { row: 3, col: 2 },
        '0' => Position { row: 3, col: 1 },
        '1' => Position { row: 2, col: 0 },
        '2' => Position { row: 2, col: 1 },
        '3' => Position { row: 2, col: 2 },
        '4' => Position { row: 1, col: 0 },
        '5' => Position { row: 1, col: 1 },
        '6' => Position { row: 1, col: 2 },
        '7' => Position { row: 0, col: 0 },
        '8' => Position { row: 0, col: 1 },
        '9' => Position { row: 0, col: 2 },
        _ => unreachable!("unexpected key: {}", key),
    }
}

fn get_directional_keypad_key_position(key: char) -> Position {
    match key {
        'A' => Position { row: 0, col: 2 },
        '^' => Position { row: 0, col: 1 },
        '<' => Position { row: 1, col: 0 },
        'v' => Position { row: 1, col: 1 },
        '>' => Position { row: 1, col: 2 },
        _ => unreachable!(),
    }
}

fn generate_numeric_keypad_movement(from: Position, to: Position) -> String {
    let row_difference = to.row - from.row;
    let col_difference = to.col - from.col;

    let mut instructions = String::new();
    instructions.reserve_exact(row_difference.unsigned_abs() + col_difference.unsigned_abs() + 1);

    if from.col == 0 && to.row == 3 {
        for _ in 0..col_difference {
            instructions.push('>');
        }
        for _ in 0..row_difference {
            instructions.push('v');
        }
    } else if to.col == 0 && from.row == 3 {
        for _ in 0..-row_difference {
            instructions.push('^');
        }
        for _ in 0..-col_difference {
            instructions.push('<');
        }
    } else {
        let row_direction = if row_difference < 0 { "^" } else { "v" };
        let col_direction = if col_difference < 0 { "<" } else { ">" };

        let row_difference = row_difference.unsigned_abs();
        let col_difference = col_difference.unsigned_abs();

        if col_direction == "<" {
            instructions.push_str(&col_direction.repeat(col_difference));
            instructions.push_str(&row_direction.repeat(row_difference));
        } else {
            instructions.push_str(&row_direction.repeat(row_difference));
            instructions.push_str(&col_direction.repeat(col_difference));
        }
    }

    instructions.push('A');

    instructions
}

#[cached]
fn generate_directional_keypad_movement(from: Position, to: Position) -> String {
    let row_difference = to.row - from.row;
    let col_difference = to.col - from.col;

    let mut instructions = String::new();
    instructions.reserve_exact(row_difference.unsigned_abs() + col_difference.unsigned_abs() + 1);

    if from.col == 0 && to.row == 0 {
        for _ in 0..col_difference {
            instructions.push('>');
        }

        instructions.push('^');
    } else if to.col == 0 && from.row == 0 {
        instructions.push('v');

        for _ in 0..-col_difference {
            instructions.push('<');
        }
    } else {
        let row_direction = if row_difference < 0 { "^" } else { "v" };
        let col_direction = if col_difference < 0 { "<" } else { ">" };

        let row_difference = row_difference.unsigned_abs();
        let col_difference = col_difference.unsigned_abs();

        if col_direction == "<" {
            instructions.push_str(&col_direction.repeat(col_difference));
            instructions.push_str(&row_direction.repeat(row_difference));
        } else {
            instructions.push_str(&row_direction.repeat(row_difference));
            instructions.push_str(&col_direction.repeat(col_difference));
        }
    }

    instructions.push('A');

    instructions
}

fn generate_numeric_keypad_instructions(code: &str) -> String {
    let mut arm_position = get_numeric_key_position('A');
    let mut instructions = String::new();

    for key in code.chars() {
        let next_position = get_numeric_key_position(key);
        let next_instructions = generate_numeric_keypad_movement(arm_position, next_position);

        instructions.push_str(&next_instructions);

        arm_position = next_position;
    }

    instructions
}

#[cached]
fn calculate_total_instructions(instructions: String, level_of_indirection: i32) -> usize {
    if level_of_indirection == 0 {
        return instructions.len();
    }

    let mut arm_position = get_directional_keypad_key_position('A');
    let mut count = 0;

    for key in instructions.chars() {
        let target_position = get_directional_keypad_key_position(key);
        let next_instructions = generate_directional_keypad_movement(arm_position, target_position);

        let new_count = calculate_total_instructions(next_instructions, level_of_indirection - 1);

        count += new_count;
        arm_position = target_position;
    }

    count
}

pub fn solve(path: &Path, level_of_indirection: i32) -> usize {
    let file = File::open(path).unwrap();

    let codes = BufReader::new(file).lines().map(Result::unwrap);

    let mut total_complexity = 0;

    for code in codes {
        let instructions = generate_numeric_keypad_instructions(&code);
        let total_instructions = calculate_total_instructions(instructions, level_of_indirection);

        total_complexity += total_instructions * code[0..code.len() - 1].parse::<usize>().unwrap();
    }

    total_complexity
}

pub fn solve_part1(path: &Path) -> String {
    solve(path, 2).to_string()
}

pub fn solve_part2(path: &Path) -> String {
    solve(path, 25).to_string()
}
