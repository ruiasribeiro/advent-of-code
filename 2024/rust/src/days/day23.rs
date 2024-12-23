use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn solve_part1(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let mut connection_mapping: HashMap<String, HashSet<String>> = HashMap::new();

    for line in BufReader::new(file).lines().map(Result::unwrap) {
        let (computer_1, computer_2) = line.split_once('-').unwrap();

        connection_mapping
            .entry(computer_1.to_owned())
            .or_default()
            .insert(computer_2.to_owned());

        connection_mapping
            .entry(computer_2.to_owned())
            .or_default()
            .insert(computer_1.to_owned());
    }

    let mut interconnections: HashSet<(String, String, String)> = HashSet::new();
    let mut visited: HashSet<String> = HashSet::new();

    for (computer, connections) in &connection_mapping {
        let connections = connections.iter().collect::<Vec<_>>();

        for (i, connection) in connections.iter().enumerate() {
            if visited.contains(*connection) {
                continue;
            }

            for other_connection in &connections[i + 1..] {
                if visited.contains(*other_connection) {
                    continue;
                }

                if connection_mapping
                    .get(*connection)
                    .unwrap()
                    .contains(*other_connection)
                {
                    interconnections.insert((
                        computer.to_string(),
                        (*connection).to_string(),
                        (*other_connection).to_string(),
                    ));
                }
            }
        }

        visited.insert(computer.to_string());
    }

    interconnections
        .iter()
        .filter(|(a, b, c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
        .count()
        .to_string()
}

pub fn solve_part2(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let mut connection_mapping: HashMap<String, HashSet<String>> = HashMap::new();

    for line in BufReader::new(file).lines().map(Result::unwrap) {
        let (computer_1, computer_2) = line.split_once('-').unwrap();

        connection_mapping
            .entry(computer_1.to_owned())
            .or_default()
            .insert(computer_2.to_owned());

        connection_mapping
            .entry(computer_2.to_owned())
            .or_default()
            .insert(computer_1.to_owned());
    }

    let mut interconnections: HashSet<(String, String, String)> = HashSet::new();
    let mut visited: HashSet<String> = HashSet::new();

    for (computer, connections) in &connection_mapping {
        let connections = connections.iter().collect::<Vec<_>>();

        for (i, connection) in connections.iter().enumerate() {
            if visited.contains(*connection) {
                continue;
            }

            for other_connection in &connections[i + 1..] {
                if visited.contains(*other_connection) {
                    continue;
                }

                if connection_mapping
                    .get(*connection)
                    .unwrap()
                    .contains(*other_connection)
                {
                    interconnections.insert((
                        computer.to_string(),
                        (*connection).to_string(),
                        (*other_connection).to_string(),
                    ));
                }
            }
        }

        visited.insert(computer.to_string());
    }

    interconnections
        .iter()
        .fold(Vec::<HashSet<String>>::new(), |mut acc, (a, b, c)| {
            let mut found = false;

            for set in &mut acc {
                if !set.contains(a) && connection_mapping.get(a).unwrap().is_superset(set) {
                    set.insert(a.to_string());
                    found = true;
                }

                if !set.contains(b) && connection_mapping.get(b).unwrap().is_superset(set) {
                    set.insert(b.to_string());
                    found = true;
                }

                if !set.contains(c) && connection_mapping.get(c).unwrap().is_superset(set) {
                    set.insert(c.to_string());
                    found = true;
                }
            }

            if !found {
                acc.push({
                    let mut set = HashSet::new();
                    set.insert(a.to_owned());
                    set.insert(b.to_owned());
                    set.insert(c.to_owned());
                    set
                });
            }

            acc
        })
        .iter()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .map(|set| {
            let mut computers = set.iter().map(String::as_str).collect::<Vec<_>>();
            computers.sort_unstable();
            computers.join(",")
        })
        .unwrap()
}
