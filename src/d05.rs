use std::collections::HashMap;

mod utils;

type Num = i32;
type Line = ((Num, Num), (Num, Num));

fn parse_input(data: &str) -> Vec<Line> {
    data.lines()
        .map(|x| {
            let coords = x.split(" -> ").collect::<Vec<&str>>();
            let a = coords[0]
                .split(',')
                .map(|x| x.parse::<Num>().unwrap())
                .collect::<Vec<Num>>();
            let b = coords[1]
                .split(',')
                .map(|x| x.parse::<Num>().unwrap())
                .collect::<Vec<Num>>();
            ((a[0], a[1]), (b[0], b[1]))
        })
        .collect()
}

type Field = HashMap<(Num, Num), usize>;

#[allow(dead_code)]
fn debug_field(field: &Field, size: usize) {
    for row in 0..size {
        for col in 0..size {
            if let Some(r) = field.get(&(col as Num, row as Num)) {
                print!(" {:02}", r);
            } else {
                print!("  -");
            }
        }
        println!();
    }
}

fn solve(data: &[Line], skip_diagonals: bool) -> usize {
    let mut field: Field = Default::default();

    for (a, b) in data {
        let &(x0, y0) = a;
        let &(x1, y1) = b;

        let dx = match x1 - x0 {
            0 => 0,
            x if x > 0 => 1,
            _ => -1,
        };

        let dy = match y1 - y0 {
            0 => 0,
            y if y > 0 => 1,
            _ => -1,
        };

        if skip_diagonals && dx != 0 && dy != 0 {
            continue;
        }

        let steps = (x1 - x0).abs().max((y1 - y0).abs()) + 1;
        for s in 0..steps {
            let key = (x0 + s * dx, y0 + s * dy);
            let entry = field.entry(key).or_insert(0);
            *entry += 1;
        }
    }

    field.iter().filter(|(_, v)| **v >= 2).count()
}

fn solve_part1(data: &[Line]) -> usize {
    solve(data, true)
}

fn solve_part2(data: &[Line]) -> usize {
    solve(data, false)
}

generate_main!(5);

generate_tests!(5, 5, 12);
