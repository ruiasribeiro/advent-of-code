use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn solve_part1(path: &Path) -> String {
    let file = File::open(path).unwrap();

    BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| is_report_safe(x))
        .count()
        .to_string()
}

pub fn solve_part2(path: &Path) -> String {
    let file = File::open(path).unwrap();

    BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| is_report_safe_v2(x))
        .count()
        .to_string()
}

fn is_report_safe(line: &str) -> bool {
    let levels = line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    if levels.len() <= 1 {
        return true;
    }

    let order = levels[0].cmp(&levels[1]);
    let diff_value = (levels[0] - levels[1]).abs();

    if order == Ordering::Equal || !(1..=3).contains(&diff_value) {
        return false;
    }

    for window in levels[1..].windows(2) {
        let (prev, curr) = (window[0], window[1]);
        let diff = prev.cmp(&curr);

        if diff != order {
            return false;
        }

        let diff_value = (prev - curr).abs();

        if !(1..=3).contains(&diff_value) {
            return false;
        }
    }

    true
}

fn is_report_safe_v2(line: &str) -> bool {
    if is_report_safe(line) {
        return true;
    }

    let levels = line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    (0..levels.len()).any(|i| {
        let mut copy = levels.clone();
        copy.remove(i);

        is_report_safe(
            &copy
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" "),
        )
    })
}
