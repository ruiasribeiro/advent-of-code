use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Eq, PartialEq)]
enum MapTile {
    Box,
    Empty,
    Wall,
}

impl TryFrom<char> for MapTile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Self::Box),
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            _ => Err(anyhow::anyhow!("unexpected char: {}", value)),
        }
    }
}

#[derive(Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

struct Map {
    current_position: Position,
    map: Vec<Vec<MapTile>>,
}

fn parse_map(lines: &[String]) -> Map {
    let mut current_position = None;

    let map = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == '@' {
                        current_position = Some(Position { row, col });
                        return MapTile::Empty;
                    }

                    MapTile::try_from(c).unwrap()
                })
                .collect()
        })
        .collect();

    let current_position = current_position.unwrap();

    Map {
        current_position,
        map,
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Movement {
    Up,
    Right,
    Down,
    Left,
}

impl Movement {
    fn step(self) -> (i8, i8) {
        match self {
            Movement::Up => (-1, 0),
            Movement::Right => (0, 1),
            Movement::Down => (1, 0),
            Movement::Left => (0, -1),
        }
    }
}

impl TryFrom<char> for Movement {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            '>' => Ok(Self::Right),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            _ => Err(anyhow::anyhow!("unexpected char: {}", value)),
        }
    }
}

fn parse_movements(lines: &[String]) -> Vec<Movement> {
    lines
        .iter()
        .flat_map(|line| line.chars().map(|c| Movement::try_from(c).unwrap()))
        .collect()
}

