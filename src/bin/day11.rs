use std::fs::read_to_string;

enum Octopus {
    Flashing,
    Energy(u32),
}

impl Octopus {
    fn new(n: u32) -> Octopus {
        Octopus::Energy(n)
    }
}

fn increment_energy(data: &mut Vec<Vec<Octopus>>) {
    for xs in data {
        for x in xs {
            if let Octopus::Energy(n) = x {
                *n += 1;
            }
        }
    }
}

fn values_to_process(data: &Vec<Vec<Octopus>>) -> usize {
    let mut count = 0;

    for xs in data {
        for x in xs {
            if let &Octopus::Energy(n) = x {
                if n > 9 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn simulate_flashing(ys: &mut Vec<Vec<Octopus>>) -> usize {
    let mut flashes = 0;

    while values_to_process(&ys) > 0 {
        for y in 0..ys.len() {
            for x in 0..ys[0].len() {
                if let Octopus::Energy(n) = ys[y][x] {
                    if n > 9 {
                        ys[y][x] = Octopus::Flashing;
                        flashes += 1;

                        if y > 0 && x > 0 {
                            if let Some(v) = ys.get_mut(y - 1).and_then(|v| v.get_mut(x - 1)) {
                                if let Octopus::Energy(n) = v {
                                    *n += 1;
                                }
                            }
                        }

                        if y > 0 {
                            if let Some(v) = ys.get_mut(y - 1).and_then(|v| v.get_mut(x)) {
                                if let Octopus::Energy(n) = v {
                                    *n += 1;
                                }
                            }

                            if let Some(v) = ys.get_mut(y - 1).and_then(|v| v.get_mut(x + 1)) {
                                if let Octopus::Energy(n) = v {
                                    *n += 1;
                                }
                            }
                        }

                        if x > 0 {
                            if let Some(v) = ys.get_mut(y).and_then(|v| v.get_mut(x - 1)) {
                                if let Octopus::Energy(n) = v {
                                    *n += 1;
                                }
                            }

                            if let Some(v) = ys.get_mut(y + 1).and_then(|v| v.get_mut(x - 1)) {
                                if let Octopus::Energy(n) = v {
                                    *n += 1;
                                }
                            }
                        }

                        if let Some(v) = ys.get_mut(y).and_then(|v| v.get_mut(x + 1)) {
                            if let Octopus::Energy(n) = v {
                                *n += 1;
                            }
                        }

                        if let Some(v) = ys.get_mut(y + 1).and_then(|v| v.get_mut(x)) {
                            if let Octopus::Energy(n) = v {
                                *n += 1;
                            }
                        }

                        if let Some(v) = ys.get_mut(y + 1).and_then(|v| v.get_mut(x + 1)) {
                            if let Octopus::Energy(n) = v {
                                *n += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    flashes
}

fn reset_flashing(data: &mut Vec<Vec<Octopus>>) {
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
                .map(|n| Octopus::new(n.to_string().parse::<u32>().unwrap()))
                .collect()
        })
        .collect();

    let mut total_flashes = 0;

    for i in 0.. {
        increment_energy(&mut data);

        let flashes = simulate_flashing(&mut data);

        total_flashes += flashes;

        reset_flashing(&mut data);

        if i == 100 {
            println!("{}", total_flashes);
        }

        if flashes == data.len() * data[0].len() {
            println!("{}", i + 1);
            break;
        }
    }
}
