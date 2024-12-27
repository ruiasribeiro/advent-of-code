use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use good_lp::Solution;
use good_lp::{self, SolverModel};

struct Button {
    x: i32,
    y: i32,
}

struct Prize {
    x: i64,
    y: i64,
}

struct Machine {
    a_button: Button,
    b_button: Button,
    prize: Prize,
}

fn parse_button(line: &str) -> Button {
    let (x, y) = line.split_once(": ").unwrap().1.split_once(", ").unwrap();

    let x = x[2..].parse::<i32>().unwrap();
    let y = y[2..].parse::<i32>().unwrap();

    Button { x, y }
}

fn parse_prize(line: &str) -> Prize {
    let (x, y) = line.split_once(": ").unwrap().1.split_once(", ").unwrap();

    let x = x[2..].parse::<i64>().unwrap();
    let y = y[2..].parse::<i64>().unwrap();

    Prize { x, y }
}

fn x_movement(
    a: good_lp::Variable,
    b: good_lp::Variable,
    machine: &Machine,
) -> good_lp::Expression {
    a * machine.a_button.x + b * machine.b_button.x
}

fn y_movement(
    a: good_lp::Variable,
    b: good_lp::Variable,
    machine: &Machine,
) -> good_lp::Expression {
    a * machine.a_button.y + b * machine.b_button.y
}

pub fn solve_part1(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let lines = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let machines = lines
        .split(String::is_empty)
        .map(|chunk| {
            let a_button = parse_button(&chunk[0]);
            let b_button = parse_button(&chunk[1]);
            let prize = parse_prize(&chunk[2]);

            Machine {
                a_button,
                b_button,
                prize,
            }
        })
        .collect::<Vec<_>>();

    let mut tokens_spent = 0_i64;

    for machine in &machines {
        good_lp::variables! {
            vars:
                0 <= a <= 100;
                0 <= b <= 100;
        }

        #[allow(clippy::cast_precision_loss)]
        let result = vars
            .minimise(3 * a + b)
            .using(good_lp::default_solver)
            .with(x_movement(a, b, machine).eq(machine.prize.x as f64))
            .with(y_movement(a, b, machine).eq(machine.prize.y as f64))
            .solve();

        if let Ok(solution) = result {
            let a_val = solution.value(a);
            let b_val = solution.value(b);

            if (a_val - a_val.round()).abs() < 1e-8 && (b_val - b_val.round()).abs() < 1e-8 {
                #[allow(clippy::cast_possible_truncation)]
                let cost = solution.eval(3 * a + b).round() as i64;

                tokens_spent = tokens_spent.checked_add(cost).unwrap();
            }
        }
    }

    tokens_spent.to_string()
}

fn parse_prize_v2(line: &str) -> Prize {
    let (x, y) = line.split_once(": ").unwrap().1.split_once(", ").unwrap();

    let x = x[2..].parse::<i64>().unwrap() + 10_000_000_000_000;
    let y = y[2..].parse::<i64>().unwrap() + 10_000_000_000_000;

    Prize { x, y }
}

pub fn solve_part2(path: &Path) -> String {
    let file = File::open(path).unwrap();

    let lines = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let machines = lines
        .split(String::is_empty)
        .map(|chunk| {
            let a_button = parse_button(&chunk[0]);
            let b_button = parse_button(&chunk[1]);
            let prize = parse_prize_v2(&chunk[2]);

            Machine {
                a_button,
                b_button,
                prize,
            }
        })
        .collect::<Vec<_>>();

    let mut tokens_spent = 0_i64;

    for machine in &machines {
        good_lp::variables! {
            vars:
                0 <= a;
                0 <= b;
        }

        #[allow(clippy::cast_precision_loss)]
        let result = vars
            .minimise(3 * a + b)
            .using(good_lp::default_solver)
            .with(x_movement(a, b, machine).eq(machine.prize.x as f64))
            .with(y_movement(a, b, machine).eq(machine.prize.y as f64))
            .solve();

        if let Ok(solution) = result {
            let a_val = solution.value(a);
            let b_val = solution.value(b);

            if (a_val - a_val.round()).abs() < 1e-2 && (b_val - b_val.round()).abs() < 1e-2 {
                #[allow(clippy::cast_possible_truncation)]
                let cost = solution.eval(3 * a + b).round() as i64;

                tokens_spent = tokens_spent.checked_add(cost).unwrap();
            }
        }
    }

    tokens_spent.to_string()
}
