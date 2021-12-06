use std::fs::read_to_string;

#[allow(dead_code)]
pub fn string_from_input(day: u32) -> String {
    let path = format!("inputs/{:02}.txt", day);
    read_to_string(path).unwrap()
}

#[allow(dead_code)]
pub fn string_from_sample(day: u32) -> String {
    let path = format!("inputs/{:02}-sample.txt", day);
    read_to_string(path).unwrap()
}

#[macro_export]
macro_rules! print_timed_result {
    ( $prefix:literal, $expression:expr ) => {
        let start = std::time::Instant::now();
        let result = $expression;
        let duration = start.elapsed();
        println!("{}: {:?} ({:?})", $prefix, result, duration);
    };
}

#[macro_export]
macro_rules! generate_main_input {
    ( $day:literal, $is_sample:expr ) => {
        fn main() {
            let data_str = if $is_sample {
                utils::string_from_sample($day)
            } else {
                utils::string_from_input($day)
            };
            let data = parse_input(&data_str);
            print_timed_result!("part 1", solve_part1(&data));
            print_timed_result!("part 2", solve_part2(&data));
        }
    };
}

#[macro_export]
macro_rules! generate_main {
    ( $day:literal ) => {
        generate_main_input!($day, false);
    };
}

#[macro_export]
macro_rules! generate_main_sample {
    ( $day:literal ) => {
        generate_main_input!($day, true);
    };
}

#[macro_export]
macro_rules! generate_tests {
    ( $day:literal, $part1_result:expr, $part2_result:expr ) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn part1_test() {
                let data_str = utils::string_from_sample($day);
                let data = parse_input(&data_str);
                assert_eq!(solve_part1(&data), $part1_result);
            }

            #[test]
            fn part2_test() {
                let data_str = utils::string_from_sample($day);
                let data = parse_input(&data_str);
                assert_eq!(solve_part2(&data), $part2_result);
            }
        }
    };
}
