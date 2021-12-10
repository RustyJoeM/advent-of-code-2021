use std::collections::{HashMap, HashSet};

mod utils;
const DAY_ID: utils::DayIdType = 9;

type Res = u32;
type DepthType = u8;

#[derive(Debug)]
struct DepthMap {
    heights: HashMap<(i32, i32), DepthType>,
    rows: usize,
    columns: usize,
}

impl DepthMap {
    pub fn get(&self, r: i32, c: i32) -> Option<DepthType> {
        self.heights.get(&(r, c)).copied()
    }

    pub fn neighbors_of(&self, row: usize, col: usize) -> Vec<DepthType> {
        let mut neighbors = Vec::with_capacity(4);
        const OFFSET: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        for (y, x) in OFFSET.iter() {
            let r = row as i32 + y;
            let c = col as i32 + x;
            if let Some(&x) = self.heights.get(&(r, c)).or(Some(&9)) {
                neighbors.push(x);
            }
        }
        neighbors
    }

    pub fn get_basin_score(
        &self,
        visited: &mut HashSet<(usize, usize)>,
        r: i32,
        c: i32,
        skip_direction: SkipDirection,
    ) -> Res {
        if r < 0 || (r > self.rows as i32 - 1) || c < 0 || (c > self.columns as i32 - 1) {
            return 0;
        }

        let me = self.get(r, c);
        if me == Some(9) {
            return 0;
        }

        let mut score = 0;

        if skip_direction != SkipDirection::Left && me < self.get(r, c - 1) {
            score += self.get_basin_score(visited, r, c - 1, SkipDirection::Right);
        }

        if skip_direction != SkipDirection::Right && me < self.get(r, c + 1) {
            score += self.get_basin_score(visited, r, c + 1, SkipDirection::Left);
        }

        if skip_direction != SkipDirection::Up && me < self.get(r - 1, c) {
            score += self.get_basin_score(visited, r - 1, c, SkipDirection::Down);
        }

        if skip_direction != SkipDirection::Down && me < self.get(r + 1, c) {
            score += self.get_basin_score(visited, r + 1, c, SkipDirection::Up);
        }

        visited.insert((r as usize, c as usize));
        score
    }
}

fn parse_input(data: &str) -> DepthMap {
    let mut heights = HashMap::new();

    let mut rows = 0;
    data.lines().enumerate().for_each(|(row, line)| {
        rows += 1;
        line.chars().enumerate().for_each(|(col, ch)| {
            heights.insert(
                (row as i32, col as i32),
                ch.to_digit(10).unwrap() as DepthType,
            );
        });
    });

    let columns = data.lines().next().unwrap().len();

    DepthMap {
        heights,
        rows,
        columns,
    }
}

fn solve_part1(data: &DepthMap) -> Res {
    let mut risk = 0;
    for r in 0..data.rows {
        for c in 0..data.columns {
            let me = data.get(r as i32, c as i32).unwrap();
            if data.neighbors_of(r, c).iter().filter(|&&x| x > me).count() == 4 {
                risk += me as Res + 1;
            }
        }
    }
    risk
}

#[derive(Eq, PartialEq)]
enum SkipDirection {
    None,
    Left,
    Right,
    Up,
    Down,
}

fn solve_part2(data: &DepthMap) -> Res {
    let mut basins: Vec<Res> = vec![];

    for r in 0..data.rows {
        for c in 0..data.columns {
            let me = data.get(r as i32, c as i32).unwrap();
            if data.neighbors_of(r, c).iter().all(|&x| x > me) {
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                data.get_basin_score(&mut visited, r as i32, c as i32, SkipDirection::None);
                basins.push(visited.len() as Res);
            }
        }
    }

    let blen = basins.len();
    basins.sort_unstable();
    basins[blen - 1] * basins[blen - 2] * basins[blen - 3]
}

generate_main!();

generate_tests!(15, 1134);
