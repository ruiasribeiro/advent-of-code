use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn solve_part1(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let file = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let row_count = file.len();
    let col_count = file[0].len();

    let mut counter = 0;

    for row in 0..row_count {
        for col in 0..col_count {
            if col + 3 < col_count
                && file[row][col] == 'X'
                && file[row][col + 1] == 'M'
                && file[row][col + 2] == 'A'
                && file[row][col + 3] == 'S'
            {
                counter += 1;
            }

            if col >= 3
                && file[row][col] == 'X'
                && file[row][col - 1] == 'M'
                && file[row][col - 2] == 'A'
                && file[row][col - 3] == 'S'
            {
                counter += 1;
            }

            if row + 3 < row_count
                && file[row][col] == 'X'
                && file[row + 1][col] == 'M'
                && file[row + 2][col] == 'A'
                && file[row + 3][col] == 'S'
            {
                counter += 1;
            }

            if row >= 3
                && file[row][col] == 'X'
                && file[row - 1][col] == 'M'
                && file[row - 2][col] == 'A'
                && file[row - 3][col] == 'S'
            {
                counter += 1;
            }

            if col + 3 < col_count
                && row + 3 < row_count
                && file[row][col] == 'X'
                && file[row + 1][col + 1] == 'M'
                && file[row + 2][col + 2] == 'A'
                && file[row + 3][col + 3] == 'S'
            {
                counter += 1;
            }

            if col + 3 < col_count
                && row >= 3
                && file[row][col] == 'X'
                && file[row - 1][col + 1] == 'M'
                && file[row - 2][col + 2] == 'A'
                && file[row - 3][col + 3] == 'S'
            {
                counter += 1;
            }

            if col >= 3
                && row >= 3
                && file[row][col] == 'X'
                && file[row - 1][col - 1] == 'M'
                && file[row - 2][col - 2] == 'A'
                && file[row - 3][col - 3] == 'S'
            {
                counter += 1;
            }

            if col >= 3
                && row + 3 < row_count
                && file[row][col] == 'X'
                && file[row + 1][col - 1] == 'M'
                && file[row + 2][col - 2] == 'A'
                && file[row + 3][col - 3] == 'S'
            {
                counter += 1;
            }
        }
    }

    counter.to_string()
}

pub fn solve_part2(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let file = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let row_count = file.len();
    let col_count = file[0].len();

    let mut counter = 0;

    for row in 1..(row_count - 1) {
        for col in 1..(col_count - 1) {
            if file[row][col] == 'A'
                && file[row - 1][col - 1] == 'M'
                && file[row - 1][col + 1] == 'M'
                && file[row + 1][col + 1] == 'S'
                && file[row + 1][col - 1] == 'S'
            {
                counter += 1;
            } else if file[row][col] == 'A'
                && file[row - 1][col - 1] == 'S'
                && file[row - 1][col + 1] == 'M'
                && file[row + 1][col + 1] == 'M'
                && file[row + 1][col - 1] == 'S'
            {
                counter += 1;
            } else if file[row][col] == 'A'
                && file[row - 1][col - 1] == 'S'
                && file[row - 1][col + 1] == 'S'
                && file[row + 1][col + 1] == 'M'
                && file[row + 1][col - 1] == 'M'
            {
                counter += 1;
            } else if file[row][col] == 'A'
                && file[row - 1][col - 1] == 'M'
                && file[row - 1][col + 1] == 'S'
                && file[row + 1][col + 1] == 'S'
                && file[row + 1][col - 1] == 'M'
            {
                counter += 1;
            }
        }
    }

    counter.to_string()
}
