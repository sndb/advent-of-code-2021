use std::fs::read_to_string;

fn read_input(s: &str) -> Vec<u32> {
    s.lines().map(|s| s.parse().unwrap()).collect()
}

fn first_half(input: &[u32]) -> usize {
    input.windows(2).filter(|p| p[1] > p[0]).count()
}

fn second_half(input: &[u32]) -> usize {
    input
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|p| p[1] > p[0])
        .count()
}

fn main() {
    let input = read_to_string("input/day1/input.txt").unwrap();
    let numbers = read_input(&input);

    println!("{}", first_half(&numbers));
    println!("{}", second_half(&numbers));
}
