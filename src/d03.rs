mod utils;
const DAY_ID: utils::DayIdType = 3;

type DataSlice<'a> = &'a [String];

fn parse_input(data: &str) -> Vec<String> {
    data.lines().map(|x| x.into()).collect()
}

fn solve_part1(data: DataSlice) -> u32 {
    let bit_count = data[0].len();

    let mut sums_of_bits = vec![0u32; bit_count];
    for s in data {
        s.chars()
            .enumerate()
            .filter(|(_, ch)| *ch == '1')
            .for_each(|(i, _)| {
                sums_of_bits[i] += 1;
            });
    }

    let gamma_str = sums_of_bits
        .iter()
        .map(|x| {
            if *x as f32 >= data.len() as f32 / 2.0 {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>();

    let epsilon_str = gamma_str
        .chars()
        .map(|x| match x {
            '0' => '1',
            '1' => '0',
            _ => unreachable!(),
        })
        .collect::<String>();

    let gamma_rate = isize::from_str_radix(gamma_str.as_str(), 2).unwrap() as u32;
    let epsilon_rate = isize::from_str_radix(epsilon_str.as_str(), 2).unwrap() as u32;

    gamma_rate * epsilon_rate
}

fn most_frequent_bit(data: DataSlice, bit_index: usize, negated: bool) -> char {
    let ones = data
        .iter()
        .filter(|&x| x.chars().nth(bit_index).unwrap() == '1')
        .count();

    let mut is_msb = ones as f32 >= ((data.len() as f32) / 2.0);

    if negated {
        is_msb = !is_msb;
    }

    if is_msb {
        '1'
    } else {
        '0'
    }
}

fn reduce_data(data: DataSlice, by_most_frequent_bit: bool) -> String {
    let bits = data[0].len();
    let mut reduced_data: Vec<String> = data.to_vec();
    for bit_index in 0..bits {
        if reduced_data.len() == 1 {
            break;
        }
        let mfb = most_frequent_bit(&reduced_data, bit_index, !by_most_frequent_bit);
        reduced_data = reduced_data
            .iter()
            .filter(|&x| x.chars().nth(bit_index).unwrap() == mfb)
            .cloned()
            .collect();
    }
    reduced_data[0].clone()
}

fn solve_part2(data: DataSlice) -> u32 {
    let oxygen_str = reduce_data(data, true);
    let oxygen = isize::from_str_radix(&oxygen_str, 2).unwrap() as u32;

    let co2_str = reduce_data(data, false);
    let co2: u32 = isize::from_str_radix(&co2_str, 2).unwrap() as u32;

    let lis1 = [0000, 1111, 2222];
    println!("{:04}", lis1[0]);

    oxygen * co2
}

generate_main!();

generate_tests!(198, 230);
