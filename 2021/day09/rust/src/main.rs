use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");

    let height_map: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let sum: u32 = height_map
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, &height)| {
                    let lowest = get_neighbours(&height_map, (i, j))
                        .iter()
                        .filter_map(|&pos| pos)
                        .min()
                        .unwrap();

                    if height < lowest {
                        height + 1
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum();

    println!("Part 1: {}", sum);
}

fn get_neighbours(height_map: &[Vec<u32>], (l, c): (usize, usize)) -> [Option<u32>; 4] {
    [
        if l > 0 {
            Some(height_map[l - 1][c])
        } else {
            None
        },
        if l < height_map.len() - 1 {
            Some(height_map[l + 1][c])
        } else {
            None
        },
        if c > 0 {
            Some(height_map[l][c - 1])
        } else {
            None
        },
        if c < height_map.first().unwrap().len() - 1 {
            Some(height_map[l][c + 1])
        } else {
            None
        },
    ]
}
