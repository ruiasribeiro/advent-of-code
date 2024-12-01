mod days;

fn main() {
    let results = vec![(
        days::day01::solve_part1("inputs/day01/input.txt"),
        days::day01::solve_part2("inputs/day01/input.txt"),
    )];

    for (day, (part1, part2)) in results.iter().enumerate() {
        println!("day{:02}: {} / {}", day + 1, part1, part2);
    }
}
