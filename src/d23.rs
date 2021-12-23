mod utils;
const DAY_ID: utils::DayIdType = 23;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    pub fn step_cost(&self) -> usize {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }
}

#[derive(Debug, Clone)]
struct Room {
    target: Amphipod,
    tenants: Vec<Option<Amphipod>>,
    hallway_index: usize,
}

impl Room {
    pub fn new(target: Amphipod, tenants: Vec<Option<Amphipod>>, hallway_index: usize) -> Self {
        Self {
            target,
            tenants,
            hallway_index,
        }
    }

    pub fn is_finished(&self) -> bool {
        let is_full = !self.tenants.iter().any(|t| t.is_none());
        if !is_full {
            return false;
        }
        for t in self.tenants.iter() {
            if t.unwrap() != self.target {
                return false;
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.tenants.iter().all(Option::is_none)
    }

    pub fn has_targets_only(&self) -> bool {
        self.tenants
            .iter()
            .all(|t| t.is_none() || t.unwrap() == self.target)
    }

    pub fn depth_of_park(&self, pod: &Amphipod) -> Option<usize> {
        // already full of some/any tenants
        if self.tenants.iter().all(|t| t.is_some()) {
            return None;
        }
        // disallow non-target pods
        if self.target != *pod {
            return None;
        }
        // non-target pods still inside
        if self
            .tenants
            .iter()
            .filter(|a| a.is_some())
            .any(|a| a.unwrap() != *pod)
        {
            return None;
        }
        // number of remaining None's is the depth to land in
        Some(self.tenants.iter().filter(|p| p.is_none()).count())
    }

    pub fn park(&mut self, pod: &Amphipod) {
        for i in 0..self.tenants.len() {
            if self.tenants[i].is_none() {
                self.tenants[i] = Some(*pod);
                return;
            }
        }
    }

    pub fn peek_top(&self) -> Option<Amphipod> {
        if let Some(p) = self.tenants.iter().rev().flatten().next() {
            return Some(*p);
        }
        None
    }

    pub fn pop_top(&mut self) -> usize {
        // dbg!("---- before", &self);
        for i in (0..=(self.tenants.len() - 1)).rev() {
            // dbg!(i, &self.tenants[i]);
            if self.tenants[i].is_some() {
                self.tenants[i] = None;
                // dbg!("---- after", &self);
                return self.tenants.len() - i - 1;
            }
        }
        unreachable!()
    }
}

fn parse_input(data: &str) -> Vec<String> {
    data.lines().map(|x| x.into()).collect()
}

#[derive(Debug, Clone)]
struct AmphiState {
    hallway: Vec<Option<Amphipod>>,
    rooms: Vec<Room>,
    score: usize,
}

impl AmphiState {
    pub fn new(
        hallway_len: usize,
        init_rooms: &[(usize, Amphipod, Vec<Option<Amphipod>>)],
    ) -> Self {
        let rooms = init_rooms
            .iter()
            .map(|(hallway_index, target, tenants)| {
                Room::new(*target, tenants.to_owned(), *hallway_index)
            })
            .collect();
        Self {
            hallway: vec![None; hallway_len],
            rooms,
            score: 0,
        }
    }

    fn passable_distance(&self, from: usize, to: usize) -> Option<usize> {
        // dbg!(&self.hallway, from, to);
        if from < to && self.hallway[from+1..=to].iter().all(Option::is_none) {
            // dbg!("<<<<", to-from);
            return Some(to-from);
        } else if from > to && self.hallway[to..=from-1].iter().all(Option::is_none) {
            // dbg!(">>>>", from-to);
            return Some(from-to);
        }
        None
        // let min = from.min(to);
        // let max = from.max(to);
        // dbg!("-----", from, to, min, max);
        // if self.hallway[min + 1..=max].iter().all(Option::is_none) {
        //     dbg!(max-min);
        //     Some(max - min)
        // } else {
        //     None
        // }
    }

    pub fn next_states(&self) -> Vec<AmphiState> {
        let mut next_states = vec![];

        // try all transitions - from hallway to target room
        for i in 0..self.hallway.len() {
            if let Some(pod) = &self.hallway[i] {
                let target_room = self.rooms.iter().find(|r| r.target == *pod).unwrap();
                let ti = target_room.hallway_index;
                if let Some(hallway_distance) = self.passable_distance(i, ti) {
                    if let Some(room_depth) = target_room.depth_of_park(pod) {
                        let mut new = self.clone();
                        new.hallway[i] = None;
                        new.rooms
                            .iter_mut()
                            .find(|r| r.target == *pod)
                            .unwrap()
                            .park(pod);
                        new.score += (hallway_distance + room_depth) * pod.step_cost();
                        next_states.push(new);
                    }
                }
            }
        }

        // if !next_states.is_empty() {
        //     dbg!(&self);
        //     dbg!(next_states.len());
        //     dbg!(&next_states);
        //     panic!();
        // }


        // try all transitions - from room to hallway
        for room in self.rooms.iter() {
            if room.is_finished() || room.is_empty() || room.has_targets_only() {
                continue;
            }
            // TODO - remove magic constants...
            for hi in [0, 1, 3, 5, 7, 9, 10] {
                // spot already taken
                if self.hallway[hi].is_some() {
                    continue;
                }
                if let Some(hallway_distance) = self.passable_distance(room.hallway_index, hi) {
                    if let Some(top_pod) = room.peek_top() {
                        let mut new = self.clone();
                        new.hallway[hi] = Some(top_pod);
                        let new_room = new
                            .rooms
                            .iter_mut()
                            .find(|r| r.hallway_index == room.hallway_index)
                            .unwrap();
                        let depth = new_room.pop_top();
                        // don't forget +1 from room entry to hallway
                        new.score += (hallway_distance + 1 + depth) * top_pod.step_cost();
                        next_states.push(new);
                    }
                }
            }
        }

        // dbg!(next_states.len());
        // dbg!(&next_states);
        // panic!();

        // for s in next_states.iter() {
        //     dbg!(s.score);
        // }
        // panic!();

        // TODO sort by score?
        next_states.sort_by_key(|x| x.score);

        next_states
    }
}

fn progress_state(state: AmphiState, min_score: &mut usize) {
    // dbg!(&state);

    if state.score >= *min_score {
        // dbg!("got higher score");
        return;
    }

    if state.rooms.iter().all(Room::is_finished) {
        // if state.score < *min_score {
            // dbg!("---- yay, got winning state");
            // dbg!(&state);
        // }
        *min_score = state.score.min(*min_score);
        return;
    }

    let next_states = state.next_states();
    for next_state in next_states {
        progress_state(next_state, min_score);
    }
}

fn solve_part1(_data: &[String]) -> usize {
    // TODO - parse input instead of hardcode
    let hallway_len = 11;
    let init_rooms = [
        (2, Amphipod::A, vec![Some(Amphipod::C), Some(Amphipod::B)]),
        (4, Amphipod::B, vec![Some(Amphipod::D), Some(Amphipod::A)]),
        (6, Amphipod::C, vec![Some(Amphipod::D), Some(Amphipod::B)]),
        (8, Amphipod::D, vec![Some(Amphipod::A), Some(Amphipod::C)]),
    ];

    // ###B#C#B#D###         SAMPLE
    //   #A#D#C#A#
    // let init_rooms = [
    //     (2, Amphipod::A, vec![Some(Amphipod::A), Some(Amphipod::B)]),
    //     (4, Amphipod::B, vec![Some(Amphipod::D), Some(Amphipod::C)]),
    //     (6, Amphipod::C, vec![Some(Amphipod::C), Some(Amphipod::B)]),
    //     (8, Amphipod::D, vec![Some(Amphipod::A), Some(Amphipod::D)]),
    // ];

    let init_state = AmphiState::new(hallway_len, &init_rooms);

    let mut min_score = usize::MAX;
    progress_state(init_state, &mut min_score);
    assert!(min_score != usize::MAX);

    min_score
}

fn solve_part2(_data: &[String]) -> usize {
    let hallway_len = 11;
    let init_rooms = [
        (2, Amphipod::A, vec![Some(Amphipod::C), Some(Amphipod::D), Some(Amphipod::D), Some(Amphipod::B)]),
        (4, Amphipod::B, vec![Some(Amphipod::D), Some(Amphipod::B), Some(Amphipod::C), Some(Amphipod::A)]),
        (6, Amphipod::C, vec![Some(Amphipod::D), Some(Amphipod::A), Some(Amphipod::B), Some(Amphipod::B)]),
        (8, Amphipod::D, vec![Some(Amphipod::A), Some(Amphipod::C), Some(Amphipod::A), Some(Amphipod::C)]),
    ];
    // ###B#A#B#C###
    //   #D#C#B#A#
    //   #D#B#A#C#
    //   #C#D#D#A#

    // ###B#C#B#D###         SAMPLE
    //   #A#D#C#A#
    // let init_rooms = [
    //     (2, Amphipod::A, vec![Some(Amphipod::A), Some(Amphipod::D), Some(Amphipod::D), Some(Amphipod::B)]),
    //     (4, Amphipod::B, vec![Some(Amphipod::D), Some(Amphipod::B), Some(Amphipod::C), Some(Amphipod::C)]),
    //     (6, Amphipod::C, vec![Some(Amphipod::C), Some(Amphipod::A), Some(Amphipod::B), Some(Amphipod::B)]),
    //     (8, Amphipod::D, vec![Some(Amphipod::A), Some(Amphipod::C), Some(Amphipod::A), Some(Amphipod::D)]),
    // ];

    // ##B#C#B#D###
    //  #D#C#B#A#
    //  #D#B#A#C#
    //  #A#D#C#A#

    let init_state = AmphiState::new(hallway_len, &init_rooms);

    let mut min_score = usize::MAX;
    progress_state(init_state, &mut min_score);
    assert!(min_score != usize::MAX);

    min_score}

generate_main_sample!();

generate_tests!(12521, 44169);
