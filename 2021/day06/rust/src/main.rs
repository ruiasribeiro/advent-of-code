use std::{fs, iter::repeat};

struct LanternFish {
    days_left: u8,
}

impl LanternFish {
    fn new() -> Self {
        Self { days_left: 8 }
    }

    fn new_with_days(days_left: u8) -> Self {
        Self { days_left }
    }

    // returns a bool that indicates if it created a new lanternfish
    fn next_day(&mut self) -> bool {
        if self.days_left == 0 {
            self.days_left = 6;
            true
        } else {
            self.days_left -= 1;
            false
        }
    }
}

impl Clone for LanternFish {
    fn clone(&self) -> Self {
        Self {
            days_left: self.days_left,
        }
    }
}

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");

    let mut fishes = contents
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .map(LanternFish::new_with_days)
        .collect::<Vec<_>>();

    for _ in 1..=80 {
        let new_fishes = fishes
            .iter_mut()
            .map(|f| f.next_day())
            .filter(|&x| x)
            .count();
        fishes.append(&mut repeat(LanternFish::new()).take(new_fishes).collect());
    }

    println!("Part 1: {}", fishes.len());
}
