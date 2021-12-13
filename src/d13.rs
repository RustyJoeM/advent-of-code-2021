use std::{borrow::BorrowMut, collections::HashSet};

mod utils;
const DAY_ID: utils::DayIdType = 13;

#[derive(Debug, Copy, Clone)]
enum Fold {
    X(usize),
    Y(usize),
}

type DotsT = HashSet<(usize, usize)>;

#[derive(Debug, Clone)]
struct TransparentPaper {
    dots: DotsT,
    rows: usize,
    cols: usize,
}

impl TransparentPaper {
    fn fold(&mut self, fold: &Fold) {
        let dots = self.dots.borrow_mut();
        match *fold {
            Fold::X(s) => {
                for r in 0..=self.rows {
                    for c in s + 1..=self.cols {
                        dots.remove(&(r, s)); // drop line elem
                        if dots.contains(&(r, c)) {
                            dots.remove(&(r, c)); // drop item being folded
                            dots.insert((r, s - (c - s))); // add folded copy
                        }
                    }
                }
                self.cols = s;
            }
            Fold::Y(s) => {
                for r in s + 1..=self.rows {
                    for c in 0..=self.cols {
                        dots.remove(&(s, c)); // drop line elem
                        if dots.contains(&(r, c)) {
                            dots.remove(&(r, c)); // drop item being folded
                            dots.insert((s - (r - s), c)); // add folded copy
                        }
                    }
                }
                self.rows = s;
            }
        }
    }

    fn to_nice_strings(&self) -> Vec<String> {
        let mut strs = Vec::new();
        for r in 0..=self.rows {
            let mut str = String::new();
            for c in 0..=self.cols {
                if self.dots.contains(&(r, c)) {
                    str.push('\u{2588}');
                } else {
                    str.push('.');
                }
            }
            strs.push(str);
        }
        strs
    }
}

fn parse_input(data: &str) -> (TransparentPaper, Vec<Fold>) {
    let mut dots: DotsT = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();
    let mut cols = 0;
    let mut rows = 0;

    for line in data.lines() {
        if line.contains(',') {
            let p: Vec<usize> = line
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            cols = cols.max(p[0]);
            rows = rows.max(p[1]);
            dots.insert((p[1], p[0]));
        }
        if line.contains('=') {
            let v = line.split('=').last().unwrap().parse::<usize>().unwrap();
            folds.push(if line.contains("x=") {
                Fold::X(v)
            } else {
                Fold::Y(v)
            });
        }
    }

    (TransparentPaper { dots, rows, cols }, folds)
}

fn fold_dots(paper: &mut TransparentPaper, folds: &[Fold]) {
    for fold in folds.iter() {
        paper.fold(fold);
    }
}

fn solve_part1(input: &(TransparentPaper, Vec<Fold>)) -> usize {
    let (paper, folds) = input;
    let mut paper = paper.clone();
    fold_dots(&mut paper, &folds[..1]);
    paper.dots.len()
}

fn solve_part2(input: &(TransparentPaper, Vec<Fold>)) -> usize {
    let (paper, folds) = input;
    let mut paper = paper.clone();
    fold_dots(&mut paper, folds);

    println!("\n-------- Part 2 output --------");
    for line in paper.to_nice_strings().iter() {
        println!("{}", line);
    }
    0
}

generate_main!();

generate_tests!(17, 0);
