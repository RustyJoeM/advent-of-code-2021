use std::collections::HashMap;

mod utils;
const DAY_ID: utils::DayIdType = 14;

type Res = usize;

type Transitions = HashMap<(char, char), char>;
type Counts = HashMap<(char, char), usize>;

fn parse_input(data: &str) -> (Vec<char>, Transitions) {
    let mut iter = data.lines();
    let x = iter.next().unwrap();
    let starting = x.chars().collect();

    let mut transitions = Transitions::new();
    for line in iter {
        if line.is_empty() {
            continue;
        }
        let a: Vec<char> = line.chars().collect();
        transitions.insert((a[0], a[1]), a[6]);
    }

    (starting, transitions)
}

#[allow(dead_code)]
fn debug_counts(prefix: &str, counts: &Counts) {
    println!("{}", prefix);
    counts.iter().filter(|x| {
        x.1 > &0
    }).for_each(|x| {
        println!("{:?}", x);
    });
}

fn solve(steps: usize, (starting, transitions): &(Vec<char>, Transitions)) -> Res {
    let mut counts = Counts::new();
    for (&key ,_) in transitions.iter() {
        counts.insert(key, 0);
    }

    let last_letter = starting.last().unwrap();

    for i in starting.windows(2) {
        let key = (i[0], i[1]);
        let entry = counts.entry(key).or_insert(0);
        *entry += 1;
    }

    for _ in 0..steps {
        // debug_counts("in:", &counts);
        let mut next_counts: Counts = counts.clone();
        for (key, &cnt) in counts.iter() {
            if cnt == 0 {
                continue;
            }
            if let Some(&ch) = transitions.get(key) {
                *next_counts.entry((key.0, key.1)).or_insert(cnt) -= cnt;
                *next_counts.entry((key.0, ch)).or_insert(0) += cnt;
                *next_counts.entry((ch, key.1)).or_insert(0) += cnt;
            };
        }
        // debug_counts("out:", &next_counts);
        counts = next_counts;
    }

    let mut letter_counts: HashMap<char, usize> = HashMap::new();
    for (key, val) in counts.iter() {
        *letter_counts.entry(key.0).or_insert(0) += val;
    }
    *letter_counts.entry(*last_letter).or_insert(0) += 1;

    let mut r = letter_counts.iter().map(|x| *x.1).collect::<Vec<usize>>();
    r.sort_unstable();

    r.last().unwrap() - r.first().unwrap()
}

fn solve_part1(input: &(Vec<char>, Transitions)) -> Res {
    solve(10, input)
}

fn solve_part2(input: &(Vec<char>, Transitions)) -> Res {
    solve(40, input)
}

generate_main!();

generate_tests!(1588, 2188189693529);
