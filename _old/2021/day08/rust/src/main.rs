use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");

    let count: usize = contents
        .lines()
        .map(|line| {
            line.split('|')
                // get patterns after '|'
                .nth(1)
                .unwrap()
                // get list of those patterns
                .split_ascii_whitespace()
                // get only patterns that match the wanted length
                .filter(|word| [2, 3, 4, 7].contains(&word.len()))
                .count()
        })
        .sum();

    println!("Part 1: {}", count);
}
