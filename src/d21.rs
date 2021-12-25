mod utils;
const DAY_ID: utils::DayIdType = 21;

type Res = usize;

fn position_from_line(line: &str) -> usize {
    let num_str = &line[28..];
    num_str.parse::<usize>().unwrap()
}

fn parse_input(data: &str) -> (usize, usize) {
    let mut lines = data.lines();
    let pos1 = position_from_line(lines.next().unwrap());
    let pos2 = position_from_line(lines.next().unwrap());
    (pos1, pos2)
}

#[derive(Debug)]
struct GrowingFaceDie {
    faces: usize,
    state: usize,
}

impl GrowingFaceDie {
    pub fn new(faces: usize) -> Self {
        Self { faces, state: 1 }
    }

    pub fn roll(&mut self) -> usize {
        let r = self.state;
        if self.state == self.faces {
            self.state = 1;
        } else {
            self.state += 1;
        }
        r
    }
}

#[derive(Debug, Clone)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    pub fn new(pos: usize) -> Self {
        Self {
            position: pos,
            score: 0,
        }
    }

    pub fn advance(&mut self, step: usize) -> usize {
        self.position += step;
        while self.position > 10 {
            self.position -= 10;
        }
        self.position
    }

    pub fn update_score(&mut self) {
        self.score += self.position;
    }
}

fn solve_part1(&(pos1, pos2): &(usize, usize)) -> Res {
    let end_score = 1000;
    let mut p1 = Player::new(pos1);
    let mut p2 = Player::new(pos2);
    let mut die = GrowingFaceDie::new(100);

    let mut rolls_taken = 0;

    let loser_score = loop {
        for _ in 0..3 {
            p1.advance(die.roll());
            rolls_taken += 1;
        }
        p1.update_score();

        if p1.score >= end_score {
            break p2.score;
        }

        for _ in 0..3 {
            p2.advance(die.roll());
            rolls_taken += 1;
        }
        p2.update_score();

        if p2.score >= end_score {
            break p1.score;
        }
    };

    loser_score * rolls_taken
}

// TODO - expensive to repeatedly compute, make static/singleton or global...
// fn three_rolls_advance() -> HashMap<usize, usize> {
//     let mut sum_counts = HashMap::new();
//     let faces = [1, 2, 3];
//     for x in faces {
//         for y in faces {
//             for z in faces {
//                 *sum_counts.entry(x + y + z).or_insert(0) += 1;
//             }
//         }
//     }
//     sum_counts
// }

const THREES: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn play_dirac_step(
    cur_pos: usize,
    cur_score: usize,
    is_curr_p1: bool,
    oth_pos: usize,
    oth_score: usize,
    multiplier: usize,
    winnings: &mut (usize, usize),
) {
    const END_SCORE: usize = 21;

    for (sum_step, variant_cnt) in THREES {
        let mut pos = cur_pos + sum_step;
        while pos > 10 {
            pos -= 10;
        }

        if cur_score + pos >= END_SCORE {
            if is_curr_p1 {
                winnings.0 += variant_cnt * multiplier;
            } else {
                winnings.1 += variant_cnt * multiplier;
            }
        } else {
            play_dirac_step(
                oth_pos,
                oth_score,
                !is_curr_p1,
                pos,
                cur_score + pos,
                variant_cnt * multiplier,
                winnings,
            );
        }
    }
}

fn solve_part2(&(pos1, pos2): &(usize, usize)) -> Res {
    let mut winnings = (0, 0);
    play_dirac_step(pos1, 0, true, pos2, 0, 1, &mut winnings);
    winnings.0.max(winnings.1)
}

generate_main!();

generate_tests!(739785, 444356092776315);
