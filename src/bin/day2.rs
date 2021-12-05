use std::fs::read_to_string;

enum Operation {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Operation {
    fn new(s: &str) -> Operation {
        let mut tokens = s.split_whitespace();

        let direction = tokens.next().unwrap();
        let units = tokens.next().unwrap().parse().unwrap();

        match direction {
            "forward" => Operation::Forward(units),
            "down" => Operation::Down(units),
            "up" => Operation::Up(units),
            _ => panic!("unknown operation {}", s),
        }
    }
}

#[derive(Debug)]
struct Submarine {
    horizontal_position: i32,
    depth: i32,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            horizontal_position: 0,
            depth: 0,
        }
    }

    fn pilot(&mut self, op: Operation) {
        match op {
            Operation::Forward(x) => self.horizontal_position += x,
            Operation::Down(x) => self.depth += x,
            Operation::Up(x) => self.depth -= x,
        }
    }
}

#[derive(Debug)]
struct Submarine2 {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
}

impl Submarine2 {
    fn new() -> Submarine2 {
        Submarine2 {
            horizontal_position: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn pilot(&mut self, op: Operation) {
        match op {
            Operation::Forward(x) => {
                self.horizontal_position += x;
                self.depth += self.aim * x;
            }
            Operation::Down(x) => self.aim += x,
            Operation::Up(x) => self.aim -= x,
        }
    }
}

fn main() {
    let input = read_to_string("input/day2/input").unwrap();

    let mut submarine = Submarine::new();
    input
        .lines()
        .map(|l| Operation::new(&l))
        .for_each(|op| submarine.pilot(op));
    println!("{}", submarine.horizontal_position * submarine.depth);

    let mut submarine2 = Submarine2::new();
    input
        .lines()
        .map(|l| Operation::new(&l))
        .for_each(|op| submarine2.pilot(op));
    println!("{}", submarine2.horizontal_position * submarine2.depth);
}
