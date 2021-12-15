use std::fs::read_to_string;
use std::iter;

type RiskMatrix = Vec<Vec<u32>>;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Path {
    path: Vec<Point>,
    sum: u32,
}

impl Path {
    fn append(&self, point: Point, sum: u32) -> Path {
        Path {
            path: self.path.iter().copied().chain(iter::once(point)).collect(),
            sum,
        }
    }
}

struct PathMatrix {
    path_matrix: Vec<Vec<Path>>,
    risk_matrix: RiskMatrix,
    size: Point,
}

impl PathMatrix {
    fn new(risk_matrix: RiskMatrix) -> PathMatrix {
        let size = Point {
            x: risk_matrix[0].len(),
            y: risk_matrix.len(),
        };

        let mut path_matrix = vec![
            vec![
                Path {
                    sum: u32::MAX - 9,
                    path: Vec::new()
                };
                size.x
            ];
            size.y
        ];

        path_matrix[0][0] = Path {
            sum: 0,
            path: Vec::new(),
        };

        PathMatrix {
            path_matrix,
            risk_matrix,
            size,
        }
    }

    fn compare(&mut self, point: Point, neighbor: Point) {
        let neighbor = &self.path_matrix[neighbor.y][neighbor.x];
        let risk = self.risk_matrix[point.y][point.x];
        let sum = neighbor.sum + risk;

        if sum < self.path_matrix[point.y][point.x].sum {
            self.path_matrix[point.y][point.x] = neighbor.append(point, sum);
        }
    }

    fn optimize(&mut self, point: Point) {
        if point.x > 0 {
            self.compare(
                point,
                Point {
                    x: point.x - 1,
                    ..point
                },
            );
        }
        if point.x + 1 < self.size.x {
            self.compare(
                point,
                Point {
                    x: point.x + 1,
                    ..point
                },
            );
        }
        if point.y > 0 {
            self.compare(
                point,
                Point {
                    y: point.y - 1,
                    ..point
                },
            );
        }
        if point.y + 1 < self.size.y {
            self.compare(
                point,
                Point {
                    y: point.y + 1,
                    ..point
                },
            );
        }
    }

    fn answer(&self) -> u32 {
        self.path_matrix[self.size.y - 1][self.size.x - 1].sum
    }
}

fn parse_input(s: &str) -> RiskMatrix {
    s.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn wrap(x: u32) -> u32 {
    (x - 1) % 9 + 1
}

fn multiply(risk_matrix: RiskMatrix, factor: usize) -> RiskMatrix {
    let product_size = Point {
        x: risk_matrix[0].len(),
        y: risk_matrix.len(),
    };

    let factor_size = Point {
        x: factor * risk_matrix[0].len(),
        y: factor * risk_matrix.len(),
    };

    let mut product = vec![vec![0; factor_size.x]; factor_size.y];

    for x in 0..product_size.x {
        for y in 0..product_size.y {
            for i in 0..factor {
                for j in 0..factor {
                    product[y][x + i * product_size.x] = wrap(risk_matrix[y][x] + i as u32);
                    product[y + j * product_size.y][x] = wrap(risk_matrix[y][x] + j as u32);
                    product[y + j * product_size.y][x + i * product_size.x] =
                        wrap(risk_matrix[y][x] + (i + j) as u32);
                }
            }
        }
    }

    product
}

fn main() {
    let input = read_to_string("input/day15/input").unwrap();
    let risk_matrix = parse_input(&input);

    let mut path_matrix = PathMatrix::new(risk_matrix.clone());
    let mut path_matrix_product = PathMatrix::new(multiply(risk_matrix, 5));

    for _ in 0..10 {
        for y in 0..path_matrix.size.y {
            for x in 0..path_matrix.size.x {
                path_matrix.optimize(Point { x, y });
            }
        }
        for y in 0..path_matrix_product.size.y {
            for x in 0..path_matrix_product.size.x {
                path_matrix_product.optimize(Point { x, y });
            }
        }
    }

    println!("{} {} ", path_matrix.answer(), path_matrix_product.answer());
}
