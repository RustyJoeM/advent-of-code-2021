use std::collections::{HashMap, HashSet};

mod utils;
const DAY_ID: utils::DayIdType = 12;

type Adjacencies<'a> = HashMap<&'a str, Vec<&'a str>>;
type Path<'a> = String;

fn parse_input(data: &str) -> Adjacencies {
    let mut map: HashMap<&str, Vec<&str>> = Default::default();
    data.lines().for_each(|line| {
        let v: Vec<&str> = line.split('-').collect();
        map.entry(v[0]).or_insert_with(Vec::new).push(v[1]);
        map.entry(v[1]).or_insert_with(Vec::new).push(v[0]);
    });

    map
}

fn path_to_string(path: &[&str]) -> String {
    path.join("-")
}

fn traverse<'a>(
    adjacencies: &'a Adjacencies,
    path: & mut Vec<&'a str>,
    allow_double: bool,
    has_double: bool,
    result: &mut HashSet<String>,
) {
    let current_node = path.iter().last().unwrap();
    let neighbors = adjacencies.get(current_node).unwrap();

    for &node in neighbors.iter() {
        if node == "start" {
            continue;
        }

        let mut my_double = has_double;
        if node.to_lowercase() == node && path.contains(&node) {
            if !allow_double || has_double {
                continue;
            } else {
                my_double = true;
            }
        }

        path.push(node);

        if node == "end" {
            result.insert(path_to_string(path));
        } else {
            traverse(adjacencies, path, allow_double, my_double, result);
        }

        path.pop();
    }
}

fn solve_part1(adjacencies: &Adjacencies) -> usize {
    let mut path: Vec<&str> = vec!["start"];
    let mut results: HashSet<Path> = HashSet::new();

    traverse(adjacencies, &mut path, false, false, &mut results);

    results.len()
}

fn solve_part2(adjacencies: &Adjacencies) -> usize {
    let mut path: Vec<&str> = vec!["start"];
    let mut results: HashSet<Path> = HashSet::new();

    traverse(adjacencies, &mut path, true, false, &mut results);

    results.len()
}

generate_main!();

generate_tests!(10, 36);
