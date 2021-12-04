use std::fs;

#[derive(Debug)]
struct BingoBoard {
    board: [[(u8, bool); 5]; 5],
}

impl BingoBoard {
    fn new() -> Self {
        BingoBoard {
            board: [[(0, false); 5]; 5],
        }
    }

    fn parse(lines: &[&str]) -> Self {
        let mut board = BingoBoard::new();

        let nums = lines
            .iter()
            .map(|line| {
                line.trim()
                    .split_whitespace()
                    .map(|v| v.trim().parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        board.populate(&nums);
        board
    }

    fn populate(&mut self, nums: &[Vec<u8>]) {
        for (i, line) in nums.iter().enumerate() {
            for (j, num) in line.iter().enumerate() {
                self.board[i][j] = (*num, false);
            }
        }
    }

    fn mark_number(&mut self, num: u8) {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j].0 == num {
                    self.board[i][j] = (num, true);
                    return;
                }
            }
        }
    }

    fn is_complete(&self) -> bool {
        // test lines
        for line in self.board {
            if line.iter().all(|(_, is_marked)| *is_marked) {
                return true;
            }
        }

        // test columns
        for j in 0..5 {
            if (0..5).all(|i| self.board[i][j].1) {
                return true;
            }
        }

        false
    }

    fn get_unmarked_sum(&self) -> u64 {
        self.board
            .iter()
            .flatten()
            .filter(|&&(_, is_marked)| !is_marked)
            .map(|&(num, _)| num)
            .fold(0u64, |acc, v| acc + (v as u64))
    }
}

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Could not read file");
    let mut lines = contents.lines();

    let draws = lines
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = vec![];
    let mut cur_board = vec![];

    lines.next();
    for line in lines {
        if line.chars().next() == None {
            boards.push(BingoBoard::parse(&cur_board));
            cur_board.clear();
        } else {
            cur_board.push(line);
        }
    }
    boards.push(BingoBoard::parse(&cur_board));

    // part 1

    'outer: for &draw in &draws {
        for board in boards.iter_mut() {
            board.mark_number(draw);

            if board.is_complete() {
                println!("Part 1: {}", (draw as u64) * board.get_unmarked_sum());
                break 'outer;
            }
        }
    }

    // part 2

    for draw in draws {
        for board in boards.iter_mut() {
            board.mark_number(draw);
        }

        let mut i = 0;
        while i < boards.len() {
            let complete = boards[i].is_complete();

            if boards.len() == 1 && complete {
                println!("Part 2: {}", (draw as u64) * boards[0].get_unmarked_sum());
                return;
            } else if complete {
                boards.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
