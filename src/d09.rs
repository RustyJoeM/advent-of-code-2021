use std::collections::HashSet;

mod utils;
const DAY_ID: utils::DayIdType = 9;

type Res = u32;

type Map = Vec<Vec<u8>>;

fn parse_input(data: &str) -> Map {
    data.lines().map(|row| {
        row.chars().map(|x| {
            x as u8 - '0' as u8
        }).collect()
    }).collect()
}

fn solve_part1(data: &Map) -> Res {
    let mut risk = 0; 

    let max_y = data.len();
    let max_x = data[0].len();

    for r in 0..max_y {
        for c in 0..max_x {
            let me = data[r][c];
            let left = if c > 0 { data[r][c-1] > me } else { true };
            let right = if c < max_x-1 { data[r][c+1] > me } else { true };
            let top = if r > 0 { data[r-1][c] > me } else { true };
            let bottom = if r < max_y-1 { data[r+1][c] > me } else { true };
            if left && right && top && bottom {
                risk += me as u32 + 1;
            }
        }
    }
    risk
}

fn get_basin_score(data: &Map, visited: &mut HashSet<(usize, usize)>, r: usize, c: usize, go_left: bool, go_right: bool, go_up: bool, go_down: bool) -> Res {
    let max_y = data.len();
    let max_x = data[0].len();
    let me = data[r][c];
    // dbg!("----", me, (r, c), go_left, go_right, go_up, go_down);

    let left = if c > 0 { data[r][c-1] > me } else { false };
    let right = if c < max_x-1 { data[r][c+1] > me } else { false };
    let up = if r > 0 { data[r-1][c] > me } else { false };
    let down = if r < max_y-1 { data[r+1][c] > me } else { false };
    // dbg!("", left, right, up, down);

    let mut score = 0;

    if left {
        if go_left && c > 0 && data[r][c-1] < 9 {
            score += get_basin_score(data, visited, r, c-1, true, false, true, true);
        }
    }

    if right {
        if go_right && c < max_x-1 && data[r][c+1] < 9 {
            score += get_basin_score(data, visited, r, c+1, false, true, true, true);
        }
    }

    if up {
        if go_up && r > 0  && data[r-1][c] < 9 {
            score += get_basin_score(data, visited, r-1, c, true, true, true, false);
        }
    }

    if down {
        if go_down && r < max_y-1 && data[r+1][c] < 9 {
            score += get_basin_score(data, visited, r+1, c, true, true, false, true);
        }
    }

    // if left || right || up || down {
        visited.insert((r, c));
    // }

    // dbg!(score);
    score
}

fn solve_part2(data: &Map) -> Res {
    let mut basins: Vec<Res> = vec![]; 

    let max_y = data.len();
    let max_x = data[0].len();

    'outer: for r in 0..max_y {
        for c in 0..max_x {
            let me = data[r][c];
            let left = if c > 0 { data[r][c-1] > me } else { true };
            let right = if c < max_x-1 { data[r][c+1] > me } else { true };
            let top = if r > 0 { data[r-1][c] > me } else { true };
            let bottom = if r < max_y-1 { data[r+1][c] > me } else { true };
            if left && right && top && bottom {
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                // dbg!("------------------------------------------------------------------------------------------------");
                get_basin_score(data, &mut visited, r, c, true, true, true, true);
                basins.push(visited.len() as Res);
                // break 'outer;
            }
        }
    }

    let blen = basins.len();
    // dbg!(&basins);
    basins.sort();
    // dbg!(basins[blen-1], basins[blen-2], basins[blen-3]);
    basins[blen-1] * basins[blen-2] * basins[blen-3]
}

generate_main!();

generate_tests!(15, 1134);
