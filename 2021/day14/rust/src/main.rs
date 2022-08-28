use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
    usize::{MAX, MIN},
};

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");

    let (template, rules) = contents.split_once("\n\n").unwrap();

    let rules: HashMap<_, _> = rules
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(pair, element)| (pair, element.chars().nth(0).unwrap()))
        .collect();

    println!("Part 1: {}", run_process(10, template, &rules));
    // println!("Part 2: {}", run_process(40, template, &rules));
}

fn run_process(steps: usize, template: &str, rules: &HashMap<&str, char>) -> usize {
    let polymer = (0..steps).fold(template.to_owned(), |polymer, i| {
        println!("starting step {}...", i);
        advance_one_step(&polymer, &rules)
    });

    let mut occurrences: HashMap<char, usize> = HashMap::new();
    for element in polymer.chars() {
        *occurrences.entry(element).or_default() += 1;
    }

    let (max, min) = occurrences
        .values()
        .fold((MIN, MAX), |(max_val, min_val), &val| {
            (max(max_val, val), min(min_val, val))
        });

    max - min
}

fn advance_one_step(template: &str, rules: &HashMap<&str, char>) -> String {
    let mut polymer: String = template
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|str| format!("{}{}", str[0], rules[String::from_iter(str).as_str()]))
        .collect();

    polymer.push(template.chars().last().unwrap());
    polymer
}
