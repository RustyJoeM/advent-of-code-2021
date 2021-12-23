use std::cmp::Ordering;

mod utils;
const DAY_ID: utils::DayIdType = 17;

type Res = i32;

#[derive(Debug, Copy, Clone)]
struct Vec2<T> {
    x: T,
    y: T,
}

#[derive(Debug, Copy, Clone)]
struct TargetArea {
    x: (Res, Res),
    y: (Res, Res),
}

fn text_range_to_tuple(s: &str) -> (Res, Res) {
    let s = s
        .split("..")
        .map(|x| x.parse::<Res>().unwrap())
        .collect::<Vec<Res>>();
    (s[0], s[1])
}

fn parse_input(data: &str) -> TargetArea {
    // target area: x=20..30, y=-10..-5
    let words = data.split_whitespace().collect::<Vec<&str>>();
    let w2 = words[2];
    let x = text_range_to_tuple(&w2[2..w2.len() - 1]);
    let w3 = words[3];
    let y = text_range_to_tuple(&w3[2..]);
    TargetArea { x, y }
}

#[derive(Debug, Clone)]
struct Probe {
    velocity: Vec2<Res>,
    position: Vec2<Res>,
    max_height: Res,
}

impl Probe {
    fn step(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        match self.velocity.x.cmp(&0) {
            Ordering::Greater => self.velocity.x -= 1,
            Ordering::Less => self.velocity.x += 1,
            Ordering::Equal => {}
        }

        self.velocity.y -= 1;

        // update max reached height (if applicable)
        if self.max_height < self.position.y {
            self.max_height = self.position.y;
        }
    }

    fn is_inside(&self, area: &TargetArea) -> bool {
        let x_in = self.position.x >= area.x.0 && self.position.x <= area.x.1;
        let y_in = self.position.y >= area.y.0 && self.position.y <= area.y.1;
        x_in && y_in
    }

    fn definitely_missed(&self, area: &TargetArea) -> bool {
        let x = self.position.x;
        let y = self.position.y;

        // beyond X
        if x > area.x.1 {
            return true;
        }
        // too deep
        if y < area.y.0 {
            return true;
        }
        // falling down only out of target X range
        if self.velocity.x == 0 && (x < area.x.0 || x > area.x.1) {
            return true;
        }

        false
    }
}

fn release_probe(area: &TargetArea) -> (Res, Res) {
    let mut hits = 0;
    let mut max_height = 0;

    let max_velocity_y = area.y.0.abs();

    for vel_y in -max_velocity_y..max_velocity_y {
        for vel_x in 1..=area.x.1 {
            let mut probe = Probe {
                velocity: Vec2 { x: vel_x, y: vel_y },
                position: Vec2 { x: 0, y: 0 },
                max_height: 0,
            };
            loop {
                probe.step();
                if probe.is_inside(area) {
                    hits += 1;
                    if probe.max_height > max_height {
                        max_height = probe.max_height;
                    }
                    break;
                }
                if probe.definitely_missed(area) {
                    break;
                }
            }
        }
    }

    (hits, max_height)
}

fn solve_part1(area: &TargetArea) -> Res {
    let (_, max_height) = release_probe(area);
    max_height
}

fn solve_part2(area: &TargetArea) -> Res {
    let (hits, _) = release_probe(area);
    hits
}

generate_main!();

generate_tests!(45, 112);
