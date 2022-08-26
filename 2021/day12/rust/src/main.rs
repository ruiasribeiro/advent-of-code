// There are a few things at the top of my head that could be improved:
//
//  - avoid checking every single time if a cave is big or small (perhaps it
//  could be achieved by storing in an enum at the time of parsing)
//
//  - avoid cloning the strings every time a possible path is generated

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");

    let mut cave: HashMap<String, HashSet<String>> = HashMap::new();

    for line in contents.lines() {
        let (orig, dest) = line.split_once('-').unwrap();
        let (orig, dest) = (orig.to_owned(), dest.to_owned());

        cave.entry(orig.clone()).or_default().insert(dest.clone());
        cave.entry(dest.clone()).or_default().insert(orig.clone());
    }

    let cave = cave;

    // Part 1

    let mut path_queue: VecDeque<Vec<String>> = cave["start"]
        .iter()
        .map(|dest| vec!["start".to_owned(), dest.to_owned()])
        .collect();

    let mut complete_paths = 0;

    while let Some(path) = path_queue.pop_front() {
        let current_position = path.last().unwrap();

        let possible_paths = cave[current_position]
            .iter()
            .filter(|position| {
                !path.contains(position) || position.chars().any(|char| char.is_ascii_uppercase())
            })
            .map(|position| {
                let mut possible_path = path.clone();
                possible_path.push(position.to_owned());
                possible_path
            });

        for path in possible_paths {
            if path.last().unwrap() == "end" {
                complete_paths += 1;
            } else {
                path_queue.push_back(path);
            }
        }
    }

    println!("Part 1: {}", complete_paths);

    // Part 2

    // A queue of pairs of possible paths + if a small cave has been visited twice
    let mut path_queue: VecDeque<(Vec<String>, bool)> = cave["start"]
        .iter()
        .map(|dest| (vec!["start".to_owned(), dest.to_owned()], false))
        .collect();

    let mut complete_paths = 0;

    while let Some((path, twice)) = path_queue.pop_front() {
        let current_position = path.last().unwrap();

        let possible_paths = cave[current_position]
            .iter()
            .filter(|&position| {
                (position != "start" && !twice)
                    || !path.contains(position)
                    || position.chars().any(|char| char.is_ascii_uppercase())
            })
            .map(|position| {
                let mut possible_path = path.clone();
                let mut twice = twice;

                if !twice
                    && position.chars().any(|char| char.is_ascii_lowercase())
                    && possible_path.contains(position)
                {
                    twice = true;
                }

                possible_path.push(position.to_owned());
                (possible_path, twice)
            });

        for (path, twice) in possible_paths {
            if path.last().unwrap() == "end" {
                complete_paths += 1;
            } else {
                path_queue.push_back((path, twice));
            }
        }
    }

    println!("Part 2: {}", complete_paths);
}
