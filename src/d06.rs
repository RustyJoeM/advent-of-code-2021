mod utils;
const DAY_ID: utils::DayIdType = 6;

type Num = u8;
type Res = u64;

fn parse_input(data: &str) -> Vec<Num> {
    data.split(',').map(|x| x.parse::<Num>().unwrap()).collect()
}

fn breed_fish(data: &[Num], days: usize) -> Res {
    let mut counts: [u64; 9] = [0; 9];
    for &days_to_birth in data {
        counts[days_to_birth as usize] += 1;
    }

    for _ in 0..days {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }

    counts.iter().sum()
}

fn solve_part1(data: &[Num]) -> Res {
    breed_fish(data, 80)
}

fn solve_part2(data: &[Num]) -> Res {
    breed_fish(data, 256)
}

generate_main!();

generate_tests!(5934, 26984457539);
