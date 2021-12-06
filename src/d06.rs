mod utils;

type Num = u8;
type Res = u64;

fn parse_input(data: &str) -> Vec<Num> {
    data.split(',').map(|x| x.parse::<Num>().unwrap()).collect()
}

fn breed_fish(data: &[Num], days: usize) -> Res {
    let mut count_by_internal: Vec<Res> = vec![0; 9];
    for &fish in data {
        count_by_internal[fish as usize] += 1;
    }

    for _ in 0..days {
        count_by_internal.rotate_left(1);
        count_by_internal[6] += count_by_internal[8];
    }

    count_by_internal.iter().sum()
}

fn solve_part1(data: &[Num]) -> Res {
    breed_fish(data, 80)
}

fn solve_part2(data: &[Num]) -> Res {
    breed_fish(data, 256)
}

generate_main!(6);

generate_tests!(6, 5934, 26984457539);
