use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn is_valid(&self, row_count: i32, col_count: i32) -> bool {
        (0..row_count).contains(&self.row) && (0..col_count).contains(&self.col)
    }
}

type Antennas = HashMap<char, Vec<Position>>;

fn parse_file(path: &Path) -> (Antennas, i32, i32) {
    let file = File::open(path).unwrap();

    let mut row_count = 0;
    let mut col_count = 0;

    let mut antennas: Antennas = HashMap::new();

    for (row, line) in BufReader::new(file).lines().enumerate() {
        row_count += 1;

        for (col, c) in line.unwrap().chars().enumerate() {
            if row == 0 {
                col_count += 1;
            }

            if c == '.' {
                continue;
            }

            if let Some(antenna) = antennas.get_mut(&c) {
                antenna.push(Position {
                    row: row.try_into().unwrap(),
                    col: col.try_into().unwrap(),
                });
            } else {
                antennas.insert(
                    c,
                    vec![Position {
                        row: row.try_into().unwrap(),
                        col: col.try_into().unwrap(),
                    }],
                );
            }
        }
    }

    (antennas, row_count, col_count)
}

pub fn solve_part1(path: &Path) -> String {
    let (antennas, row_count, col_count) = parse_file(path);

    let mut antinodes: HashSet<Position> = HashSet::new();

    for antenna_positions in antennas.values() {
        for (i, position) in antenna_positions.iter().enumerate() {
            for another_position in antenna_positions.iter().skip(i + 1) {
                let diff_row = another_position.row - position.row;
                let diff_col = another_position.col - position.col;

                {
                    let resulting_position = Position {
                        row: position.row - diff_row,
                        col: position.col - diff_col,
                    };

                    if resulting_position.is_valid(row_count, col_count) {
                        antinodes.insert(resulting_position);
                    }
                }

                {
                    let resulting_position = Position {
                        row: another_position.row + diff_row,
                        col: another_position.col + diff_col,
                    };

                    if resulting_position.is_valid(row_count, col_count) {
                        antinodes.insert(resulting_position);
                    }
                }
            }
        }
    }

    antinodes.len().to_string()
}

pub fn solve_part2(path: &Path) -> String {
    let (antennas, row_count, col_count) = parse_file(path);

    let mut antinodes: HashSet<Position> = HashSet::new();

    for antenna_positions in antennas.values() {
        for (i, position) in antenna_positions.iter().enumerate() {
            for another_position in antenna_positions.iter().skip(i + 1) {
                antinodes.insert(position.clone());
                antinodes.insert(another_position.clone());

                let diff_row = another_position.row - position.row;
                let diff_col = another_position.col - position.col;

                {
                    let mut resulting_position = Position {
                        row: position.row - diff_row,
                        col: position.col - diff_col,
                    };

                    while resulting_position.is_valid(row_count, col_count) {
                        antinodes.insert(resulting_position.clone());

                        resulting_position = Position {
                            row: resulting_position.row - diff_row,
                            col: resulting_position.col - diff_col,
                        };
                    }
                }

                {
                    let mut resulting_position = Position {
                        row: another_position.row + diff_row,
                        col: another_position.col + diff_col,
                    };

                    while resulting_position.is_valid(row_count, col_count) {
                        antinodes.insert(resulting_position.clone());

                        resulting_position = Position {
                            row: resulting_position.row + diff_row,
                            col: resulting_position.col + diff_col,
                        };
                    }
                }
            }
        }
    }

    antinodes.len().to_string()
}
