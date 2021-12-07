use std::fs::read_to_string;

fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

fn triangle_number(x: i32) -> i32 {
    x * (x + 1) / 2
}

fn main() {
    let input = read_to_string("input/day7/input").unwrap();

    let numbers = parse_numbers(&input);

    let min = numbers.iter().copied().min().unwrap();
    let max = numbers.iter().copied().max().unwrap();

    let part1: i32 = (min..=max)
        .map(|position| {
            numbers
                .iter()
                .copied()
                .map(|n| (n as i32 - position as i32).abs())
                .sum()
        })
        .min()
        .unwrap();

    let part2: i32 = (min..=max)
        .map(|position| {
            numbers
                .iter()
                .copied()
                .map(|n| triangle_number((n as i32 - position as i32).abs()))
                .sum()
        })
        .min()
        .unwrap();

    println!("{} {}", part1, part2);
}
