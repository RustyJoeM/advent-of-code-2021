mod utils;

type Num = u64;

fn parse_input(data: &str) -> Vec<Num> {
    data.split(',').map(|x| x.parse::<Num>().unwrap()).collect()
}

fn diff(x: Num, y: Num) -> Num {
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn diff2(x: Num, y: Num) -> Num {
    let d = diff(x, y);
    d * (d + 1) / 2
}

fn smallest_distance(data: &[Num], distance: &dyn Fn(Num, Num) -> Num) -> Num {
    let &min = data.iter().min().unwrap();
    let &max = data.iter().max().unwrap();

    let mut r = data.len() as Num * distance(min, max);
    for i in min..=max {
        let fuel: Num = data.iter().map(|&x| distance(i, x)).sum();
        r = fuel.min(r);
    }

    r
}

fn solve_part1(data: &[Num]) -> Num {
    smallest_distance(data, &diff)
}

fn solve_part2(data: &[Num]) -> Num {
    smallest_distance(data, &diff2)
}

generate_main!(7);

generate_tests!(7, 37, 168);
