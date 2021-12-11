mod utils;
const DAY_ID: utils::DayIdType = 11;

type Res = u32;

#[derive(Copy, Clone)]
struct Octopus {
    energy: u8,
    flashed: bool,
}

impl Octopus {
    pub fn charge(&mut self) {
        self.energy += 1;
    }
}

#[derive(Clone)]
struct Field {
    pub octopi: Vec<Vec<Octopus>>,
    pub rows: i32,
    pub cols: i32,
}

impl Field {
    pub fn iter_mut_octopi(&mut self) -> impl IntoIterator<Item = &mut Octopus> {
        self.octopi.iter_mut().flatten()
    }

    pub fn iter_coords(&self) -> impl IntoIterator<Item = (i32, i32)> {
        (0..self.rows)
            .map(|x| (0..self.cols).map(move |y| (x, y)))
            .flatten()
            .collect::<Vec<_>>()
    }

    fn is_out(&self, r: i32, c: i32) -> bool {
        r < 0 || r > self.rows - 1 || c < 0 || c > self.cols - 1
    }

    fn charge(&mut self, r: i32, c: i32) {
        if !self.is_out(r, c) {
            let octopus = &mut self.octopi[r as usize][c as usize];
            octopus.charge();
        }
    }

    pub fn do_flash(&mut self, r: i32, c: i32) {
        if self.is_out(r, c) {
            return;
        }

        let mut octopus = &mut self.octopi[r as usize][c as usize];

        if octopus.energy <= 9 || octopus.flashed {
            return;
        }
        octopus.flashed = true;

        for (r1, c1) in neighbors_of(r, c) {
            self.charge(r1, c1);
            self.do_flash(r1, c1);
        }
    }
}

fn parse_input(data: &str) -> Field {
    let octopi = data
        .lines()
        .map(|row| {
            row.chars()
                .map(|ch| Octopus {
                    energy: { ch as u8 - b'0' },
                    flashed: false,
                })
                .collect()
        })
        .collect();

    let rows = data.lines().count() as i32;
    let cols = data.lines().next().unwrap().len() as i32;

    Field { octopi, rows, cols }
}

fn neighbors_of(r: i32, c: i32) -> Vec<(i32, i32)> {
    vec![
        (r - 1, c - 1),
        (r - 1, c),
        (r - 1, c + 1),
        (r, c - 1),
        (r, c + 1),
        (r + 1, c - 1),
        (r + 1, c),
        (r + 1, c + 1),
    ]
}

fn perform_step(field: &mut Field) -> u32 {
    let mut flashes = 0;
    for octo in field.iter_mut_octopi() {
        octo.charge();
    }

    for (r, c) in field.iter_coords() {
        field.do_flash(r, c);
    }

    for octo in field.iter_mut_octopi() {
        if octo.flashed {
            octo.energy = 0;
            flashes += 1;
        }
        octo.flashed = false;
    }

    flashes
}

fn solve_part1(field: &Field) -> Res {
    let mut field = field.clone();

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += perform_step(&mut field);
    }

    flashes
}

fn solve_part2(field: &Field) -> Res {
    let mut field = field.clone();

    let max_flashes = field.rows * field.cols;

    for step in 1.. {
        if perform_step(&mut field) == max_flashes as u32 {
            return step;
        };
    }

    unreachable!();
}

generate_main!();

generate_tests!(1656, 195);
