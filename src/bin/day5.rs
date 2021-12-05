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
    fn x1(&self) -> u32 {
        self.a.0
    }

    fn x2(&self) -> u32 {
        self.b.0
    }

    fn y1(&self) -> u32 {
        self.a.1
    }

    fn y2(&self) -> u32 {
        self.b.1
    }

    fn min_x(&self) -> u32 {
        u32::min(self.x1(), self.x2())
    }

    fn max_x(&self) -> u32 {
        u32::max(self.x1(), self.x2())
    }

    fn min_y(&self) -> u32 {
        u32::min(self.y1(), self.y2())
    }

    fn max_y(&self) -> u32 {
        u32::max(self.y1(), self.y2())
    }

    fn is_straight(&self) -> bool {
        self.x1() == self.x2() || self.y1() == self.y2()
    }

    fn points_straight(&self) -> Vec<(u32, u32)> {
        if self.is_straight() {
            (self.min_x()..=self.max_x())
                .flat_map(|x| (self.min_y()..=self.max_y()).map(move |y| (x, y)))
                .collect()
        } else {
            Vec::new()
        }
    }

    fn points(&self) -> Vec<(u32, u32)> {
        if self.is_straight() {
            self.points_straight()
        } else {
            in_between(self.x1(), self.x2())
                .into_iter()
                .zip(in_between(self.y1(), self.y2()).into_iter())
                .collect()
        }
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

fn count_overlapping_points<F>(input: &str, line_to_points: F) -> usize
where
    F: Fn(&Line) -> Vec<(u32, u32)>,
{
    let mut overlapping_points: HashMap<(u32, u32), u32> = HashMap::new();

    input
        .lines()
        .map(|line| line.parse::<Line>().unwrap())
        .flat_map(|line| line_to_points(&line))
        .for_each(|point| *overlapping_points.entry(point).or_insert(0) += 1);

    overlapping_points
        .into_iter()
        .filter(|(_, v)| *v >= 2)
        .count()
}

fn main() {
    let input = read_to_string("input/day5/input").unwrap();

    println!(
        "{} {}",
        count_overlapping_points(&input, Line::points_straight),
        count_overlapping_points(&input, Line::points),
    );
}
