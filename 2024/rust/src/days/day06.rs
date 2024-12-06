use std::{
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone, PartialEq)]
enum PositionType {
    Unvisited,
    Obstruction,
    Visited(HashSet<Direction>),
}

#[derive(Clone)]
struct Position {
    row: usize,
    col: usize,
    direction: Direction,
}

#[derive(PartialEq)]
enum EndingReason {
    StuckInLoop,
    OutOfBounds,
}

#[derive(Clone)]
struct Map {
    current_position: Position,
    map: Vec<Vec<PositionType>>,
}

impl Map {
    fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let mut current_position = None;

        let map: Vec<Vec<PositionType>> = BufReader::new(file)
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        '.' => PositionType::Unvisited,
                        '#' => PositionType::Obstruction,
                        '^' => {
                            current_position = Some(Position {
                                row,
                                col,
                                direction: Direction::Up,
                            });
                            PositionType::Unvisited
                        }
                        c => panic!("unexpected value while reading file: got '{c}'"),
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

    fn generate_all_obstruction_possibilities(path: &str) -> impl Iterator<Item = Map> {
        let original_map = Self::from_file(path);
        let row_count = original_map.map.len();
        let col_count = original_map.map[0].len();

        let Position {
            row: starting_row,
            col: starting_col,
            direction: _,
        } = original_map.current_position;

        (0..row_count)
            .flat_map(move |row| (0..col_count).map(move |col| (row, col)))
            .filter_map(move |(row, col)| {
                if let PositionType::Unvisited = original_map.map[row][col] {
                    if row != starting_row || col != starting_col {
                        let mut copy = original_map.clone();
                        copy.map[row][col] = PositionType::Obstruction;
                        return Some(copy);
                    }
                }

                None
            })
    }

    fn step(&mut self) -> Result<(), EndingReason> {
        let Position {
            row,
            col,
            direction,
        } = self.current_position;

        let updated_position = match self.map.get(row).unwrap().get(col).unwrap() {
            PositionType::Unvisited => PositionType::Visited({
                let mut set = HashSet::new();
                set.insert(direction);
                set
            }),
            PositionType::Visited(set) => PositionType::Visited({
                let mut set = set.clone();
                set.insert(direction);
                set
            }),
            PositionType::Obstruction => unreachable!(),
        };

        self.map[row][col] = updated_position;

        match self.get_next_position() {
            Ok(next_position) => {
                self.current_position = next_position;
                Ok(())
            }
            Err(reason) => Err(reason),
        }
    }

    fn get_next_position(&self) -> Result<Position, EndingReason> {
        let Position {
            row,
            col,
            direction,
        } = self.current_position;

        let row_count = self.map.len();
        let col_count = self.map[0].len();

        if (row == 0 && direction == Direction::Up)
            || (row == row_count - 1 && direction == Direction::Down)
            || (col == 0 && direction == Direction::Left)
            || (col == col_count - 1 && direction == Direction::Right)
        {
            return Err(EndingReason::OutOfBounds);
        }

        let (next_row, next_col) = match direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        };

        match &self.map[next_row][next_col] {
            PositionType::Unvisited => Ok(Position {
                row: next_row,
                col: next_col,
                direction,
            }),

            PositionType::Obstruction => {
                let mut new_direction = direction.turn();

                while new_direction != direction {
                    let (next_row, next_col) = match new_direction {
                        Direction::Up => (row - 1, col),
                        Direction::Down => (row + 1, col),
                        Direction::Left => (row, col - 1),
                        Direction::Right => (row, col + 1),
                    };

                    if let PositionType::Obstruction = &self.map[next_row][next_col] {
                        new_direction = new_direction.turn();
                        continue;
                    } else if let PositionType::Visited(set) = &self.map[next_row][next_col] {
                        if set.contains(&new_direction) {
                            return Err(EndingReason::StuckInLoop);
                        }
                    }

                    return Ok(Position {
                        row: next_row,
                        col: next_col,
                        direction: new_direction,
                    });
                }

                unreachable!()
            }

            PositionType::Visited(set) => {
                if set.contains(&direction) {
                    Err(EndingReason::StuckInLoop)
                } else {
                    Ok(Position {
                        row: next_row,
                        col: next_col,
                        direction,
                    })
                }
            }
        }
    }

    fn count_distinct_visited_positions(&self) -> usize {
        self.map
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|position| matches!(position, PositionType::Visited { .. }))
                    .count()
            })
            .sum()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
            for position in row {
                match position {
                    PositionType::Unvisited => write!(f, ".")?,
                    PositionType::Obstruction => write!(f, "#")?,
                    PositionType::Visited(_) => write!(f, "O")?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn solve_part1(path: &str) -> i32 {
    let mut map = Map::from_file(path);

    while let Ok(()) = map.step() {}

    map.count_distinct_visited_positions() as i32
}

pub fn solve_part2(path: &str) -> i32 {
    Map::generate_all_obstruction_possibilities(path)
        .map(|map| {
            let mut map = map.to_owned();

            let mut result = Ok(());

            while result.is_ok() {
                result = map.step();
            }

            result.unwrap_err()
        })
        .filter(|reason| *reason == EndingReason::StuckInLoop)
        .count() as i32
}
