use std::{cmp::max, collections::HashSet, fs};

enum FoldAlong {
    X(usize),
    Y(usize),
}

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");

    // Parsing

    let (dots, folds) = contents.split_once("\n\n").unwrap();

    let mut dots: HashSet<(usize, usize)> = dots
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let folds: Vec<FoldAlong> = folds
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .last()
                .unwrap()
                .split_once('=')
                .unwrap()
        })
        .map(|(direction, position)| match direction {
            "x" => FoldAlong::X(position.parse().unwrap()),
            "y" => FoldAlong::Y(position.parse().unwrap()),
            _ => panic!(),
        })
        .collect();

    // Part 1

    println!(
        "Part 1: {}",
        fold_paper(&dots, folds.first().unwrap()).len()
    );

    // Part 2

    dots = folds
        .iter()
        .fold(dots, |folded_dots, fold| fold_paper(&folded_dots, fold));

    let (x_len, y_len) = dots.iter().fold((0, 0), |(x_max, y_max), &(x, y)| {
        (max(x_max, x), max(y_max, y))
    });

    println!("Part 2:");
    for y in 0..=y_len {
        for x in 0..=x_len {
            print!("{}", if dots.contains(&(x, y)) { '#' } else { '.' });
        }
        println!()
    }
}

fn fold_paper(dots: &HashSet<(usize, usize)>, fold: &FoldAlong) -> HashSet<(usize, usize)> {
    match fold {
        FoldAlong::X(pos) => dots
            .iter()
            .map(|(x, y)| {
                if x > pos {
                    (2 * pos - *x, *y)
                } else {
                    (*x, *y)
                }
            })
            .collect(),
        FoldAlong::Y(pos) => dots
            .iter()
            .map(|(x, y)| {
                if y > pos {
                    (*x, 2 * pos - *y)
                } else {
                    (*x, *y)
                }
            })
            .collect(),
    }
}
