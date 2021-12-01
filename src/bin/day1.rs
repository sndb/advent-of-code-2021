use std::error::Error;
use std::fs::read_to_string;
use std::io;

fn read_input(path: &str) -> io::Result<Vec<i32>> {
    let numbers = read_to_string(path)?;
    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    Ok(numbers)
}

fn first_half(input: &[i32]) -> i32 {
    let mut n = 0;
    input.windows(2).for_each(|p| {
        if p[1] > p[0] {
            n += 1
        }
    });
    n
}

fn second_half(input: &[i32]) -> i32 {
    let mut n = 0;
    input
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .for_each(|p| {
            if p[1] > p[0] {
                n += 1
            }
        });
    n
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input("day1_input.txt")?;
    println!("{}", first_half(&input));
    println!("{}", second_half(&input));
    Ok(())
}
