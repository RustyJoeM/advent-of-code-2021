mod utils;
const DAY_ID: utils::DayIdType = 18;

type Num = u8;
type Res = u32;

#[derive(Debug, Clone)]
enum Node {
    Number(Num),
    Pair(Box<Node>, Box<Node>),
}

fn parse_input(data: &str) -> Vec<Node> {
    data.lines().map(|line| line.into()).collect()
}

fn parse_node(s: &mut &str) -> Node {
    if &s[0..1] == "[" {
        *s = &s[1..]; // skip "["
        let left = parse_node(s);
        *s = &s[1..]; // skip ","
        let right = parse_node(s);
        *s = &s[1..]; // skip "]"
        Node::Pair(Box::new(left), Box::new(right))
    } else {
        // all input numbers appear to be single digit only!
        let num = s.chars().next().unwrap() as u8 - b'0';
        *s = &s[1..];
        Node::Number(num)
    }
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        let mut parsed = s;
        parse_node(&mut parsed)
    }
}

impl Node {
    pub fn val(&self) -> Num {
        match self {
            Node::Number(n) => *n,
            Node::Pair(_, _) => unreachable!(),
        }
    }

    pub fn magnitude(&self) -> Res {
        match self {
            Node::Number(n) => *n as Res,
            Node::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    #[allow(dead_code)]
    pub fn debug_string(&self) -> String {
        match self {
            Node::Number(n) => n.to_string(),
            Node::Pair(left, right) => {
                format!("[{},{}]", left.debug_string(), right.debug_string())
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Node::Number(n) if *n >= 10 => {
                let f = *n as f32 / 2.0;
                let left = Box::new(Node::Number(f.floor() as Num));
                let right = Box::new(Node::Number(f.ceil() as Num));
                *self = Node::Pair(left, right);
                return true;
            }
            Node::Pair(left, right) => {
                if left.split() {
                    return true;
                }
                if right.split() {
                    return true;
                }
            }
            _ => {}
        }
        false
    }

    pub fn reduce(&mut self) {
        loop {
            let mut blast = None;
            let mut affected_number = None;
            explode_node(self, 0, &mut affected_number, &mut blast);
            if blast.is_some() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }
}

fn sum_and_reduce(left: &Node, right: &Node) -> Node {
    let mut sum = Node::Pair(Box::new(left.clone()), Box::new(right.clone()));
    sum.reduce();
    sum
}

fn explode_node<'a>(
    node: &'a mut Node,
    depth: usize,
    affected_number: &mut Option<&'a mut Num>,
    blast: &mut Option<Num>,
) -> bool {
    match node {
        Node::Number(n) => {
            if let Some(b) = blast {
                *n += *b;
                return true;
            } else {
                *affected_number = Some(n)
            }
        }
        Node::Pair(left, right) if depth > 3 && blast.is_none() => {
            if let Some(affected) = affected_number {
                **affected += left.val();
            }
            *blast = Some(right.val());
            *node = Node::Number(0);
        }
        Node::Pair(left, right) => {
            if explode_node(left, depth + 1, affected_number, blast) {
                return true;
            }
            if explode_node(right, depth + 1, affected_number, blast) {
                return true;
            }
        }
    }
    false
}

fn solve_part1(nodes: &[Node]) -> Res {
    let mut sum_node = nodes[0].clone();
    for node in nodes.iter().skip(1) {
        sum_node = sum_and_reduce(&sum_node, node);
    }
    sum_node.magnitude()
}

fn solve_part2(nodes: &[Node]) -> Res {
    let mut max_magnitude = 0;

    let len = nodes.len();
    for i in 0..len {
        for j in 0..len {
            if i == j {
                continue;
            }
            let mut sum_node = nodes[i].clone();
            sum_node = sum_and_reduce(&sum_node, &nodes[j]);
            max_magnitude = sum_node.magnitude().max(max_magnitude);
        }
    }

    max_magnitude
}

generate_main!();

generate_tests!(4140, 3993);
