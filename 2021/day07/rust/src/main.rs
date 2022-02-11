use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");

    let positions = contents
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect::<Vec<isize>>();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let fuel = (min..=max)
        .map(|x| positions.iter().map(|p| isize::abs(p - x)).sum::<isize>())
        .min()
        .unwrap();

    println!("Part 1: {}", fuel);

    let fuel = (min..=max)
        .map(|x| {
            positions
                .iter()
                .map(|p| (1..=isize::abs(p - x)).sum::<isize>())
                .sum::<isize>()
        })
        .min()
        .unwrap();

    println!("Part 2: {}", fuel);
}
