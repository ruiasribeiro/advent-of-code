use std::{collections::HashMap, fs};

struct LanternFish {
    fishes: HashMap<u8, usize>,
}

impl LanternFish {
    fn new() -> Self {
        Self {
            fishes: HashMap::new(),
        }
    }

    fn add(&mut self, days_left: u8) {
        *self.fishes.entry(days_left).or_insert(0) += 1;
    }

    fn next_day(&mut self) {
        let mut new_map = HashMap::new();

        self.fishes.iter().for_each(|(k, v)| {
            if *k == 0 {
                *new_map.entry(6).or_insert(0) += *v;
                *new_map.entry(8).or_insert(0) += *v;
            } else {
                *new_map.entry(*k - 1).or_insert(0) += *v;
            }
        });

        self.fishes = new_map;
    }

    fn count(&self) -> usize {
        self.fishes.values().sum()
    }
}

impl Clone for LanternFish {
    fn clone(&self) -> Self {
        Self {
            fishes: self.fishes.clone(),
        }
    }
}

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");

    let mut fishes = LanternFish::new();

    contents
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .for_each(|v| fishes.add(v));

    let mut part2 = fishes.clone();

    for _ in 1..=80 {
        fishes.next_day();
    }

    println!("Part 1: {}", fishes.count());

    for _ in 1..=256 {
        part2.next_day();
    }

    println!("Part 2: {}", part2.count());
}
