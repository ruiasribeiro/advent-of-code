use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

type Map = Vec<Vec<char>>;
type Region = HashSet<Position>;

fn extract_regions(mut map: Map) -> Vec<Region> {
    let mut regions = vec![];

    let row_count = map.len();
    let col_count = map[0].len();

    for row in 0..row_count {
        for col in 0..col_count {
            let plant = map[row][col];

            if plant == '.' {
                continue;
            }

            let mut queue = VecDeque::new();
            queue.push_back(Position { row, col });

            let mut region = HashSet::new();

            while let Some(Position {
                row: pos_row,
                col: pos_col,
            }) = queue.pop_front()
            {
                region.insert(Position {
                    row: pos_row,
                    col: pos_col,
                });

                if pos_row > 0 && map[pos_row - 1][pos_col] == plant {
                    map[pos_row - 1][pos_col] = '.';

                    queue.push_back(Position {
                        row: pos_row - 1,
                        col: pos_col,
                    });
                }

                if pos_col > 0 && map[pos_row][pos_col - 1] == plant {
                    map[pos_row][pos_col - 1] = '.';

                    queue.push_back(Position {
                        row: pos_row,
                        col: pos_col - 1,
                    });
                }

                if pos_row < row_count - 1 && map[pos_row + 1][pos_col] == plant {
                    map[pos_row + 1][pos_col] = '.';

                    queue.push_back(Position {
                        row: pos_row + 1,
                        col: pos_col,
                    });
                }

                if pos_col < col_count - 1 && map[pos_row][pos_col + 1] == plant {
                    map[pos_row][pos_col + 1] = '.';

                    queue.push_back(Position {
                        row: pos_row,
                        col: pos_col + 1,
                    });
                }
            }

            regions.push(region);
        }
    }

    regions
}

fn calculate_price(region: &Region, row_count: usize, col_count: usize) -> i32 {
    let mut area = 0;
    let mut perimeter = 0;

    for plot in region {
        area += 1;

        if plot.row == 0
            || !region.contains(&Position {
                row: plot.row - 1,
                col: plot.col,
            })
        {
            perimeter += 1;
        }

        if plot.col == 0
            || !region.contains(&Position {
                row: plot.row,
                col: plot.col - 1,
            })
        {
            perimeter += 1;
        }

        if plot.row == row_count - 1
            || !region.contains(&Position {
                row: plot.row + 1,
                col: plot.col,
            })
        {
            perimeter += 1;
        }

        if plot.col == col_count - 1
            || !region.contains(&Position {
                row: plot.row,
                col: plot.col + 1,
            })
        {
            perimeter += 1;
        }
    }

    area * perimeter
}

pub fn solve_part1(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let map: Map = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let row_count = map.len();
    let col_count = map[0].len();

    extract_regions(map)
        .iter()
        .fold(0, |total_price, region| {
            total_price + calculate_price(region, row_count, col_count)
        })
        .to_string()
}

fn calculate_corners(position: &Position, map: &Map) -> i32 {
    let Position { row, col } = *position;
    let plot = map[row][col];

    let rows = map.len();
    let cols = map[0].len();

    let has_top_edge = row == 0 || (row > 0 && map[row - 1][col] != plot);
    let has_bottom_edge = row == rows - 1 || (row < rows - 1 && map[row + 1][col] != plot);
    let has_left_edge = col == 0 || (col > 0 && map[row][col - 1] != plot);
    let has_right_edge = col == cols - 1 || (col < cols - 1 && map[row][col + 1] != plot);

    let edges = i32::from(has_top_edge)
        + i32::from(has_bottom_edge)
        + i32::from(has_left_edge)
        + i32::from(has_right_edge);

    if edges == 4 {
        return 4;
    }

    if edges == 3 {
        return 2;
    }

    if edges == 2 && ((has_left_edge && has_right_edge) || (has_top_edge && has_bottom_edge)) {
        return 0;
    }

    let mut inner_corners = 0;

    if !has_bottom_edge
        && !has_right_edge
        && row < rows - 1
        && col < cols - 1
        && map[row + 1][col + 1] != plot
    {
        inner_corners += 1;
    }

    if !has_bottom_edge
        && !has_left_edge
        && row < rows - 1
        && col > 0
        && map[row + 1][col - 1] != plot
    {
        inner_corners += 1;
    }

    if !has_top_edge && !has_left_edge && row > 0 && col > 0 && map[row - 1][col - 1] != plot {
        inner_corners += 1;
    }

    if !has_top_edge
        && !has_right_edge
        && row > 0
        && col < cols - 1
        && map[row - 1][col + 1] != plot
    {
        inner_corners += 1;
    }

    if edges == 2 {
        1 + inner_corners
    } else {
        inner_corners
    }
}

pub fn solve_part2(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let map: Map = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    extract_regions(map.clone())
        .iter()
        .fold(0, |total_price, region| {
            let mut area = 0;
            let mut sides = 0;

            for plot in region {
                area += 1;
                sides += calculate_corners(plot, &map);
            }

            total_price + area * sides
        })
        .to_string()
}
