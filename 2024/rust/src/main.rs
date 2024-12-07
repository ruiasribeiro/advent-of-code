mod days;

type SolveDay = Box<dyn Fn() -> String>;

fn main() {
    let results: Vec<(SolveDay, SolveDay)> = vec![
        (
            Box::new(|| days::day01::solve_part1("inputs/day01/input.txt")),
            Box::new(|| days::day01::solve_part2("inputs/day01/input.txt")),
        ),
        (
            Box::new(|| days::day02::solve_part1("inputs/day02/input.txt")),
            Box::new(|| days::day02::solve_part2("inputs/day02/input.txt")),
        ),
        (
            Box::new(|| days::day03::solve_part1("inputs/day03/input.txt")),
            Box::new(|| days::day03::solve_part2("inputs/day03/input.txt")),
        ),
        (
            Box::new(|| days::day04::solve_part1("inputs/day04/input.txt")),
            Box::new(|| days::day04::solve_part2("inputs/day04/input.txt")),
        ),
        (
            Box::new(|| days::day05::solve_part1("inputs/day05/input.txt")),
            Box::new(|| days::day05::solve_part2("inputs/day05/input.txt")),
        ),
        (
            Box::new(|| String::from("skipped")), //days::day06::solve_part1("inputs/day06/input.txt")),
            Box::new(|| String::from("skipped")), //days::day06::solve_part2("inputs/day06/input.txt")),
        ),
    ];

    for (day, (part1, part2)) in results.iter().enumerate() {
        print!("day{:02}: ", day + 1);
        print!("{}", part1());
        print!(" / ");
        print!("{}", part2());
        println!()
    }
}
