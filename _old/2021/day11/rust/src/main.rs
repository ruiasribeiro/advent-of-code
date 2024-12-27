use std::fs;

// Dimension of the grid.
const SIZE: usize = 10;

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");

    // Part 1

    let mut grid: Vec<Vec<_>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|char| char.to_digit(10))
                .map(Some)
                .collect()
        })
        .collect();

    assert!(grid.len() == SIZE);
    assert!(grid.iter().all(|row| row.len() == SIZE));

    let mut flashes = 0;
    for _ in 0..100 {
        increment_all(&mut grid);
        while flash(&mut grid) {
            flashes += 1;
        }
        reset_flashes(&mut grid);
    }

    println!("Part 1: {}", flashes);

    // Part 2

    let mut grid: Vec<Vec<_>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|char| char.to_digit(10))
                .map(Some)
                .collect()
        })
        .collect();

    for step in 1.. {
        increment_all(&mut grid);
        while flash(&mut grid) {
            flashes += 1;
        }

        if grid
            .iter()
            .all(|row| row.iter().all(|energy| energy.is_none()))
        {
            println!("Part 2: {}", step);
            return;
        }

        reset_flashes(&mut grid);
    }
}

fn increment_all(grid: &mut Vec<Vec<Option<u32>>>) {
    for row in grid {
        for energy in row.iter_mut().flatten() {
            *energy += 1;
        }
    }
}

fn flash(grid: &mut [Vec<Option<u32>>]) -> bool {
    for r in 0..SIZE {
        for c in 0..SIZE {
            if grid[r][c] > Some(9) {
                grid[r][c] = None;
                increment_neighbours(grid, (r, c));
                return true;
            }
        }
    }

    false
}

fn increment_neighbours(grid: &mut [Vec<Option<u32>>], (r, c): (usize, usize)) {
    let (r, c) = (r as isize, c as isize);
    let size = SIZE as isize;

    let neighbours = [
        (r - 1, c - 1),
        (r - 1, c),
        (r - 1, c + 1),
        (r, c - 1),
        (r, c + 1),
        (r + 1, c - 1),
        (r + 1, c),
        (r + 1, c + 1),
    ];

    for (nr, nc) in neighbours {
        if 0 <= nr && nr < size && 0 <= nc && nc < size {
            grid[nr as usize][nc as usize] =
                grid[nr as usize][nc as usize].map(|energy| energy + 1);
        }
    }
}

fn reset_flashes(grid: &mut [Vec<Option<u32>>]) {
    for row in grid {
        for energy in row {
            if energy.is_none() {
                *energy = Some(0);
            }
        }
    }
}
