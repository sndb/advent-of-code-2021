use std::{collections::HashMap, fs::read_to_string, iter::repeat};

fn count_fish_vec(list: &[u8], days: u32) -> usize {
    let mut list: Vec<u8> = list.iter().copied().collect();

    for _ in 0..days {
        let mut new_fish = 0;

        list = list
            .into_iter()
            .map(|age| {
                age.checked_sub(1).unwrap_or_else(|| {
                    new_fish += 1;
                    6
                })
            })
            .collect();

        list.extend(repeat(8).take(new_fish))
    }

    list.len()
}

fn count_fish_map(list: &[u8], days: u32) -> usize {
    let mut day_fish: HashMap<u8, usize> = HashMap::new();

    for age in 0..=8 {
        day_fish.insert(age, list.iter().copied().filter(|&n| n == age).count());
    }

    for _ in 0..days {
        let new_fish = day_fish[&0];

        for day in 0..8 {
            day_fish.insert(day, day_fish[&(day + 1)]);
        }

        day_fish.insert(6, new_fish + day_fish[&6]);
        day_fish.insert(8, new_fish);
    }

    day_fish.values().sum()
}

fn parse_input(s: &str) -> Vec<u8> {
    s.trim()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect()
}

fn main() {
    let input = read_to_string("input/day6/input").unwrap();

    let list = parse_input(&input);

    println!("{}", count_fish_vec(&list, 80));
    println!("{}", count_fish_map(&list, 256));
}
