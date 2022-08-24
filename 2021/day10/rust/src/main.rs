use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");

    let mut illegal_chars = vec![];

    for line in contents.lines() {
        let mut stack = vec![];

        for char in line.chars() {
            match char {
                '(' | '[' | '{' | '<' => stack.push(char),
                ')' | ']' | '}' | '>' => {
                    if let Some(opening) = stack.pop() {
                        if !is_a_matching_pair(opening, char) {
                            illegal_chars.push(char);
                            break;
                        }
                    }
                }
                _ => panic!("Unexpected char in file: {}", char),
            }
        }
    }

    let score: usize = illegal_chars
        .iter()
        .map(|char| match char {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Unexpected illegal char: {}", char),
        })
        .sum();

    println!("Part 1: {}", score);

    let mut scores = vec![];

    'outer: for line in contents.lines() {
        let mut stack = vec![];

        for char in line.chars() {
            match char {
                '(' | '[' | '{' | '<' => stack.push(char),
                ')' | ']' | '}' | '>' => {
                    if let Some(opening) = stack.pop() {
                        if !is_a_matching_pair(opening, char) {
                            continue 'outer;
                        }
                    }
                }
                _ => panic!("Unexpected char in file: {}", char),
            }
        }

        if !stack.is_empty() {
            let mut score = 0;

            while let Some(char) = stack.pop() {
                match char {
                    '(' | '[' | '{' | '<' => {
                        score = score * 5 + autocomplete_score(char);
                    }
                    _ => panic!("Unexpected char in stack: {}", char),
                }
            }

            scores.push(score);
        }
    }

    scores.sort();
    let middle = scores[scores.len() / 2];

    println!("Part 2: {:?}", middle);
}

fn is_a_matching_pair(opening: char, closing: char) -> bool {
    (opening == '(' && closing == ')')
        || (opening == '[' && closing == ']')
        || (opening == '{' && closing == '}')
        || (opening == '<' && closing == '>')
}

fn autocomplete_score(char: char) -> usize {
    match char {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("Unexpected char at score conversion: {}", char),
    }
}
