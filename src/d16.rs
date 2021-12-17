use std::cmp::Ordering;

mod utils;
const DAY_ID: utils::DayIdType = 16;

type Res = usize;

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    number: Option<usize>,
    kids: Vec<Packet>,
}

impl Packet {
    pub fn version_sum(&self) -> usize {
        let vv: usize = self.kids.iter().map(|ch| ch.version_sum()).sum();
        self.version + vv
    }

    fn twins_cmp(&self, op: Ordering) -> usize {
        let a = self.kids[0].value();
        let b = self.kids[1].value();
        let res = match op {
            Ordering::Equal => a == b,
            Ordering::Greater => a > b,
            Ordering::Less => a < b,
        };
        res as usize
    }

    fn kids_values(&self) -> impl Iterator<Item = usize> + '_ {
        self.kids.iter().map(|x| x.value())
    }

    pub fn value(&self) -> usize {
        match self.type_id {
            0 => self.kids_values().sum(),
            1 => self.kids_values().product(),
            2 => self.kids_values().min().unwrap(),
            3 => self.kids_values().max().unwrap(),
            4 => self.number.unwrap(),
            5 => self.twins_cmp(Ordering::Greater),
            6 => self.twins_cmp(Ordering::Less),
            7 => self.twins_cmp(Ordering::Equal),
            _ => unreachable!(),
        }
    }
}

fn can_continue(pos: usize, pos_limit: usize, kid_cnt: usize, kid_limit: Option<usize>) -> bool {
    let pos_ok = pos < pos_limit;
    match kid_limit {
        Some(max_kids) => pos_ok && kid_cnt < max_kids,
        None => pos_ok,
    }
}

fn parse_literal_body(version: usize, type_id: usize, slice: &str) -> (Packet, usize) {
    let mut val: usize = 0;
    let mut pos = 0;
    while pos < slice.len() {
        let chunk = &slice[pos..pos + 5];
        val *= 16;
        val += usize::from_str_radix(&chunk[1..], 2).unwrap();
        pos += 5;
        if &chunk[0..1] == "0" {
            break;
        }
    }
    let packet = Packet {
        version,
        type_id,
        number: Some(val),
        kids: vec![],
    };
    (packet, pos)
}

fn parse_packet(slice: &str) -> (Packet, usize) {
    let mut pos = 0;

    let version = usize::from_str_radix(&slice[pos..pos + 3], 2).unwrap();
    pos += 3;

    let type_id = usize::from_str_radix(&slice[pos..pos + 3], 2).unwrap();
    pos += 3;

    match type_id {
        4 => {
            let (packet, non_header_eaten) = parse_literal_body(version, type_id, &slice[pos..]);
            (packet, non_header_eaten + pos)
        }
        _ => {
            let length_type_id = &slice[pos..pos + 1];
            pos += 1;
            let char_cnt = if length_type_id == "0" { 15 } else { 11 };
            let limit_value = usize::from_str_radix(&slice[pos..pos + char_cnt], 2).unwrap();
            pos += char_cnt;

            let mut kids = vec![];
            let mut max_kids = None;
            let mut max_offset = slice.len();

            if length_type_id == "0" {
                max_offset = pos + limit_value;
            } else {
                max_kids = Some(limit_value);
            };

            while can_continue(pos, max_offset, kids.len(), max_kids) {
                let (packet, packet_len) = parse_packet(&slice[pos..max_offset]);
                pos += packet_len;
                kids.push(packet);
            }

            let packet = Packet {
                version,
                type_id,
                number: None,
                kids,
            };

            (packet, pos)
        }
    }
}

fn parse_input(data: &str) -> &str {
    data.lines().next().unwrap()
}

fn parse_root_packet(data: &str) -> Packet {
    let binary_str: String = data.chars().map(to_binary).collect();
    let slice: &str = &binary_str;
    let (packet, _) = parse_packet(&slice[0..slice.len()]);
    packet
}

fn solve_part1(data: &str) -> Res {
    let packet = parse_root_packet(data);
    packet.version_sum()
}

fn solve_part2(data: &str) -> Res {
    let packet = parse_root_packet(data);
    packet.value()
}

generate_main!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_part1() {
        for (data, result) in [
            ("D2FE28", 6),
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ] {
            let packet = parse_root_packet(data);
            assert_eq!(packet.version_sum(), result);
        }
    }

    #[test]
    fn tests_part2() {
        for (data, result) in [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ] {
            let packet = parse_root_packet(data);
            assert_eq!(packet.value(), result);
        }
    }
}
