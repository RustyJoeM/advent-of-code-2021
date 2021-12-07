use std::collections::HashMap;

mod utils;

type Number = u32;
type InputRow = Vec<Number>;
type InputBoard = Vec<InputRow>;
type InputBoardRef<'a> = &'a [InputRow];

fn parse_input(data: &str) -> (Vec<Number>, Vec<InputBoard>) {
    let drawn_numbers: Vec<Number> = data
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<Number>().unwrap())
        .collect();

    let mut boards: Vec<InputBoard> = Default::default();
    let mut pending_board: InputBoard = Default::default();

    for line in data.lines().skip(2) {
        if line.is_empty() {
            boards.push(pending_board);
            pending_board = Default::default();
            continue;
        }

        let board_row: InputRow = line
            .split_whitespace()
            .map(|x| x.parse::<Number>().unwrap())
            .collect();
        pending_board.push(board_row);
    }

    if !pending_board.is_empty() {
        boards.push(pending_board);
    }

    (drawn_numbers, boards)
}

type BingoCoords = (usize, usize);
type BingoCell = (Number, bool);
type BingoState = HashMap<BingoCoords, BingoCell>;

#[derive(Debug, Clone)]
struct BingoBoard {
    size: usize,
    state: BingoState,
    pub is_winning: bool,
    pub score: Number,
}

impl BingoBoard {
    pub fn new(fields: InputBoardRef) -> Self {
        let size = fields.len();
        let mut score = 0;
        let mut state: BingoState = Default::default();
        #[allow(clippy::needless_range_loop)]
        for row in 0..size {
            for col in 0..size {
                let num = fields[row][col];
                let key = (row, col);
                state.insert(key, (num, false));
                score += num;
            }
        }
        Self {
            size,
            state,
            is_winning: false,
            score,
        }
    }

    pub fn draw_number(&mut self, num: Number) {
        let mut drawn_coords: Option<BingoCoords> = None;
        self.state
            .iter_mut()
            .filter(|(_, (n, _))| *n == num)
            .for_each(|(k, val)| {
                if num == val.0 {
                    val.1 = true;
                    self.score -= num;
                    drawn_coords = Some(*k);
                }
            });
        if let Some((x, y)) = drawn_coords {
            let row_full = (0..self.size).all(|i| {
                let v = self.state.get(&(x, i)).unwrap();
                v.1
            });
            let column_full = (0..self.size).all(|i| {
                let v = self.state.get(&(i, y)).unwrap();
                v.1
            });
            if row_full || column_full {
                self.is_winning = true;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Bingo {
    pub boards: Vec<BingoBoard>,
}

impl Bingo {
    pub fn new(boards: &[InputBoard]) -> Self {
        let boards = boards
            .iter()
            .map(|board| BingoBoard::new(board))
            .collect::<Vec<BingoBoard>>();
        Self { boards }
    }

    pub fn remove_won(&mut self) {
        self.boards.retain(|x| !x.is_winning);
    }

    pub fn draw_number(&mut self, num: Number) {
        self.boards
            .iter_mut()
            .for_each(|board| board.draw_number(num));
    }

    pub fn get_winning_boards(&self) -> Vec<BingoBoard> {
        self.boards
            .iter()
            .filter(|b| b.is_winning)
            .cloned()
            .collect()
    }
}

fn solve_part1(data: &(Vec<Number>, Vec<InputBoard>)) -> Number {
    let (drawn_numbers, init_boards) = data;
    let mut bingo = Bingo::new(init_boards);

    for &num in drawn_numbers.iter() {
        bingo.draw_number(num);
        let winning_boards = bingo.get_winning_boards();
        if !winning_boards.is_empty() {
            let score = winning_boards[0].score;
            return score * num;
        }
    }
    unreachable!();
}

fn solve_part2(data: &(Vec<Number>, Vec<InputBoard>)) -> Number {
    let (drawn_numbers, init_boards) = data;
    let mut bingo = Bingo::new(init_boards);

    for &num in drawn_numbers.iter() {
        bingo.draw_number(num);
        let winning_boards = bingo.get_winning_boards();
        if !winning_boards.is_empty() {
            if bingo.boards.len() > 1 {
                bingo.remove_won();
                continue;
            }
            let score = winning_boards[0].score;
            return score * num;
        }
    }
    unreachable!();
}

generate_main!(4);

generate_tests!(4, 188 * 24, 148 * 13);
