use std::fs::read_to_string;

const CHUNK_OPENERS: &str = "([{<";
const CHUNK_CLOSERS: &str = ")]}>";

fn matching_character(c: char) -> char {
    if CHUNK_CLOSERS.contains(c) {
        CHUNK_OPENERS
            .chars()
            .nth(CHUNK_CLOSERS.find(c).unwrap())
            .unwrap()
    } else {
        CHUNK_CLOSERS
            .chars()
            .nth(CHUNK_OPENERS.find(c).unwrap())
            .unwrap()
    }
}

fn main() {
    let input = read_to_string("input/day10/input").unwrap();
    let lines = input.lines();

    let mut incomplete_lines = Vec::new();
    let mut count = [0; 4];

    for line in lines {
        let mut context = String::new();
        let mut corrupted = false;

        for c in line.chars() {
            if !CHUNK_OPENERS.contains(c) && !context.ends_with(matching_character(c)) {
                count[CHUNK_CLOSERS.find(c).unwrap()] += 1;
                corrupted = true;
                break;
            }
            if CHUNK_OPENERS.contains(c) {
                context.push(c);
            }
            if CHUNK_CLOSERS.contains(c) && context.ends_with(matching_character(c)) {
                context.pop();
            }
        }

        if !corrupted {
            incomplete_lines.push(line);
        }
    }

    let total = count[0] * 3 + count[1] * 57 + count[2] * 1197 + count[3] * 25137;
    println!("{}", total);

    let mut scores = Vec::new();

    for line in incomplete_lines {
        let mut context = String::new();

        for c in line.chars() {
            if CHUNK_OPENERS.contains(c) {
                context.push(c);
            }
            if CHUNK_CLOSERS.contains(c) && context.ends_with(matching_character(c)) {
                context.pop();
            }
        }

        scores.push(
            context
                .chars()
                .rev()
                .map(matching_character)
                .fold(0, |score, c| {
                    (score * 5) + CHUNK_CLOSERS.find(c).unwrap() + 1
                }),
        )
    }

    scores.sort();

    let middle_score = scores[scores.len() / 2];
    println!("{}", middle_score);
}
