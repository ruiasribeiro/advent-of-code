use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

enum Schematic {
    Lock(u8, u8, u8, u8, u8),
    Key(u8, u8, u8, u8, u8),
}

fn parse_schematic(lines: &[String]) -> Schematic {
    let first = lines.first().unwrap();
    let last = lines.last().unwrap();

    let grid = lines
        .iter()
        .skip(1)
        .take(5)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let heights = (0..5)
        .map(|col| (0..5).filter(|row| grid[*row][col] == '#').count() as u8)
        .collect::<Vec<_>>();

    match (first.as_str(), last.as_str()) {
        ("#####", ".....") => {
            Schematic::Lock(heights[0], heights[1], heights[2], heights[3], heights[4])
        }

        (".....", "#####") => {
            Schematic::Key(heights[0], heights[1], heights[2], heights[3], heights[4])
        }

        _ => unreachable!(),
    }
}

pub fn solve_part1(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let schematics = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(parse_schematic)
        .collect::<Vec<_>>();

    let locks = schematics
        .iter()
        .filter(|schematic| matches!(schematic, Schematic::Lock(_, _, _, _, _)))
        .collect::<Vec<_>>();

    let keys = schematics
        .iter()
        .filter(|schematic| matches!(schematic, Schematic::Key(_, _, _, _, _)))
        .collect::<Vec<_>>();

    let mut unique_pairs = 0;

    for lock in &locks {
        for key in &keys {
            let Schematic::Lock(lock0, lock1, lock2, lock3, lock4) = lock else {
                unreachable!()
            };

            let Schematic::Key(key0, key1, key2, key3, key4) = key else {
                unreachable!()
            };

            if lock0 + key0 <= 5
                && lock1 + key1 <= 5
                && lock2 + key2 <= 5
                && lock3 + key3 <= 5
                && lock4 + key4 <= 5
            {
                unique_pairs += 1;
            }
        }
    }

    unique_pairs.to_string()
}

pub fn solve_part2(_path: &Path) -> String {
    "to do".into()
}
