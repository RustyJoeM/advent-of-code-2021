use std::collections::{HashMap, HashSet};

mod utils;
const DAY_ID: utils::DayIdType = 19;

type Num = i32;
type Res = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Beacon {
    x: Num,
    y: Num,
    z: Num,
}

#[derive(Debug, Clone)]
struct Scanner {
    id: usize,
    beacons: Vec<Beacon>,
}

fn parse_input(data: &str) -> Vec<Scanner> {
    let mut scanners = vec![];

    for group in data.split("\n\n") {
        let mut lines = group.lines();
        let id = lines
            .next()
            .unwrap()
            .split("--- scanner ")
            .last()
            .unwrap()
            .split(' ')
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let beacons = lines
            .map(|line| {
                let coords = line
                    .split(',')
                    .map(|x| x.parse::<Num>().unwrap())
                    .collect::<Vec<Num>>();
                Beacon {
                    x: coords[0],
                    y: coords[1],
                    z: coords[2],
                }
            })
            .collect();
        scanners.push(Scanner { id, beacons });
    }

    scanners
}

impl Beacon {
    pub fn abs(&self) -> Beacon {
        Beacon {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn size(&self) -> usize {
        (self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as usize
    }

    pub fn distance(&self, other: &Beacon) -> Beacon {
        Beacon {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn rotate(&self, index: usize) -> Beacon {
        let &Self { x, y, z } = self;
        let transformed = match index {
            0 => (-z, -y, -x),
            1 => (-z, -x, y),
            2 => (-z, x, -y),
            3 => (-z, y, x),
            4 => (-y, -z, x),
            5 => (-y, -x, -z),
            6 => (-y, x, z),
            7 => (-y, z, -x),
            8 => (-x, -z, -y),
            9 => (-x, -y, z),
            10 => (-x, y, -z),
            11 => (-x, z, y),
            12 => (x, -z, y),
            13 => (x, -y, -z),
            14 => (x, y, z),
            15 => (x, z, -y),
            16 => (y, -z, -x),
            17 => (y, -x, z),
            18 => (y, x, -z),
            19 => (y, z, x),
            20 => (z, -y, x),
            21 => (z, -x, -y),
            22 => (z, x, y),
            23 => (z, y, -x),
            _ => unreachable!(),
        };
        Beacon {
            x: transformed.0,
            y: transformed.1,
            z: transformed.2,
        }
    }
}

impl Scanner {
    pub fn rotate(&self, var: usize) -> Scanner {
        let id = 100 * self.id + var;
        let beacons = self.beacons.iter().map(|b| b.rotate(var)).collect();
        Scanner { id, beacons }
    }

    pub fn extend_by(&mut self, other: &Scanner, offset: &Beacon) {
        let Beacon { x, y, z } = offset;
        let beacons = &mut self.beacons;
        for b in other.beacons.iter() {
            let fixed = Beacon {
                x: b.x + x,
                y: b.y + y,
                z: b.z + z,
            };
            if !beacons.contains(&fixed) {
                beacons.push(fixed);
            }
        }
        beacons.sort_unstable();
        beacons.dedup();
    }
}

fn scanners_overlap(left: &Scanner, right: &Scanner) -> Option<Beacon> {
    let mut distances: HashMap<Beacon, HashSet<Beacon>> = HashMap::new();
    for lb in &left.beacons {
        for rb in &right.beacons {
            let distance = lb.distance(rb);
            distances.entry(distance).or_default().insert(*rb);
        }
    }
    distances
        .iter()
        .find(|(_, e)| e.len() >= 12)
        .map(|entry| entry.0)
        .copied()
}

fn solve_part1(scanners: &[Scanner]) -> Res {
    let mut acc = scanners[0].clone();
    let mut scanners = scanners.iter().skip(1).cloned().collect::<Vec<Scanner>>();

    while !scanners.is_empty() {
        'attempt: for i in 0..scanners.len() {
            for var in 0..24 {
                let rotated = scanners[i].rotate(var);
                if let Some(offset) = scanners_overlap(&acc, &rotated) {
                    acc.extend_by(&rotated, &offset);
                    scanners.remove(i);
                    break 'attempt;
                }
            }
        }
    }

    acc.beacons.len()
}

fn solve_part2(scanners: &[Scanner]) -> Res {
    let mut acc = scanners[0].clone();
    let mut scanners = scanners.iter().skip(1).cloned().collect::<Vec<Scanner>>();

    let mut scanner_distances: Vec<Beacon> = vec![];

    while !scanners.is_empty() {
        'attempt: for i in 0..scanners.len() {
            for var in 0..24 {
                let rotated = scanners[i].rotate(var);
                if let Some(offset) = scanners_overlap(&acc, &rotated) {
                    acc.extend_by(&rotated, &offset);
                    scanner_distances.push(offset);
                    scanners.remove(i);
                    break 'attempt;
                }
            }
        }
    }

    let mut taxis: Vec<Vec<Beacon>> = vec![];

    for (i, b1) in scanner_distances.iter().enumerate() {
        let mut dists = vec![];
        for (j, b2) in scanner_distances.iter().enumerate() {
            if i == j {
                continue;
            }
            dists.push(b1.distance(b2).abs());
        }
        dists.sort_unstable();
        taxis.push(dists);
    }
    taxis.sort_unstable();

    let mut max_dist = 0;
    let mut max_beacon = taxis[0][0];

    for b in taxis.iter().flatten() {
        let dist = b.size();
        if dist > max_dist {
            max_dist = dist;
            max_beacon = *b;
        }
    }

    (max_beacon.x + max_beacon.y + max_beacon.z) as usize
}

#[allow(dead_code)]
fn debug_vec_vec_beacons(taxis: &[Vec<Beacon>]) {
    for t in taxis.iter() {
        for b in t.iter() {
            print!("{{{:-5}, {:-5}, {:-5}}}, ", b.x, b.y, b.z);
        }
        println!();
    }
}

generate_main!();

generate_tests!(79, 3621);
