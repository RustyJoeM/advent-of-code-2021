use std::collections::BTreeSet;

mod utils;
const DAY_ID: utils::DayIdType = 13;

#[derive(Debug, Copy, Clone)]
enum Fold {
    Row(usize),
    Column(usize),
}

type DotsT = BTreeSet<(usize, usize)>;

#[derive(Debug, Clone)]
struct TransparentPaper {
    dots: DotsT,
    rows: usize,
    cols: usize,
}

fn parse_input(data: &str) -> (TransparentPaper, Vec<Fold>) {
    let mut dots = BTreeSet::new();
    let mut cols = 0;
    let mut rows = 0;
    let mut folds: Vec<Fold> = Vec::new();

    for line in data.lines() {
        if line.contains(',') {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            cols = cols.max(x);
            rows = rows.max(y);
            dots.insert((x, y));
        } else if line.contains('=') {
            let v = line.split('=').last().unwrap().parse::<usize>().unwrap();
            folds.push(if line.contains("x=") {
                Fold::Column(v)
            } else {
                Fold::Row(v)
            });
        }
    }

    (TransparentPaper { dots, rows, cols }, folds)
}

impl TransparentPaper {
    fn fold(&mut self, folds: &[Fold]) {
        for fold in folds.iter() {
            let mut next_dots = BTreeSet::new();
            for &(x, y) in self.dots.iter() {
                match *fold {
                    Fold::Column(f) => match x.cmp(&f) {
                        std::cmp::Ordering::Less => {
                            next_dots.insert((x, y));
                        }
                        std::cmp::Ordering::Equal => {}
                        std::cmp::Ordering::Greater => {
                            next_dots.insert((f - (x - f), y));
                        }
                    },
                    Fold::Row(f) => match y.cmp(&f) {
                        std::cmp::Ordering::Less => {
                            next_dots.insert((x, y));
                        }
                        std::cmp::Ordering::Equal => {}
                        std::cmp::Ordering::Greater => {
                            next_dots.insert((x, f - (y - f)));
                        }
                    },
                }
            }
            self.dots = next_dots;
            match *fold {
                Fold::Column(f) => {
                    self.cols = f - 1;
                }
                Fold::Row(f) => {
                    self.rows = f - 1;
                }
            }
        }
    }

    fn to_nice_strings(&self) -> Vec<String> {
        let mut strs = Vec::new();
        for r in 0..=self.rows {
            let mut s = String::new();
            for c in 0..=self.cols {
                if self.dots.contains(&(c, r)) {
                    s.push('\u{2588}');
                } else {
                    s.push('.');
                }
            }
            strs.push(s);
        }
        strs
    }

    fn debug_print(&self) {
        for line in self.to_nice_strings().iter() {
            println!("{}", line);
        }
    }
}

fn solve_part1((paper, folds): &(TransparentPaper, Vec<Fold>)) -> usize {
    let mut paper = paper.clone();
    paper.fold(&folds[..1]);
    paper.dots.len()
}

fn solve_part2((paper, folds): &(TransparentPaper, Vec<Fold>)) -> usize {
    let mut paper = paper.clone();
    paper.fold(folds);

    println!("\n-------- Part 2 output --------");
    paper.debug_print();
    0
}

generate_main!();

generate_tests!(17, 0);
