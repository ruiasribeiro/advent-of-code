use std::fs;

fn part1(nums: &[i32]) {
    let mut count = 0;

    for slice in nums.windows(2) {
        if slice[1] > slice[0] {
            count += 1;
        }
    }

    println!("Part 1: {}", count);
}

fn part2(nums: &[i32]) {
    let mut count = 0;

    let sums: Vec<i32> = nums.windows(3).map(|win| win.iter().sum()).collect();

    for slice in sums.windows(2) {
        if slice[1] > slice[0] {
            count += 1;
        }
    }

    println!("Part 2: {}", count);
}

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");

    let nums = contents
        .split('\n')
        .flat_map(|s| String::from(s).trim().parse::<i32>())
        .collect::<Vec<_>>();

    part1(&nums);
    part2(&nums);
}