pub fn solve_part1(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let lines = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let (map, movements) = lines.split_at(lines.iter().position(String::is_empty).unwrap());
    let movements = &movements[1..];

    let mut map = parse_map(map);
    let movements = parse_movements(movements);

    for movement in movements {
        let Position { row, col } = map.current_position;
        let step = movement.step();

        let Position {
            row: next_row,
            col: next_col,
        } = Position {
            row: usize::try_from(i32::try_from(row).unwrap() + i32::from(step.0)).unwrap(),
            col: usize::try_from(i32::try_from(col).unwrap() + i32::from(step.1)).unwrap(),
        };

        match map.map[next_row].get(next_col).unwrap() {
            MapTile::Wall => (), // do nothing

            MapTile::Empty => {
                map.current_position.row = next_row;
                map.current_position.col = next_col;
            }

            MapTile::Box => {
                let Position {
                    row: mut iter_row,
                    col: mut iter_col,
                } = Position {
                    row: next_row,
                    col: next_col,
                };

                while map.map[iter_row][iter_col] == MapTile::Box {
                    iter_row =
                        usize::try_from(i32::try_from(iter_row).unwrap() + i32::from(step.0))
                            .unwrap();
                    iter_col =
                        usize::try_from(i32::try_from(iter_col).unwrap() + i32::from(step.1))
                            .unwrap();
                }

                match map.map[iter_row].get(iter_col).unwrap() {
                    MapTile::Box => unreachable!(),
                    MapTile::Wall => (),

                    MapTile::Empty => {
                        map.map[iter_row][iter_col] = MapTile::Box;
                        map.map[next_row][next_col] = MapTile::Empty;

                        map.current_position.row = next_row;
                        map.current_position.col = next_col;
                    }
                };
            }
        };
    }

    map.map
        .iter()
        .enumerate()
        .map(|(row, tiles)| {
            tiles
                .iter()
                .enumerate()
                .map(|(col, tile)| {
                    if *tile == MapTile::Box {
                        (100 * row) + col
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum MapTileV2 {
    BoxLeftSide,
    BoxRightSide,
    Empty,
    Wall,
}

impl TryFrom<char> for MapTileV2 {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '[' => Ok(Self::BoxLeftSide),
            ']' => Ok(Self::BoxRightSide),
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            _ => Err(anyhow::anyhow!("unexpected char: {}", value)),
        }
    }
}

struct MapV2 {
    current_position: Position,
    map: Vec<Vec<MapTileV2>>,
}

fn parse_map_v2(lines: &[String]) -> MapV2 {
    let mut current_position = None;

    let map = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(col, c)| {
                    if c == '@' {
                        current_position = Some(Position { row, col: col * 2 });
                        return [MapTileV2::Empty, MapTileV2::Empty];
                    }

                    match MapTile::try_from(c).unwrap() {
                        MapTile::Box => [MapTileV2::BoxLeftSide, MapTileV2::BoxRightSide],
                        MapTile::Empty => [MapTileV2::Empty, MapTileV2::Empty],
                        MapTile::Wall => [MapTileV2::Wall, MapTileV2::Wall],
                    }
                })
                .collect()
        })
        .collect();

    let current_position = current_position.unwrap();

    MapV2 {
        current_position,
        map,
    }
}

fn try_moving_box(
    box_left_side_position: Position,
    movement: Movement,
    map: &mut Vec<Vec<MapTileV2>>,
) -> bool {
    let Position { row, col } = box_left_side_position;
    // let step = movement.step();

    // let Position {
    //     row: next_row,
    //     col: next_col,
    // } = Position {
    //     row: usize::try_from(i32::try_from(row).unwrap() + i32::from(step.0)).unwrap(),
    //     col: usize::try_from(i32::try_from(col).unwrap() + i32::from(step.1)).unwrap(),
    // };

    if movement == Movement::Up {
        return match (map[row - 1][col], map[row - 1][col + 1]) {
            (MapTileV2::Wall, _) | (_, MapTileV2::Wall) => false,

            (MapTileV2::Empty, MapTileV2::Empty) => {
                map[row - 1][col] = MapTileV2::BoxLeftSide;
                map[row - 1][col + 1] = MapTileV2::BoxRightSide;

                map[row][col] = MapTileV2::Empty;
                map[row][col + 1] = MapTileV2::Empty;

                true
            }

            (MapTileV2::BoxLeftSide, MapTileV2::Empty | MapTileV2::BoxLeftSide)
            | (MapTileV2::Empty | MapTileV2::BoxRightSide, MapTileV2::BoxRightSide) => {
                unreachable!()
            }

            (MapTileV2::BoxLeftSide, MapTileV2::BoxRightSide) => {
                let moved_box = try_moving_box(Position { row: row - 1, col }, movement, map);

                if moved_box {
                    map[row - 1][col] = MapTileV2::BoxLeftSide;
                    map[row - 1][col + 1] = MapTileV2::BoxRightSide;

                    map[row][col] = MapTileV2::Empty;
                    map[row][col + 1] = MapTileV2::Empty;

                    true
                } else {
                    false
                }
            }

            (MapTileV2::BoxRightSide, MapTileV2::BoxLeftSide) => {
                let moved_left_box = try_moving_box(
                    Position {
                        row: row - 1,
                        col: col - 1,
                    },
                    movement,
                    map,
                );

                let moved_right_box = try_moving_box(
                    Position {
                        row: row - 1,
                        col: col + 1,
                    },
                    movement,
                    map,
                );

                if moved_left_box && moved_right_box {
                    map[row - 1][col] = MapTileV2::BoxLeftSide;
                    map[row - 1][col + 1] = MapTileV2::BoxRightSide;

                    map[row][col] = MapTileV2::Empty;
                    map[row][col + 1] = MapTileV2::Empty;

                    true
                } else {
                    false
                }
            }

            (MapTileV2::BoxRightSide, MapTileV2::Empty) => {
                let moved_left_box = try_moving_box(
                    Position {
                        row: row - 1,
                        col: col - 1,
                    },
                    movement,
                    map,
                );

                if moved_left_box {
                    map[row - 1][col] = MapTileV2::BoxLeftSide;
                    map[row - 1][col + 1] = MapTileV2::BoxRightSide;

                    map[row][col] = MapTileV2::Empty;
                    map[row][col + 1] = MapTileV2::Empty;

                    true
                } else {
                    false
                }
            }

            (MapTileV2::Empty, MapTileV2::BoxLeftSide) => {
                let moved_right_box = try_moving_box(
                    Position {
                        row: row - 1,
                        col: col + 1,
                    },
                    movement,
                    map,
                );

                if moved_right_box {
                    map[row - 1][col] = MapTileV2::BoxLeftSide;
                    map[row - 1][col + 1] = MapTileV2::BoxRightSide;

                    map[row][col] = MapTileV2::Empty;
                    map[row][col + 1] = MapTileV2::Empty;

                    true
                } else {
                    false
                }
            }
        };
    }

    if movement == Movement::Down {
        return match (map[row + 1][col], map[row + 1][col + 1]) {
            (MapTileV2::Wall, _) | (_, MapTileV2::Wall) => false,

            (MapTileV2::Empty, MapTileV2::Empty) => {
                map[row + 1][col] = MapTileV2::BoxLeftSide;
                map[row + 1][col + 1] = MapTileV2::BoxRightSide;

                map[row][col] = MapTileV2::Empty;
                map[row][col + 1] = MapTileV2::Empty;

                true
            }

            (MapTileV2::BoxLeftSide, MapTileV2::Empty | MapTileV2::BoxLeftSide)
            | (MapTileV2::Empty | MapTileV2::BoxRightSide, MapTileV2::BoxRightSide) => {
                unreachable!()
            }

            (MapTileV2::BoxLeftSide, MapTileV2::BoxRightSide) => {
                let moved_box = try_moving_box(Position { row: row + 1, col }, movement, map);

                if moved_box {
                    map[row + 1][col] = MapTileV2::BoxLeftSide;
                    map[row + 1][col + 1] = MapTileV2::BoxRightSide;

                    map[row][col] = MapTileV2::Empty;
                    map[row][col + 1] = MapTileV2::Empty;

                    true
                } else {
                    false
                }
            }

            (MapTileV2::BoxRightSide, MapTileV2::BoxLeftSide) => {
                let moved_left_box = try_moving_box(
                    Position {
                        row: row + 1,
                        col: col - 1,
                    },
                    movement,
                    map,
                );

                let moved_right_box = try_moving_box(
                    Position {
                        row: row + 1,
                        col: col + 1,
                    },
                    movement,
                    map,
                );

                if moved_left_box && moved_right_box {
                    map[row + 1][col] = MapTileV2::BoxLeftSide;
                    map[row + 1][col + 1] = MapTileV2::BoxRightSide;

                    map[row][col] = MapTileV2::Empty;
                    map[row][col + 1] = MapTileV2::Empty;

                    true
                } else {
                    false
                }
            }

            (MapTileV2::BoxRightSide, MapTileV2::Empty) => {
                let moved_left_box = try_moving_box(
                    Position {
                        row: row + 1,
                        col: col - 1,
                    },
                    movement,
                    map,
                );

                if moved_left_box {
                    map[row + 1][col] = MapTileV2::BoxLeftSide;
                    map[row + 1][col + 1] = MapTileV2::BoxRightSide;

                    map[row][col] = MapTileV2::Empty;
                    map[row][col + 1] = MapTileV2::Empty;

                    true
                } else {
                    false
                }
            }

            (MapTileV2::Empty, MapTileV2::BoxLeftSide) => {
                let moved_right_box = try_moving_box(
                    Position {
                        row: row + 1,
                        col: col + 1,
                    },
                    movement,
                    map,
                );

                if moved_right_box {
                    map[row + 1][col] = MapTileV2::BoxLeftSide;
                    map[row + 1][col + 1] = MapTileV2::BoxRightSide;

                    map[row][col] = MapTileV2::Empty;
                    map[row][col + 1] = MapTileV2::Empty;

                    true
                } else {
                    false
                }
            }
        };
    }

    if movement == Movement::Left {
        return match map[row][col - 1] {
            MapTileV2::BoxLeftSide => unreachable!(),

            MapTileV2::BoxRightSide => {
                let moved_box = try_moving_box(Position { row, col: col - 2 }, movement, map);

                if moved_box {
                    map[row][col - 1] = MapTileV2::BoxLeftSide;
                    map[row][col] = MapTileV2::BoxRightSide;
                    map[row][col + 1] = MapTileV2::Empty;
                }

                moved_box
            }

            MapTileV2::Empty => {
                map[row][col - 1] = MapTileV2::BoxLeftSide;
                map[row][col] = MapTileV2::BoxRightSide;
                map[row][col + 1] = MapTileV2::Empty;

                true
            }

            MapTileV2::Wall => false,
        };
    }

    if movement == Movement::Right {
        return match map[row][col + 2] {
            MapTileV2::BoxRightSide => unreachable!(),

            MapTileV2::BoxLeftSide => {
                let moved_box = try_moving_box(Position { row, col: col + 2 }, movement, map);

                if moved_box {
                    map[row][col] = MapTileV2::Empty;
                    map[row][col + 1] = MapTileV2::BoxLeftSide;
                    map[row][col + 2] = MapTileV2::BoxRightSide;
                }

                moved_box
            }

            MapTileV2::Empty => {
                map[row][col] = MapTileV2::Empty;
                map[row][col + 1] = MapTileV2::BoxLeftSide;
                map[row][col + 2] = MapTileV2::BoxRightSide;

                true
            }

            MapTileV2::Wall => false,
        };
    }

    unreachable!()
}

pub fn solve_part2(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let lines = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let (map, movements) = lines.split_at(lines.iter().position(String::is_empty).unwrap());
    let movements = &movements[1..];

    let mut map = parse_map_v2(map);
    let movements = parse_movements(movements);

    for movement in movements {
        let Position { row, col } = map.current_position;
        let step = movement.step();

        let Position {
            row: next_row,
            col: next_col,
        } = Position {
            row: usize::try_from(i32::try_from(row).unwrap() + i32::from(step.0)).unwrap(),
            col: usize::try_from(i32::try_from(col).unwrap() + i32::from(step.1)).unwrap(),
        };

        match map.map[next_row].get(next_col).unwrap() {
            MapTileV2::Wall => (), // do nothing

            MapTileV2::Empty => {
                map.current_position.row = next_row;
                map.current_position.col = next_col;
            }

            MapTileV2::BoxLeftSide => {
                let mut map_copy = map.map.clone();

                let moved_box = try_moving_box(
                    Position {
                        row: next_row,
                        col: next_col,
                    },
                    movement,
                    &mut map_copy,
                );

                if moved_box {
                    map.map = map_copy;
                    map.current_position.row = next_row;
                    map.current_position.col = next_col;
                }
            }

            MapTileV2::BoxRightSide => {
                let mut map_copy = map.map.clone();

                let moved_box = try_moving_box(
                    Position {
                        row: next_row,
                        col: next_col - 1,
                    },
                    movement,
                    &mut map_copy,
                );

                if moved_box {
                    map.map = map_copy;
                    map.current_position.row = next_row;
                    map.current_position.col = next_col;
                }
            }
        };
    }

    map.map
        .iter()
        .enumerate()
        .map(|(row, tiles)| {
            tiles
                .iter()
                .enumerate()
                .map(|(col, tile)| {
                    if *tile == MapTileV2::BoxLeftSide {
                        (100 * row) + col
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}
