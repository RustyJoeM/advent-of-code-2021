mod utils;
const DAY_ID: utils::DayIdType = 10;

type Res = u64;

fn parse_input(data: &str) -> Vec<&str> {
    data.lines().collect()
}

// fn is_incomplete(line: &str) -> bool {
//     let openings = line.chars().filter(|&x| "<[({".contains(x)).count();
//     let closings = line.chars().filter(|&x| ">])}".contains(x)).count();
//     openings != closings
// }

fn is_opener(ch: char) -> bool {
    "([{<".contains(ch)
}

fn is_closer(ch: char) -> bool {
    ")]}>".contains(ch)
}

fn first_illegal(line: &str) -> Option<(usize, char)> {
    let mut stack = vec![];
    for (i, ch) in line.chars().enumerate() {
        if is_opener(ch) {
            stack.push(ch);
        } else if is_closer(ch) {
            let opener = stack.pop().unwrap();
            if !does_match(opener, ch) {
                return Some((i, ch));
            }
        }
    }
    None
}

fn does_match(o: char, c: char) -> bool {
    c == match o {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => {
            return false;
        }
    }
}

fn solve_part1(data: &[&str]) -> Res {
    let mut failed: Vec<char> = vec![];
    for line in data.iter() {
        if let Some(ch) = first_illegal(line) {
            failed.push(ch.1);
        };
    }

    failed
        .iter()
        .map(|x| match x {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}

fn get_suffix(line: &str) -> Vec<char> {
    let mut stack = vec![];
    for ch in line.chars() {
        if is_opener(ch) {
            stack.push(ch);
        } else if is_closer(ch) {
            stack.pop();
        }
    }
    stack
        .iter()
        .map(|x| match x {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => unreachable!(),
        })
        .rev()
        .collect()
}

fn score(missing_suffix: &[char]) -> u64 {
    let mut score = 0;
    for ch in missing_suffix.iter() {
        score = 5 * score
            + match ch {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            };
    }
    score
}

fn solve_part2(data: &[&str]) -> Res {
    let mut scores = vec![];
    for line in data.iter() {
        if first_illegal(line).is_some() {
            continue;
        };
        let missing_suffix = get_suffix(line);
        scores.push(score(&missing_suffix));
    }

    scores.sort_unstable();

    let index = scores.len() / 2;
    scores[index]
}

generate_main!();

generate_tests!(26397, 288957);
