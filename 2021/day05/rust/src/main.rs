use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Floor {
    floor: HashMap<u32, HashMap<u32, u32>>,
}

impl Floor {
    fn new() -> Self {
        Floor {
            floor: HashMap::new(),
        }
    }

    fn cover_pos(&mut self, pos: (u32, u32)) {
        if let None = self.floor.get(&pos.0) {
            self.floor.insert(pos.0, HashMap::new());
        }

        *self
            .floor
            .get_mut(&pos.0)
            .unwrap()
            .entry(pos.1)
            .or_insert(0) += 1;
    }

    fn cover_line(&mut self, fst: (u32, u32), lst: (u32, u32)) {
        if fst.0 == lst.0 {
            let min = fst.1.min(lst.1);
            let max = fst.1.max(lst.1);
            for y in min..=max {
                self.cover_pos((fst.0, y));
            }
        } else if fst.1 == lst.1 {
            let min = fst.0.min(lst.0);
            let max = fst.0.max(lst.0);
            for x in min..=max {
                self.cover_pos((x, fst.1));
            }
        } else {
            // comment this entire branch for part 1 result
            if fst.0 < lst.0 && fst.1 < lst.1 {
                (fst.0..=lst.0)
                    .zip(fst.1..=lst.1)
                    .for_each(|(x, y)| self.cover_pos((x, y)));
            } else if fst.0 > lst.0 && fst.1 < lst.1 {
                ((lst.0..=fst.0).rev())
                    .zip(fst.1..=lst.1)
                    .for_each(|(x, y)| self.cover_pos((x, y)));
            } else if fst.0 < lst.0 && fst.1 > lst.1 {
                (fst.0..=lst.0)
                    .zip((lst.1..=fst.1).rev())
                    .for_each(|(x, y)| self.cover_pos((x, y)));
            } else {
                ((lst.0..=fst.0).rev())
                    .zip((lst.1..=fst.1).rev())
                    .for_each(|(x, y)| self.cover_pos((x, y)));
            }
        }
    }

    fn num_overlaps(&self) -> usize {
        self.floor
            .iter()
            .map(|(_, line)| line.iter().filter(|(_, &count)| count > 1).count())
            .sum()
    }
}

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");

    let mut x = Floor::new();

    let re_vents = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    contents.lines().for_each(|l| {
        let caps = re_vents.captures(l).unwrap();
        let x1 = caps.get(1).unwrap().as_str().parse().unwrap();
        let y1 = caps.get(2).unwrap().as_str().parse().unwrap();
        let x2 = caps.get(3).unwrap().as_str().parse().unwrap();
        let y2 = caps.get(4).unwrap().as_str().parse().unwrap();
        x.cover_line((x1, y1), (x2, y2));
    });

    println!("Result: {}", x.num_overlaps());
}
