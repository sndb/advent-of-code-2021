use std::{collections::HashSet, fs::read_to_string};

fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .map(|line| {
            line.splitn(2, '|')
                .map(|split| split.trim().split_whitespace().map(|s| s.to_string()))
        })
        .map(|mut splits| {
            (
                splits.next().unwrap().collect(),
                splits.next().unwrap().collect(),
            )
        })
        .collect()
}

fn is_within(number: &str, pattern: &str) -> bool {
    number.chars().all(|c| pattern.contains(c))
}

fn main() {
    let input = read_to_string("input/day8/input").unwrap();
    let data = parse_input(&input);

    let mut count = 0;
    for (_, output_value) in &data {
        for digit in output_value {
            if let 2 | 3 | 4 | 7 = digit.len() {
                count += 1;
            }
        }
    }

    println!("{}", count);

    let mut total = 0;
    for (mut signal_pattern, output_value) in data {
        signal_pattern.sort_by_key(|p| p.len());

        let (one, seven, four, eight) = (
            &signal_pattern[0],
            &signal_pattern[1],
            &signal_pattern[2],
            &signal_pattern[9],
        );
        let (mut zero, mut two, mut three, mut five, mut six, mut nine): (
            &str,
            &str,
            &str,
            &str,
            &str,
            &str,
        ) = ("", "", "", "", "", "");

        for pattern in &signal_pattern {
            if pattern.len() == 5 {
                if is_within(one, pattern) {
                    three = pattern;
                } else if pattern
                    .chars()
                    .collect::<HashSet<char>>()
                    .intersection(&four.chars().collect::<HashSet<char>>())
                    .count()
                    == 3
                {
                    five = pattern;
                } else if pattern
                    .chars()
                    .collect::<HashSet<char>>()
                    .intersection(&four.chars().collect::<HashSet<char>>())
                    .count()
                    == 2
                {
                    two = pattern;
                }
            }
        }

        for pattern in &signal_pattern {
            if pattern.len() == 6 {
                if is_within(three, pattern) {
                    nine = pattern;
                } else if is_within(seven, pattern) && !is_within(three, pattern) {
                    zero = pattern;
                } else if !is_within(seven, pattern) && !is_within(three, pattern) {
                    six = pattern;
                }
            }
        }

        let mut m = 1;
        let mut result = 0;

        for digit in output_value.iter().rev() {
            for (number, pattern) in [zero, one, two, three, four, five, six, seven, eight, nine]
                .iter()
                .enumerate()
            {
                if digit.chars().collect::<HashSet<char>>()
                    == pattern.chars().collect::<HashSet<char>>()
                {
                    result += number * m;
                    m *= 10;
                }
            }
        }

        total += result;
    }

    println!("{}", total);
}
