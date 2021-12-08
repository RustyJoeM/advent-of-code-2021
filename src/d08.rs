mod utils;
const DAY_ID: utils::DayIdType = 8;

type Res = u32;

#[derive(Debug, Clone)]
struct DigitEntry {
    input: Vec<Vec<char>>,
    output: Vec<Vec<char>>,
}

impl From<&str> for DigitEntry {
    fn from(line: &str) -> Self {
        let groups: Vec<&str> = line.split('|').collect();

        let input = groups[0]
            .split_whitespace()
            .map(|x| x.chars().collect())
            .collect();

        let output = groups[1]
            .split_whitespace()
            .map(|x| x.chars().collect())
            .collect();

        Self { input, output }
    }
}

fn vec_includes_all(container: &[char], seed: &[char]) -> bool {
    seed.iter().all(|x| container.contains(x))
}

impl DigitEntry {
    pub fn output_value(&self) -> u32 {
        let v = &self.output;
        self.field_to_num(&v[0], false).unwrap() * 1000
            + self.field_to_num(&v[1], false).unwrap() * 100
            + self.field_to_num(&v[2], false).unwrap() * 10
            + self.field_to_num(&v[3], false).unwrap()
    }

    fn chars_of_digit_1(&self) -> Vec<char> {
        self.all_fields()
            .find(|&predicate| predicate.len() == 2)
            .unwrap()
            .clone()
    }

    fn chars_of_digit_4(&self) -> Vec<char> {
        self.all_fields()
            .find(|&predicate| predicate.len() == 4)
            .unwrap()
            .clone()
    }

    fn chars_of_digit_9(&self) -> Vec<char> {
        self.all_fields()
            .find(|&predicate| self.field_to_num(predicate, true) == Some(9))
            .unwrap()
            .clone()
    }

    fn all_fields(&self) -> impl Iterator<Item = &Vec<char>> {
        let input_iter = self.input.iter();
        let output_iter = self.output.iter();
        let chain = input_iter.chain(output_iter);
        chain
    }

    fn field_to_num(&self, field: &[char], nested: bool) -> Option<u32> {
        match field.len() {
            2 => return Some(1),
            3 => return Some(7),
            4 => return Some(4),
            7 => return Some(8),
            5 => {
                // 2 | 3 | 5
                if nested {
                    return None;
                }
                if vec_includes_all(field, &self.chars_of_digit_1()) {
                    return Some(3);
                }
                let nine_bytes = self.chars_of_digit_9();
                for ch in field.iter() {
                    if !nine_bytes.contains(ch) {
                        return Some(2);
                    }
                }
                return Some(5);
            }
            6 => {
                // 0 | 6 | 9
                if vec_includes_all(field, &self.chars_of_digit_1()) {
                    if vec_includes_all(field, &self.chars_of_digit_4()) {
                        return Some(9);
                    } else {
                        return Some(0);
                    }
                } else {
                    return Some(6);
                }
            }
            _ => {}
        }

        None
    }

    // fn debug(&self) -> String {
    //     let mut o = String::new();
    //     for input_index in 0..self.input.len() {
    //         let substr = match self.chars_to_num(input_index, true, false) {
    //             Some(x) => format!(" | {:8}", x),
    //             None => {
    //                 let s: String = self.input[input_index].iter().collect::<String>();
    //                 format!(" | {:8}", s)
    //             },
    //         };
    //         o += &substr;
    //     }
    //     o += " | --";
    //     for output_index in 0..4 {
    //         let substr = match self.chars_to_num(output_index, false, false) {
    //             Some(x) => format!(" | {:8}", x),
    //             None => {
    //                 let s: String = self.output[output_index].iter().collect::<String>();
    //                 format!(" | {:8}", s)
    //             },
    //         };
    //         o += &substr;
    //     }
    //     o += " |";
    //     o
    // }
}

fn parse_input(data: &str) -> Vec<DigitEntry> {
    data.lines().map(|x| x.into()).collect()
}

fn solve_part1(data: &[DigitEntry]) -> usize {
    let mut cnt = 0;

    for e in data.iter() {
        cnt += e
            .output
            .iter()
            .filter(|x| [2usize, 3usize, 4usize, 7usize].contains(&x.len()))
            .count();
    }
    cnt
}

fn solve_part2(data: &[DigitEntry]) -> Res {
    data.iter().map(|x| x.output_value()).sum()
}

generate_main!();

generate_tests!(26, 61229);
