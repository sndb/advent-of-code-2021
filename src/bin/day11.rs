use std::fs::read_to_string;

enum Octopus {
    Flashing,
    Energy(u32),
}

fn increment_energy(data: &mut [Vec<Octopus>]) {
    for xs in data {
        for x in xs {
            if let Octopus::Energy(n) = x {
                *n += 1;
            }
        }
    }
}

fn values_to_process(data: &[Vec<Octopus>]) -> usize {
    let mut count = 0;

    for xs in data {
        for x in xs {
            if let Octopus::Energy(10..) = x {
                count += 1;
            }
        }
    }

    count
}

fn apply_to_surrounding_elements<F>(data: &mut [Vec<Octopus>], y: usize, x: usize, f: F)
where
    F: Fn(&mut Octopus),
{
    for i in 0..=2 {
        for j in 0..=2 {
            if (i, j) == (1, 1) {
                continue;
            }

            let i = match (y + 1).checked_sub(i) {
                Some(x) => x,
                None => continue,
            };
            let j = match (x + 1).checked_sub(j) {
                Some(x) => x,
                None => continue,
            };

            if let Some(octopus) = data.get_mut(i).and_then(|v| v.get_mut(j)) {
                f(octopus);
            }
        }
    }
}

fn simulate_flashing(data: &mut [Vec<Octopus>]) -> usize {
    let mut flashes = 0;

    while values_to_process(data) > 0 {
        for y in 0..data.len() {
            for x in 0..data[0].len() {
                if let Octopus::Energy(10..) = data[y][x] {
                    data[y][x] = Octopus::Flashing;

                    flashes += 1;

                    apply_to_surrounding_elements(data, y, x, |octopus| {
                        if let Octopus::Energy(n) = octopus {
                            *n += 1
                        }
                    });
                }
            }
        }
    }

    flashes
}

fn reset_flashing(data: &mut [Vec<Octopus>]) {
    for xs in data {
        for x in xs {
            if let Octopus::Flashing = x {
                *x = Octopus::Energy(0);
            }
        }
    }
}

fn main() {
    let input = read_to_string("input/day11/input").unwrap();
    let mut data: Vec<Vec<Octopus>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|n| Octopus::Energy(n.to_digit(10).unwrap()))
                .collect()
        })
        .collect();

    let mut total_flashes = 0;

    for i in 0.. {
        increment_energy(&mut data);
        let flashes = simulate_flashing(&mut data);
        reset_flashing(&mut data);

        total_flashes += flashes;

        if i == 100 {
            println!("{}", total_flashes);
        }

        if flashes == data.len() * data[0].len() {
            println!("{}", i + 1);
            break;
        }
    }
}
