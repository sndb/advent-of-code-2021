use std::{collections::HashMap, fs::read_to_string};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Pair(char, char);

struct Data {
    numbers_of_pairs: HashMap<Pair, usize>,
    numbers_of_letters: HashMap<char, usize>,
    rules: HashMap<Pair, char>,
}

impl Data {
    fn answer(&self) -> usize {
        self.numbers_of_letters.values().max().unwrap()
            - self.numbers_of_letters.values().min().unwrap()
    }
}

fn read_input(s: &str) -> Data {
    let mut s = s.split("\n\n");

    let template: Vec<char> = s.next().unwrap().chars().collect();

    let mut numbers_of_letters = HashMap::new();
    for letter in &template {
        *numbers_of_letters.entry(*letter).or_insert(0) += 1;
    }

    let mut numbers_of_pairs = HashMap::new();
    for pair in template.windows(2) {
        *numbers_of_pairs.entry(Pair(pair[0], pair[1])).or_insert(0) += 1;
    }

    let rules = s
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(" -> ").map(|s| s.chars()))
        .map(|mut split| {
            let mut adjacent = split.next().unwrap();
            let mut between = split.next().unwrap();

            (
                Pair(adjacent.next().unwrap(), adjacent.next().unwrap()),
                between.next().unwrap(),
            )
        })
        .collect();

    Data {
        numbers_of_pairs,
        numbers_of_letters,
        rules,
    }
}

fn main() {
    let input = read_to_string("input/day14/input").unwrap();
    let mut data = read_input(&input);

    for i in 0..40 {
        for (pair, n) in data.numbers_of_pairs.clone() {
            if n > 0 {
                let e = *data.rules.get(&pair).unwrap();

                *data.numbers_of_pairs.get_mut(&pair).unwrap() -= n;
                *data.numbers_of_pairs.entry(Pair(pair.0, e)).or_insert(0) += n;
                *data.numbers_of_pairs.entry(Pair(e, pair.1)).or_insert(0) += n;
                *data.numbers_of_letters.entry(e).or_insert(0) += n;
            }
        }

        if i == 9 {
            println!("{:?}", data.answer());
        }
    }

    println!("{:?}", data.answer());
}
