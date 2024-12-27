use std::fs;

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Command {
    fn new(command: &str) -> Option<Self> {
        let tokens = command.split(' ').collect::<Vec<_>>();

        if tokens.len() != 2 {
            return None;
        }

        match tokens[0] {
            "forward" => tokens[1].parse().map(Command::Forward).ok(),
            "down" => tokens[1].parse().map(Command::Down).ok(),
            "up" => tokens[1].parse().map(Command::Up).ok(),
            _ => None,
        }
    }
}

fn part1(commands: &[Command]) -> (u32, u32) {
    commands.iter().fold((0, 0), |(h, d), c| match c {
        Command::Forward(v) => (h + v, d),
        Command::Down(v) => (h, d + v),
        Command::Up(v) => (h, d - v),
    })
}

fn part2(commands: &[Command]) -> (u32, u32, u32) {
    commands.iter().fold((0, 0, 0), |(h, d, a), c| match c {
        Command::Forward(v) => (h + v, d + (a * v), a),
        Command::Down(v) => (h, d, a + v),
        Command::Up(v) => (h, d, a - v),
    })
}

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");

    let commands = contents
        .lines()
        .flat_map(|c| Command::new(c))
        .collect::<Vec<_>>();

    let (hor, dep) = part1(&commands);
    println!("Part 1: {}", hor * dep);
    let (hor, dep, _) = part2(&commands);
    println!("Part 2: {}", hor * dep);
}
