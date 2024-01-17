use std::{collections::VecDeque, fs::read_to_string};

struct Data {
    points: Vec<Point>,
    folds: VecDeque<Fold>,
}

impl Data {
    fn fold(&mut self) {
        match self.folds.pop_front().unwrap() {
            Fold::X(x) => {
                self.points = self
                    .points
                    .iter()
                    .map(|point| Point {
                        x: if point.x > x {
                            x - (point.x - x)
                        } else {
                            point.x
                        },
                        y: point.y,
                    })
                    .collect();
            }
            Fold::Y(y) => {
                self.points = self
                    .points
                    .iter()
                    .map(|point| Point {
                        x: point.x,
                        y: if point.y > y {
                            y - (point.y - y)
                        } else {
                            point.y
                        },
                    })
                    .collect();
            }
        }
        self.points.sort_unstable();
        self.points.dedup();
    }

    fn draw(&self) {
        let width = self.points.iter().max_by_key(|point| point.x).unwrap().x + 1;
        let height = self.points.iter().max_by_key(|point| point.y).unwrap().y + 1;
        let mut canvas = vec![vec!['.'; width]; height];

        for point in &self.points {
            canvas[point.y][point.x] = '#';
        }
        for line in canvas {
            println!("{}", line.iter().collect::<String>());
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

enum Fold {
    X(usize),
    Y(usize),
}

fn parse_input(s: &str) -> Data {
    let mut s = s.split("\n\n").map(|split| split.lines());

    let points = s
        .next()
        .unwrap()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()))
        .map(|mut split| Point {
            x: split.next().unwrap(),
            y: split.next().unwrap(),
        })
        .collect();

    let folds = s
        .next()
        .unwrap()
        .map(|line| line.trim_start_matches("fold along ").split('='))
        .map(|mut split| {
            if split.next().unwrap() == "x" {
                Fold::X(split.next().unwrap().parse().unwrap())
            } else {
                Fold::Y(split.next().unwrap().parse().unwrap())
            }
        })
        .collect();

    Data { points, folds }
}

fn main() {
    let input = read_to_string("input/day13/input").unwrap();
    let mut data = parse_input(&input);

    data.fold();
    println!("{}", data.points.len());

    while !data.folds.is_empty() {
        data.fold();
    }
    data.draw();
}
