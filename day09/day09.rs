use std::fs::read_to_string;

fn traverse_basin(height_map: &[Vec<u32>], starting_point: (usize, usize)) -> Vec<(usize, usize)> {
    if height_map[starting_point.0][starting_point.1] == 9 {
        return Vec::new();
    }

    let y_len = height_map.len();
    let x_len = height_map[0].len();

    let mut known_points: Vec<(usize, usize)> = Vec::new();
    let mut to_traverse: Vec<(usize, usize)> = vec![starting_point];

    while !to_traverse.is_empty() {
        let initial_position = to_traverse[0];
        to_traverse.remove(0);

        if known_points.contains(&initial_position) {
            continue;
        }

        let mut cursor = initial_position;
        known_points.push(cursor);
        if cursor.0 < y_len - 1 && height_map[cursor.0 + 1][cursor.1] < 9 {
            to_traverse.push((cursor.0 + 1, cursor.1));
        }
        if cursor.0 > 0 && height_map[cursor.0 - 1][cursor.1] < 9 {
            to_traverse.push((cursor.0 - 1, cursor.1));
        }

        while cursor.1 > 0 && height_map[cursor.0][cursor.1 - 1] < 9 {
            cursor = (cursor.0, cursor.1 - 1);
            known_points.push(cursor);

            if cursor.0 < y_len - 1 && height_map[cursor.0 + 1][cursor.1] < 9 {
                to_traverse.push((cursor.0 + 1, cursor.1));
            }
            if cursor.0 > 0 && height_map[cursor.0 - 1][cursor.1] < 9 {
                to_traverse.push((cursor.0 - 1, cursor.1));
            }
        }

        cursor = initial_position;
        while cursor.1 < x_len - 1 && height_map[cursor.0][cursor.1 + 1] < 9 {
            cursor = (cursor.0, cursor.1 + 1);
            known_points.push(cursor);

            if cursor.0 < y_len - 1 && height_map[cursor.0 + 1][cursor.1] < 9 {
                to_traverse.push((cursor.0 + 1, cursor.1));
            }
            if cursor.0 > 0 && height_map[cursor.0 - 1][cursor.1] < 9 {
                to_traverse.push((cursor.0 - 1, cursor.1));
            }
        }
    }

    known_points
}

fn main() {
    let input = read_to_string("input/day9/input").unwrap();
    let data: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let y_len = data.len();
    let x_len = data[0].len();

    let mut result = 0;

    for y in 0..y_len {
        for x in 0..x_len {
            let mut numbers: Vec<u32> = Vec::new();
            if y > 0 {
                numbers.push(data[y - 1][x]);
            }
            if y < y_len - 1 {
                numbers.push(data[y + 1][x]);
            }
            if x > 0 {
                numbers.push(data[y][x - 1]);
            }
            if x < x_len - 1 {
                numbers.push(data[y][x + 1])
            }
            if numbers.into_iter().all(|number| data[y][x] < number) {
                result += data[y][x] + 1;
            }
        }
    }

    println!("{}", result);

    let mut basin_sizes: Vec<usize> = Vec::new();
    let mut known_points: Vec<(usize, usize)> = Vec::new();
    for y in 0..y_len {
        for x in 0..x_len {
            if !known_points.contains(&(y, x)) {
                let basin_points = traverse_basin(&data, (y, x));
                known_points.extend(&basin_points);
                basin_sizes.push(basin_points.len());
            }
        }
    }
    basin_sizes.sort_unstable();
    println!(
        "{}",
        basin_sizes.into_iter().rev().take(3).product::<usize>()
    );
}
