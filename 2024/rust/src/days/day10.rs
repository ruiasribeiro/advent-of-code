use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

pub fn solve(path: &Path) -> (String, String) {
    let file = File::open(path).unwrap();

    let map: Vec<Vec<_>> = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut sum_distinct = 0;
    let mut sum_all = 0;

    let row_count = map.len();
    let col_count = map[0].len();

    for i in 0..row_count {
        for j in 0..col_count {
            if map[i][j] != 0 {
                continue;
            }

            let mut to_analyse = VecDeque::new();
            to_analyse.push_back(Position { row: i, col: j });

            let mut distinct_destinations = HashSet::new();

            while let Some(position) = to_analyse.pop_front() {
                let Position { row, col } = position;
                let current_value = map[row][col];

                if current_value == 9 {
                    sum_all += 1;
                    distinct_destinations.insert(position);
                    continue;
                }

                if row > 0 && map[row - 1][col] == current_value + 1 {
                    to_analyse.push_back(Position { row: row - 1, col });
                }

                if col > 0 && map[row][col - 1] == current_value + 1 {
                    to_analyse.push_back(Position { row, col: col - 1 });
                }

                if row < row_count - 1 && map[row + 1][col] == current_value + 1 {
                    to_analyse.push_back(Position { row: row + 1, col });
                }

                if col < col_count - 1 && map[row][col + 1] == current_value + 1 {
                    to_analyse.push_back(Position { row, col: col + 1 });
                }
            }

            sum_distinct += distinct_destinations.len();
        }
    }

    (sum_distinct.to_string(), sum_all.to_string())
}

pub fn solve_part1(path: &Path) -> String {
    let (solution, _) = solve(path);
    solution
}

pub fn solve_part2(path: &Path) -> String {
    let (_, solution) = solve(path);
    solution
}
