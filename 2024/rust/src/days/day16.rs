use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    row: usize,
    col: usize,
    dir: Direction,
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn get_turns_required(from: Direction, to: Direction) -> u8 {
        if (from == Direction::Up && to == Direction::Left)
            || (from == Direction::Left && to == Direction::Up)
        {
            return 1;
        }

        let from = from as u8;
        let to = to as u8;

        match from.cmp(&to) {
            std::cmp::Ordering::Less => to - from,
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => from - to,
        }
    }

    fn get_step(self) -> (i8, i8) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Empty,
    End,
    Visited { score: u64 },
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            'E' => Ok(Self::End),
            '#' => Ok(Self::Wall),
            _ => Err(anyhow::anyhow!("unexpected char: {}", value)),
        }
    }
}

type Map = Vec<Vec<Tile>>;

fn parse_file(path: &Path) -> (Map, Position) {
    let mut starting_position = None;

    let file = File::open(path).unwrap();

    let map = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        starting_position = Some(Position {
                            row,
                            col,
                            dir: Direction::Right,
                        });

                        return Tile::Empty;
                    }

                    Tile::try_from(c).unwrap()
                })
                .collect()
        })
        .collect();

    let starting_position = starting_position.unwrap();

    (map, starting_position)
}

fn generate_possible_movements(map: &Map, current_position: &Position) -> Vec<(u64, Position)> {
    let mut movements = vec![];

    let Position { row, col, dir } = *current_position;

    let Tile::Visited {
        score: accumulated_score,
    } = map[row][col]
    else {
        panic!()
    };

    for d in [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ] {
        let (row_step, col_step) = d.get_step();

        let r = usize::try_from(i32::try_from(row).unwrap() + i32::from(row_step)).unwrap();
        let c = usize::try_from(i32::try_from(col).unwrap() + i32::from(col_step)).unwrap();

        if map[r][c] == Tile::Empty || map[r][c] == Tile::End {
            let turns = Direction::get_turns_required(dir, d);
            let score = accumulated_score + 1 + (1000 * u64::from(turns));

            movements.push((
                score,
                Position {
                    row: r,
                    col: c,
                    dir: d,
                },
            ));
        }
    }

    movements
}

pub fn solve_part1(path: &Path) -> String {
    let (mut map, starting_position) = parse_file(path);

    let mut possible_paths = BinaryHeap::new();
    possible_paths.push(Reverse((0, starting_position)));

    while let Some(Reverse((score, position))) = possible_paths.pop() {
        let Position { row, col, .. } = position;

        if map[row][col] == Tile::End {
            return score.to_string();
        }

        map[row][col] = Tile::Visited { score };

        generate_possible_movements(&map, &position)
            .iter()
            .for_each(|movement| possible_paths.push(Reverse(*movement)));
    }

    unreachable!()
}

pub fn solve_part2(_path: &Path) -> String {
    String::from("to do")
}
