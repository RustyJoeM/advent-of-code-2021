mod utils;

enum Move {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let words = s.split(' ').collect::<Vec<&str>>();
        let cmd = words[0];
        let len = words[1].parse::<u32>().unwrap();
        match cmd {
            "forward" => Move::Forward(len),
            "up" => Move::Up(len),
            "down" => Move::Down(len),
            _ => unreachable!(),
        }
    }
}

fn parse_input(data: &str) -> Vec<Move> {
    data.lines().map(|x| x.into()).collect()
}

fn solve_part1(moves: &[Move]) -> u32 {
    let (mut dist, mut depth) = (0, 0);
    for m in moves {
        match m {
            Move::Forward(x) => {
                dist += x;
            }
            Move::Up(x) => {
                depth -= x;
            }
            Move::Down(x) => {
                depth += x;
            }
        }
    }
    dist * depth
}

fn solve_part2(commands: &[Move]) -> u32 {
    let (mut dist, mut depth, mut aim) = (0, 0, 0);
    for cmd in commands {
        match cmd {
            Move::Forward(x) => {
                dist += x;
                depth += aim * x;
            }
            Move::Up(x) => {
                aim -= x;
            }
            Move::Down(x) => {
                aim += x;
            }
        }
    }
    dist * depth
}

generate_main!(2);

generate_tests!(2, 150, 900);
