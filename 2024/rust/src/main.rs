mod days;

fn main() {
    let results = vec![
        (
            days::day01::solve_part1("inputs/day01/input.txt"),
            days::day01::solve_part2("inputs/day01/input.txt"),
        ),
        (
            days::day02::solve_part1("inputs/day02/input.txt"),
            days::day02::solve_part2("inputs/day02/input.txt"),
        ),
        (
            days::day03::solve_part1("inputs/day03/input.txt"),
            days::day03::solve_part2("inputs/day03/input.txt"),
        ),
        (
            days::day04::solve_part1("inputs/day04/input.txt"),
            days::day04::solve_part2("inputs/day04/input.txt"),
        ),
        (
            days::day05::solve_part1("inputs/day05/input.txt"),
            days::day05::solve_part2("inputs/day05/input.txt"),
        ),
        (
            days::day06::solve_part1("inputs/day06/input.txt"),
            days::day06::solve_part2("inputs/day06/input.txt"),
        ),
    ];

    for (day, (part1, part2)) in results.iter().enumerate() {
        println!("day{:02}: {} / {}", day + 1, part1, part2);
    }
}
