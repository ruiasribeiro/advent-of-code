use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    vec,
};

type Pages = Vec<Vec<u32>>;

pub fn solve_part1(path: &Path) -> String {
    let (valid_pages, _invalid_pages, _precedence_mapping) = process_input(path);

    valid_pages
        .iter()
        .map(|page| page.get(page.len() / 2))
        .map(Option::unwrap)
        .sum::<u32>()
        .to_string()
}

pub fn solve_part2(path: &Path) -> String {
    let (_valid_pages, invalid_pages, precedence_mapping) = process_input(path);

    invalid_pages
        .iter()
        .map(|page| {
            let mut frozen_page;
            let mut modified_page = page.clone();

            'outer: loop {
                frozen_page = modified_page.clone();

                for (i, current) in frozen_page.iter().enumerate() {
                    for (j, must_be_before) in frozen_page.iter().enumerate().skip(i + 1) {
                        if let Some(precedences) = precedence_mapping.get(current) {
                            if precedences.contains(must_be_before) {
                                modified_page.remove(j);
                                modified_page.insert(i, *must_be_before);

                                // The idea here is to place incorrectly-ordered
                                // numbers right before the ones that they
                                // should be behind. Doing this multiple times
                                // will (hopefull) make every number fall
                                // correctly into place.

                                continue 'outer;
                            }
                        }
                    }
                }

                break;
            }

            let middle = frozen_page.len() / 2;
            *frozen_page.get(middle).unwrap()
        })
        .sum::<u32>()
        .to_string()
}

fn process_input(path: &Path) -> (Pages, Pages, HashMap<u32, HashSet<u32>>) {
    let file = File::open(path).unwrap();

    let lines = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let (rules, pages) = lines.split_at(lines.iter().position(String::is_empty).unwrap());

    // { number => [precedents] }
    let mut precedence_mapping: HashMap<u32, HashSet<u32>> = HashMap::new();

    for rule in rules {
        let (precedent, number) = rule.split_once('|').unwrap();

        let precedent = precedent.parse::<u32>().unwrap();
        let number = number.parse::<u32>().unwrap();

        if let Some(value) = precedence_mapping.get_mut(&number) {
            value.insert(precedent);
        } else {
            let mut precedents = HashSet::new();
            precedents.insert(precedent);

            precedence_mapping.insert(number, precedents);
        }
    }

    let mut valid_pages: Vec<Vec<u32>> = vec![];
    let mut invalid_pages: Vec<Vec<u32>> = vec![];

    for page in pages.iter().skip(1) {
        let page = page
            .split(',')
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect::<Vec<_>>();

        if is_page_valid(&page, &precedence_mapping) {
            valid_pages.push(page);
        } else {
            invalid_pages.push(page);
        }
    }

    (valid_pages, invalid_pages, precedence_mapping)
}

fn is_page_valid(page: &[u32], precedence_mapping: &HashMap<u32, HashSet<u32>>) -> bool {
    page.iter().enumerate().all(|(i, current)| {
        (i + 1..page.len()).all(|j| {
            let precedences = precedence_mapping.get(current);

            match precedences {
                None => true,
                Some(precedences) => !precedences.contains(&page[j]),
            }
        })
    })
}
