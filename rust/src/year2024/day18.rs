use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

struct Simulation {
    space: Vec<Vec<bool>>,
}

impl Simulation {
    fn from_file(path: &Path) -> Self {
        let is_full_input = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .contains("input");

        let (size, falls) = if is_full_input { (71, 1024) } else { (7, 12) };

        let mut space: Vec<Vec<bool>> = (0..size)
            .map(|_| (0..size).map(|_| true).collect())
            .collect();

        let file = File::open(path).unwrap();

        let incoming = BufReader::new(file).lines().take(falls).map(|line| {
            let line = line.unwrap();

            let (col, row) = line.split_once(',').unwrap();

            let row = row.parse::<usize>().unwrap();
            let col = col.parse::<usize>().unwrap();

            Position { row, col }
        });

        for Position { row, col } in incoming {
            space[row][col] = false;
        }

        Simulation { space }
    }

    fn get_successors(&self, position: Position) -> Vec<Position> {
        let mut successors = vec![];

        let row_count = self.space.len();
        let col_count = self.space[0].len();

        let Position { row, col } = position;

        if row > 0 && self.space[row - 1][col] {
            successors.push(Position { row: row - 1, col });
        }

        if col > 0 && self.space[row][col - 1] {
            successors.push(Position { row, col: col - 1 });
        }

        if row < row_count - 1 && self.space[row + 1][col] {
            successors.push(Position { row: row + 1, col });
        }

        if col < col_count - 1 && self.space[row][col + 1] {
            successors.push(Position { row, col: col + 1 });
        }

        successors
    }
}

pub fn solve_part1(path: &Path) -> String {
    let simulation = Simulation::from_file(path);

    let start = Position { row: 0, col: 0 };
    let end = Position {
        row: simulation.space.len() - 1,
        col: simulation.space[0].len() - 1,
    };

    let result = pathfinding::prelude::bfs(
        &start,
        |position| simulation.get_successors(*position),
        |position| *position == end,
    );

    (result.unwrap().len() - 1).to_string()
}

struct SimulationV2 {
    space: Vec<Vec<bool>>,
    incoming: VecDeque<Position>,
}

impl SimulationV2 {
    fn from_file(path: &Path) -> Self {
        let is_full_input = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .contains("input");

        let size = if is_full_input { 71 } else { 7 };

        let space: Vec<Vec<bool>> = (0..size)
            .map(|_| (0..size).map(|_| true).collect())
            .collect();

        let file = File::open(path).unwrap();

        let incoming = BufReader::new(file)
            .lines()
            .map(|line| {
                let line = line.unwrap();

                let (col, row) = line.split_once(',').unwrap();

                let row = row.parse::<usize>().unwrap();
                let col = col.parse::<usize>().unwrap();

                Position { row, col }
            })
            .collect();

        SimulationV2 { space, incoming }
    }

    fn get_successors(&self, position: Position) -> Vec<Position> {
        let mut successors = vec![];

        let row_count = self.space.len();
        let col_count = self.space[0].len();

        let Position { row, col } = position;

        if row > 0 && self.space[row - 1][col] {
            successors.push(Position { row: row - 1, col });
        }

        if col > 0 && self.space[row][col - 1] {
            successors.push(Position { row, col: col - 1 });
        }

        if row < row_count - 1 && self.space[row + 1][col] {
            successors.push(Position { row: row + 1, col });
        }

        if col < col_count - 1 && self.space[row][col + 1] {
            successors.push(Position { row, col: col + 1 });
        }

        successors
    }

    fn simulate_next_byte(&mut self) -> Position {
        let position @ Position { row, col } = self.incoming.pop_front().unwrap();

        self.space[row][col] = false;

        position
    }
}

pub fn solve_part2(path: &Path) -> String {
    let mut simulation = SimulationV2::from_file(path);

    let start = Position { row: 0, col: 0 };
    let end = Position {
        row: simulation.space.len() - 1,
        col: simulation.space[0].len() - 1,
    };

    loop {
        let Position { row, col } = simulation.simulate_next_byte();

        let result = pathfinding::prelude::bfs(
            &start,
            |position| simulation.get_successors(*position),
            |position| *position == end,
        );

        if result.is_none() {
            return format!("{col},{row}");
        }
    }
}
