use std::{collections::HashMap, fs::read_to_string};

fn power_consumption(input: &str) -> u32 {
    let count = input.lines().count();
    let mut map = HashMap::new();

    for line in input.lines() {
        for bit in line.chars().rev().enumerate() {
            let (position, bit) = (bit.0, bit.1);
            let counter = map.entry(position).or_insert(0);
            if bit == '1' {
                *counter += 1;
            }
        }
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for (position, v) in map {
        let zeros = count - v;

        let gamma_mask = if v > zeros { 1 } else { 0 } << position;
        gamma_rate |= gamma_mask;

        let epsilon_mask = if v > zeros { 0 } else { 1 } << position;
        epsilon_rate |= epsilon_mask;
    }

    gamma_rate * epsilon_rate
}

fn read_numbers(input: &str) -> (Vec<u32>, usize) {
    let numbers: Vec<u32> = input
        .lines()
        .map(|n| u32::from_str_radix(n, 2).unwrap())
        .collect();
    let number_length = input.lines().next().unwrap().len();

    (numbers, number_length)
}

fn life_support_rating(input: &str, prefer_zeros: bool) -> u32 {
    let (mut numbers, number_length) = read_numbers(input);

    for i in 1.. {
        if numbers.len() == 1 {
            break;
        }

        let mask = 1 << (number_length - i);
        let count = numbers.iter().filter(|&n| n & mask != 0).count();

        let (nn0, nn1) = numbers.iter().partition(|&n| n & mask == 0);
        numbers = if count >= numbers.len() - count {
            if prefer_zeros {
                nn0
            } else {
                nn1
            }
        } else {
            if prefer_zeros {
                nn1
            } else {
                nn0
            }
        }
    }

    numbers[0]
}

fn oxygen_generator_rating(input: &str) -> u32 {
    life_support_rating(input, false)
}

fn co2_scrubber_rating(input: &str) -> u32 {
    life_support_rating(input, true)
}

fn main() {
    let input = read_to_string("input/day3/input.txt").unwrap();

    println!("{}", power_consumption(&input));
    println!(
        "{}",
        oxygen_generator_rating(&input) * co2_scrubber_rating(&input)
    );
}
