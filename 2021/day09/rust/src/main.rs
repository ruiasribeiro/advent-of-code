use std::{
    collections::{HashSet, VecDeque},
    fs,
};

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

    let low_points = height_map
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(j, &height)| {
                    let lowest = get_neighbours(&height_map, (i, j))
                        .iter()
                        .filter_map(|&pos| pos)
                        .min()
                        .unwrap();

                    if height < lowest {
                        Some((i, j))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut basin_sizes = vec![];

    for low_point in low_points {
        // Locations that have been checked to be part of the basin.
        let mut verified_locations = HashSet::new();
        // Locations yet to be checked.
        let mut potential_locations = VecDeque::new();

        potential_locations.push_back(low_point);

        while let Some((ni, nj)) = potential_locations.pop_front() {
            if height_map[ni][nj] != 9 {
                verified_locations.insert((ni, nj));

                for neighbour in get_neighbour_locations(&height_map, (ni, nj))
                    .into_iter()
                    .flatten()
                {
                    if !verified_locations.contains(&neighbour) {
                        potential_locations.push_back(neighbour)
                    }
                }
            }
        }

        basin_sizes.push(verified_locations.len())
    }

    basin_sizes.sort();
    match basin_sizes[..] {
        [.., a, b, c] => println!("Part 2: {}", a * b * c),
        _ => panic!("Less than 3 basins were found"),
    }
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

fn get_neighbour_locations(
    height_map: &[Vec<u32>],
    (l, c): (usize, usize),
) -> [Option<(usize, usize)>; 4] {
    [
        if l > 0 { Some((l - 1, c)) } else { None },
        if l < height_map.len() - 1 {
            Some((l + 1, c))
        } else {
            None
        },
        if c > 0 { Some((l, c - 1)) } else { None },
        if c < height_map.first().unwrap().len() - 1 {
            Some((l, c + 1))
        } else {
            None
        },
    ]
}
