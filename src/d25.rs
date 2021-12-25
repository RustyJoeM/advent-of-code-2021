use std::collections::HashSet;

mod utils;
const DAY_ID: utils::DayIdType = 25;

#[derive(Debug, Clone)]
struct Population {
    rows: usize,
    cols: usize,
    easts: HashSet<(usize, usize)>,
    souths: HashSet<(usize, usize)>,
}

fn parse_input(data: &str) -> Population {
    let mut easts = HashSet::new();
    let mut souths = HashSet::new();

    for (r, line) in data.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            match ch {
                '>' => {
                    easts.insert((r, c));
                }
                'v' => {
                    souths.insert((r, c));
                }
                _ => {}
            }
        }
    }
    let rows = data.lines().count();
    let cols = data.lines().next().unwrap().len();

    Population {
        rows,
        cols,
        easts,
        souths,
    }
}

impl Population {
    fn char_at(&self, row: usize, col: usize) -> char {
        if self.easts.contains(&(row, col)) {
            '>'
        } else if self.souths.contains(&(row, col)) {
            'v'
        } else {
            '.'
        }
    }

    #[allow(dead_code)]
    pub fn debug_print(&self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                print!("{}", self.char_at(r, c));
            }
            println!();
        }
    }

    pub fn free_east(&self, row: usize, col: usize) -> bool {
        let col = (col + 1) % self.cols;
        !self.easts.contains(&(row, col)) && !self.souths.contains(&(row, col))
    }

    pub fn free_south(&self, row: usize, col: usize) -> bool {
        let row = (row + 1) % self.rows;
        !self.easts.contains(&(row, col)) && !self.souths.contains(&(row, col))
    }
}

fn solve_part1(cucumbers: &Population) -> usize {
    let mut steps = 0;

    let mut pop = cucumbers.clone();

    let mut done_something = true;
    while done_something {
        done_something = false;

        let moving: Vec<(usize, usize)> = pop
            .easts
            .iter()
            .filter(|(r, c)| pop.free_east(*r, *c))
            .copied()
            .collect();

        for (r, c) in moving {
            pop.easts.remove(&(r, c));
            pop.easts.insert((r, (c + 1) % pop.cols));
            done_something = true;
        }

        let moving: Vec<(usize, usize)> = pop
            .souths
            .iter()
            .filter(|(r, c)| pop.free_south(*r, *c))
            .copied()
            .collect();

        for (r, c) in moving {
            pop.souths.remove(&(r, c));
            pop.souths.insert(((r + 1) % pop.rows, c));
            done_something = true;
        }

        steps += 1;
    }

    steps
}

fn solve_part2(_cucumbers: &Population) -> usize {
    // no part 2 on day 25 - gather all the previous stars...
    0
}

generate_main!();

generate_tests!(58, 0);
