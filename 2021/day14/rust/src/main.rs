use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
    usize::{MAX, MIN},
};

// {pair of elements -> number of occurrences}
type Polymer = HashMap<String, usize>;

// {pair of elements -> resulting pairs after a step}
type Rules = HashMap<String, (String, String)>;

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");

    let (template, rules) = contents.split_once("\n\n").unwrap();

    let rules: Rules = rules
        .lines()
        .map(|line| {
            let (pair, element) = line.split_once(" -> ").unwrap();
            let element = element.chars().nth(0).unwrap();

            (
                String::from(pair),
                (
                    String::from_iter([pair.chars().nth(0).unwrap(), element]),
                    String::from_iter([element, pair.chars().nth(1).unwrap()]),
                ),
            )
        })
        .collect();

    println!("Part 1: {}", run(10, template, &rules));
    println!("Part 2: {}", run(40, template, &rules));
}

fn run(steps: usize, template: &str, rules: &Rules) -> usize {
    let template = template.chars().collect::<Vec<_>>();

    let mut polymer = Polymer::new();
    for pair in template.windows(2).map(String::from_iter) {
        *polymer.entry(pair).or_default() += 1;
    }

    let polymer = (0..steps).fold(polymer, |polymer, _| advance_one_step(&polymer, rules));

    calculate_difference(&template, &polymer)
}

fn advance_one_step(polymer: &Polymer, rules: &Rules) -> Polymer {
    let mut new_polymer = Polymer::new();

    for (pair, count) in polymer.iter() {
        let (left_result, right_result) = &rules[pair];

        *new_polymer.entry(left_result.clone()).or_default() += count;
        *new_polymer.entry(right_result.clone()).or_default() += count;
    }

    new_polymer
}

fn calculate_difference(template: &[char], polymer: &Polymer) -> usize {
    let mut occurrences: HashMap<char, usize> = HashMap::new();

    // Since the polymer contains the occurrence of pairs of elements, each
    // element of the pair will be counted twice, except for the first and last.
    for (pair, count) in polymer.iter() {
        for char in pair.chars() {
            *occurrences.entry(char).or_default() += count;
        }
    }

    // In order to get the correct count, the first and last elements of the
    // polymer (which are the same as the ones from the template) must be
    // removed before halving all the other ones.
    *occurrences.get_mut(template.first().unwrap()).unwrap() -= 1;
    *occurrences.get_mut(template.last().unwrap()).unwrap() -= 1;

    let mut occurrences: HashMap<char, usize> = occurrences
        .iter()
        .map(|(char, count)| (char.to_owned(), count / 2))
        .collect();

    // After halving, the first and last elements can be added again.
    *occurrences.get_mut(template.first().unwrap()).unwrap() += 1;
    *occurrences.get_mut(template.last().unwrap()).unwrap() += 1;

    let (max, min) = occurrences
        .values()
        .fold((MIN, MAX), |(max_val, min_val), &val| {
            (max(max_val, val), min(min_val, val))
        });

    max - min
}
