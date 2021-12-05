use std::fs::read_to_string;

fn power_consumption(input: &str) -> u32 {
    let input_len = input.lines().count();
    let number_len = input.lines().next().unwrap().chars().count();
    let mut map = vec![0; number_len];

    input.lines().for_each(|number| {
        number
            .chars()
            .rev()
            .enumerate()
            .for_each(|(position, bit)| {
                if bit == '1' {
                    map[position] += 1
                }
            })
    });

    let mut gamma_rate = 0;
    map.into_iter().enumerate().for_each(|(position, count)| {
        gamma_rate |= if count > input_len - count { 1 } else { 0 } << position
    });

    let epsilon_rate = gamma_rate ^ ((1 << number_len) - 1);

    gamma_rate * epsilon_rate
}

fn read_numbers(input: &str) -> (Vec<u32>, usize) {
    let numbers = input
        .lines()
        .map(|n| u32::from_str_radix(n, 2).unwrap())
        .collect();
    let number_length = input.lines().next().unwrap().len();

    (numbers, number_length)
}

fn life_support_rating(input: &str, part: LifeSupportRatingPart) -> u32 {
    let (mut numbers, number_length) = read_numbers(input);

    for i in 1.. {
        if numbers.len() == 1 {
            break;
        }

        let mask = 1 << (number_length - i);
        let ones = numbers.iter().filter(|&n| n & mask > 0).count();
        let zeros = numbers.len() - ones;

        numbers.retain(|&n| match part {
            LifeSupportRatingPart::OxygenGeneratorRating => {
                if ones >= zeros {
                    n & mask > 0
                } else {
                    n & mask == 0
                }
            }
            LifeSupportRatingPart::Co2ScrubberRating => {
                if ones >= zeros {
                    n & mask == 0
                } else {
                    n & mask > 0
                }
            }
        });
    }

    numbers[0]
}

enum LifeSupportRatingPart {
    OxygenGeneratorRating,
    Co2ScrubberRating,
}

fn oxygen_generator_rating(input: &str) -> u32 {
    life_support_rating(input, LifeSupportRatingPart::OxygenGeneratorRating)
}

fn co2_scrubber_rating(input: &str) -> u32 {
    life_support_rating(input, LifeSupportRatingPart::Co2ScrubberRating)
}

fn main() {
    let input = read_to_string("input/day3/input").unwrap();

    println!("{}", power_consumption(&input));
    println!(
        "{}",
        oxygen_generator_rating(&input) * co2_scrubber_rating(&input)
    );
}
