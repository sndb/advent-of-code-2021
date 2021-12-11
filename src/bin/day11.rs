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
            if let Octopus::Energy(n) = *x {
                if n > 9 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn simulate_flashing(data: &mut [Vec<Octopus>]) -> usize {
    let mut flashes = 0;

    while values_to_process(data) > 0 {
        for y in 0..data.len() {
            for x in 0..data[0].len() {
                if let Octopus::Energy(10..) = data[y][x] {
                    data[y][x] = Octopus::Flashing;
                    flashes += 1;

                    if y > 0 && x > 0 {
                        if let Some(Octopus::Energy(n)) =
                            data.get_mut(y - 1).and_then(|v| v.get_mut(x - 1))
                        {
                            *n += 1;
                        }
                    }

                    if y > 0 {
                        if let Some(Octopus::Energy(n)) =
                            data.get_mut(y - 1).and_then(|v| v.get_mut(x))
                        {
                            *n += 1;
                        }

                        if let Some(Octopus::Energy(n)) =
                            data.get_mut(y - 1).and_then(|v| v.get_mut(x + 1))
                        {
                            *n += 1;
                        }
                    }

                    if x > 0 {
                        if let Some(Octopus::Energy(n)) =
                            data.get_mut(y).and_then(|v| v.get_mut(x - 1))
                        {
                            *n += 1;
                        }

                        if let Some(Octopus::Energy(n)) =
                            data.get_mut(y + 1).and_then(|v| v.get_mut(x - 1))
                        {
                            *n += 1;
                        }
                    }

                    if let Some(Octopus::Energy(n)) = data.get_mut(y).and_then(|v| v.get_mut(x + 1))
                    {
                        *n += 1;
                    }

                    if let Some(Octopus::Energy(n)) = data.get_mut(y + 1).and_then(|v| v.get_mut(x))
                    {
                        *n += 1;
                    }

                    if let Some(Octopus::Energy(n)) =
                        data.get_mut(y + 1).and_then(|v| v.get_mut(x + 1))
                    {
                        *n += 1;
                    }
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
