use std::fs::read_to_string;

#[derive(Debug)]
struct Numbers(Vec<u32>);

impl Numbers {
    fn new(input: &str) -> Numbers {
        Numbers(
            input
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect(),
        )
    }
}

impl From<&[u32]> for Numbers {
    fn from(numbers: &[u32]) -> Self {
        Numbers(numbers.iter().copied().collect::<Vec<u32>>())
    }
}

#[derive(Debug)]
struct Board([[u32; 5]; 5]);

impl Board {
    fn new(input: &str) -> Board {
        let mut board = Board([[0; 5]; 5]);

        input.lines().enumerate().for_each(|(i, line)| {
            line.split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .enumerate()
                .for_each(|(j, number)| board.0[i][j] = number)
        });

        board
    }

    fn rows(&self) -> Vec<Vec<u32>> {
        self.0
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect()
    }

    fn columns(&self) -> Vec<Vec<u32>> {
        let rows = self.rows();
        let mut columns: Vec<Vec<u32>> = vec![[].to_vec(); rows[0].len()];

        for row in rows {
            row.into_iter()
                .enumerate()
                .for_each(|(i, number)| columns[i].push(number))
        }

        columns
    }

    fn is_winning(&self, numbers: &Numbers) -> bool {
        for row in self.rows() {
            if row.iter().all(|number| numbers.0.contains(number)) {
                return true;
            }
        }

        for column in self.columns() {
            if column.iter().all(|number| numbers.0.contains(number)) {
                return true;
            }
        }

        false
    }

    fn unmarked_sum(&self, numbers: &Numbers) -> u32 {
        let mut sum = 0;

        for row in self.rows() {
            for n in row.iter() {
                if !numbers.0.contains(n) {
                    sum += n;
                }
            }
        }

        sum
    }
}

#[derive(Debug)]
struct BingoSubsystem {
    numbers: Numbers,
    boards: Vec<Board>,
}

impl BingoSubsystem {
    fn new(input: &str) -> BingoSubsystem {
        let mut parts = input.split("\n\n");
        let numbers = Numbers::new(parts.next().unwrap());
        let boards = parts.map(Board::new).collect();

        BingoSubsystem { boards, numbers }
    }
}

fn main() {
    let input = read_to_string("input/day4/input").unwrap();
    let mut bingo = BingoSubsystem::new(&input);

    'outer: for i in 5..bingo.numbers.0.len() {
        let numbers = &bingo.numbers.0[..i];
        for board in bingo.boards.iter() {
            if board.is_winning(&numbers.into()) {
                println!(
                    "{}",
                    board.unmarked_sum(&numbers.into()) * numbers.last().unwrap()
                );
                break 'outer;
            }
        }
    }

    for i in 5..bingo.numbers.0.len() {
        let numbers = &bingo.numbers.0[..i];
        bingo.boards.retain(|board| {
            if board.is_winning(&numbers.into()) {
                println!(
                    "{}",
                    board.unmarked_sum(&numbers.into()) * numbers.last().unwrap()
                );
                return false;
            }
            true
        })
    }
}
