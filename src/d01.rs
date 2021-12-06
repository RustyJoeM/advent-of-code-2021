mod utils;

type Num = u32;

fn parse_input(data: &str) -> Vec<Num> {
    data.lines().map(|x| x.parse::<Num>().unwrap()).collect()
}

fn solve_part1(numbers: &[Num]) -> usize {
    numbers.windows(2).filter(|&x| x[0] < x[1]).count()
}

fn solve_part2(numbers: &[Num]) -> usize {
    let window_size = 3;
    let window_sums = numbers
        .windows(window_size)
        .map(|x| x.iter().sum())
        .collect::<Vec<Num>>();
    solve_part1(&window_sums)
}

generate_main!(1);

generate_tests!(1, 7, 5);
