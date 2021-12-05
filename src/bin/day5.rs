use std::{collections::HashMap, fs::read_to_string, num::ParseIntError, str::FromStr};

fn in_between(n1: u32, n2: u32) -> Vec<u32> {
    if n1 < n2 {
        (n1..=n2).collect()
    } else {
        (n2..=n1).rev().collect()
    }
}

struct Line {
    a: (u32, u32),
    b: (u32, u32),
}

impl Line {
    fn is_straight(&self) -> bool {
        self.a.0 == self.b.0 || self.a.1 == self.b.1
    }

    fn points_straight(&self) -> Vec<(u32, u32)> {
        let mut points = Vec::new();

        if !self.is_straight() {
            return points;
        }

        for x in self.a.0.min(self.b.0)..=self.a.0.max(self.b.0) {
            for y in self.a.1.min(self.b.1)..=self.a.1.max(self.b.1) {
                points.push((x, y));
            }
        }

        points
    }

    fn points(&self) -> Vec<(u32, u32)> {
        let mut points = Vec::new();

        if self.is_straight() {
            for x in self.a.0.min(self.b.0)..=self.a.0.max(self.b.0) {
                for y in self.a.1.min(self.b.1)..=self.a.1.max(self.b.1) {
                    points.push((x, y));
                }
            }
        } else {
            points = in_between(self.a.0, self.b.0)
                .into_iter()
                .zip(in_between(self.a.1, self.b.1).into_iter())
                .collect()
        }

        points
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split("->");

        let a = s.next().unwrap().trim().split_once(",").unwrap();
        let a: (u32, u32) = (a.0.parse()?, a.1.parse()?);

        let b = s.next().unwrap().trim().split_once(",").unwrap();
        let b: (u32, u32) = (b.0.parse()?, b.1.parse()?);

        Ok(Line { a, b })
    }
}

fn count_overlapping_points(
    input: &str,
    points_generator: impl Fn(&Line) -> Vec<(u32, u32)>,
) -> usize {
    let mut overlapping_points: HashMap<(u32, u32), u32> = HashMap::new();

    for line in input.lines().map(|line| line.parse::<Line>().unwrap()) {
        for p in points_generator(&line) {
            *overlapping_points.entry(p).or_insert(0) += 1;
        }
    }

    overlapping_points
        .into_iter()
        .filter(|(_, v)| *v >= 2)
        .count()
}

fn main() {
    let input = read_to_string("input/day5/input.txt").unwrap();

    println!(
        "{} {}",
        count_overlapping_points(&input, Line::points_straight),
        count_overlapping_points(&input, Line::points),
    );
}
