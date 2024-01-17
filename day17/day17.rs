use std::fs::read_to_string;

#[derive(Debug)]
struct Probe {
    x: i32,
    y: i32,
    x_velocity: i32,
    y_velocity: i32,
}

impl Probe {
    fn new(x_velocity: i32, y_velocity: i32) -> Probe {
        Probe {
            x: 0,
            y: 0,
            x_velocity,
            y_velocity,
        }
    }

    fn update(&mut self) {
        self.x += self.x_velocity;
        self.y += self.y_velocity;
        self.x_velocity += match self.x_velocity {
            1.. => -1,
            0 => 0,
            _ => 1,
        };
        self.y_velocity -= 1;
    }

    fn overkill(&self, area: &Area) -> bool {
        self.x > area.x1.max(area.x2) || self.y < area.y1.min(area.y2)
    }

    fn within(&self, area: &Area) -> bool {
        self.x <= area.x1.max(area.x2)
            && self.x >= area.x1.min(area.x2)
            && self.y <= area.y1.max(area.y2)
            && self.y >= area.y1.min(area.y2)
    }
}

#[derive(Debug)]
struct Area {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Area {
    fn new(s: &str) -> Area {
        let mut s = s.split_whitespace().skip(2).map(|s| {
            s.split_once('=')
                .unwrap()
                .1
                .trim_matches(',')
                .split("..")
                .map(|n| n.parse().unwrap())
        });

        let (mut s1, mut s2) = (s.next().unwrap(), s.next().unwrap());

        let (x1, x2) = (s1.next().unwrap(), s1.next().unwrap());
        let (y1, y2) = (s2.next().unwrap(), s2.next().unwrap());

        Area { x1, x2, y1, y2 }
    }
}

fn main() {
    let input = read_to_string("input/day17/input").unwrap();
    let area = Area::new(&input);

    let mut highest_y = 0;
    let mut count = 0;

    for x in -1000..1000 {
        for y in -1000..1000 {
            let mut probe = Probe::new(x, y);
            let mut max_y = 0;

            while !probe.overkill(&area) {
                if probe.y > max_y {
                    max_y = probe.y;
                }

                if probe.within(&area) {
                    if max_y > highest_y {
                        highest_y = max_y;
                    }
                    count += 1;
                    break;
                }

                probe.update();
            }
        }
    }

    println!("{} {}", highest_y, count);
}
