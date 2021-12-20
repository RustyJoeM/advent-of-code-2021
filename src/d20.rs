use std::collections::HashSet;

mod utils;
const DAY_ID: utils::DayIdType = 20;

type Res = usize;

type Lights = HashSet<(i32, i32)>;

fn parse_input(data: &str) -> Screen {
    let mut lines = data.lines();

    let algo = lines.next().unwrap().chars().collect::<Vec<char>>();

    lines.next();

    let mut lights = HashSet::new();
    let mut max_row = 0;
    let mut max_col = 0;
    for (row, line) in lines.enumerate() {
        max_row += 1;
        max_col = line.len() as i32;
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                lights.insert((row as i32, col as i32));
            }
        }
    }

    let is_charging = algo[0] == '#';
    let is_shutting = algo[algo.len()-1] == '.';

    Screen {
        algo,
        is_charging,
        is_shutting,
        lights,
        min_row: 0,
        max_row,
        min_col: 0,
        max_col,
    }
}

#[derive(Debug, Clone)]
struct Screen {
    algo: Vec<char>,
    is_charging: bool,
    is_shutting: bool,
    lights: Lights,
    min_row: i32,
    max_row: i32,
    min_col: i32,
    max_col: i32,
}

impl Screen {
    fn pixel_value(&self, row: i32, col: i32, is_outer_lighted: bool) -> usize {
        let mut binary_str = "".to_string();
        for r in row - 1..=row + 1 {
            for c in col - 1..=col + 1 {
                let is_out = r <= self.min_row
                    || r >= self.max_row
                    || c <= self.min_col
                    || c >= self.max_col;
                let next_char = if is_out {
                    if is_outer_lighted {
                        '1'
                    } else {
                        '0'
                    }
                } else if self.lights.contains(&(r, c)) {
                    '1'
                } else {
                    '0'
                };
                binary_str.push(next_char);
            }
        }
        usize::from_str_radix(binary_str.as_str(), 2).unwrap()
    }

    fn new_pixel(&self, row: i32, col: i32, is_outer_lighted: bool) -> char {
        let val = self.pixel_value(row, col, is_outer_lighted);
        self.algo[val]
    }

    pub fn enhance(&mut self, steps: usize) {
        let mut is_outer_lighted = false;

        for _ in 0..steps {
            let mut new_lights = HashSet::new();

            self.min_row -= 1;
            self.max_row += 1;
            self.min_col -= 1;
            self.max_col += 1;

            for r in self.min_row..=self.max_row {
                for c in self.min_col..=self.max_col {
                    let p = self.new_pixel(r, c, is_outer_lighted);
                    if p == '#' {
                        new_lights.insert((r, c));
                    }
                }
            }

            if is_outer_lighted && self.is_shutting {
                is_outer_lighted = false;
            } else if self.is_charging {
                is_outer_lighted = true;
            }

            self.lights = new_lights;
        }
        // self.debug_print();
    }

    #[allow(dead_code)]
    pub fn debug_print(&self) {
        println!("--------------------------------");
        for r in self.min_row..self.max_row {
            for c in self.min_col..self.max_col {
                let ch = if self.lights.contains(&(r, c)) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

fn solve_part1(screen: &Screen) -> Res {
    let mut screen = screen.clone();
    screen.enhance(2);
    screen.lights.len()
}

fn solve_part2(screen: &Screen) -> Res {
    let mut screen = screen.clone();
    screen.enhance(50);
    screen.lights.len()
}

generate_main!();

generate_tests!(35, 3351);
