use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    row: i32,
    col: i32,
}

struct Velocity {
    row: i32,
    col: i32,
}

struct Robot {
    position: Position,
    velocity: Velocity,
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s
            .split_once(' ')
            .ok_or(anyhow::anyhow!("input string badly formatted"))?;

        let position = {
            let (x, y) = position[2..]
                .split_once(',')
                .ok_or(anyhow::anyhow!("input string badly formatted"))?;

            Position {
                row: y.parse()?,
                col: x.parse()?,
            }
        };

        let velocity = {
            let (x, y) = velocity[2..]
                .split_once(',')
                .ok_or(anyhow::anyhow!("input string badly formatted"))?;

            Velocity {
                row: y.parse()?,
                col: x.parse()?,
            }
        };

        Ok(Self { position, velocity })
    }
}

pub fn solve_part1(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let is_full_input = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .contains("input");

    let (row_count, col_count) = if is_full_input { (103, 101) } else { (7, 11) };

    let mut robots: Vec<Robot> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<_>>();

    let iterations = 100;

    for robot in &mut robots {
        let Position { row, col } = robot.position;

        let Velocity {
            row: vel_row,
            col: vel_col,
        } = robot.velocity;

        robot.position.row = {
            let mut final_row = row + vel_row * iterations;

            while final_row < 0 {
                final_row += row_count;
            }

            final_row % row_count
        };

        robot.position.col = {
            let mut next_col = col + vel_col * iterations;

            while next_col < 0 {
                next_col += col_count;
            }

            next_col % col_count
        };
    }

    let middle_row = row_count / 2;
    let middle_col = col_count / 2;

    let (quad1, quad2, quad3, quad4) = robots.iter().fold(
        (0, 0, 0, 0),
        |(quad1, quad2, quad3, quad4), robot| match &robot.position {
            position if position.row < middle_row && position.col < middle_col => {
                (quad1 + 1, quad2, quad3, quad4)
            }
            position if position.row > middle_row && position.col < middle_col => {
                (quad1, quad2 + 1, quad3, quad4)
            }
            position if position.row > middle_row && position.col > middle_col => {
                (quad1, quad2, quad3 + 1, quad4)
            }
            position if position.row < middle_row && position.col > middle_col => {
                (quad1, quad2, quad3, quad4 + 1)
            }
            _ => (quad1, quad2, quad3, quad4),
        },
    );

    (quad1 * quad2 * quad3 * quad4).to_string()
}

pub fn solve_part2(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let is_full_input = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .contains("input");

    if !is_full_input {
        return String::from("N/A");
    }

    let (row_count, col_count) = (103, 101);

    // Determined manually by visually checking which iterations would show a
    // vertical strip of robots. This will likely be different for other inputs.
    let col_period_offset = 89;

    // The Christmas tree frame.
    let (frame_rows, frame_cols) = (33, 31);

    let mut robots: Vec<Robot> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<_>>();

    for robot in &mut robots {
        let Position { row, col } = robot.position;

        let Velocity {
            row: vel_row,
            col: vel_col,
        } = robot.velocity;

        robot.position.row = {
            let mut final_row = row + vel_row * col_period_offset;

            while final_row < 0 {
                final_row += row_count;
            }

            final_row % row_count
        };

        robot.position.col = {
            let mut next_col = col + vel_col * col_period_offset;

            while next_col < 0 {
                next_col += col_count;
            }

            next_col % col_count
        };
    }

    let max_iterations = row_count * col_count;

    for i in (col_period_offset..max_iterations).step_by(col_count.try_into().unwrap()) {
        let positions = robots
            .iter()
            .map(|robot| robot.position)
            .collect::<HashSet<_>>();

        for row in 0..(row_count - frame_rows) {
            for col in 0..(col_count - frame_cols) {
                let rows_with_robots = ((row + 1)..(row + frame_rows))
                    .filter(|r| positions.contains(&Position { row: *r, col }))
                    .count();

                let cols_with_robots = ((col + 1)..(col + frame_cols))
                    .filter(|c| positions.contains(&Position { row, col: *c }))
                    .count();

                if rows_with_robots as i32 == frame_rows - 1
                    && cols_with_robots as i32 == frame_cols - 1
                {
                    return i.to_string();
                }
            }
        }

        for robot in &mut robots {
            let Position { row, col } = robot.position;

            let Velocity {
                row: vel_row,
                col: vel_col,
            } = robot.velocity;

            robot.position.row = {
                let mut final_row = row + vel_row * col_count;

                while final_row < 0 {
                    final_row += row_count;
                }

                final_row % row_count
            };

            robot.position.col = {
                let mut next_col = col + vel_col * col_count;

                while next_col < 0 {
                    next_col += col_count;
                }

                next_col % col_count
            };
        }
    }

    unreachable!()
}
