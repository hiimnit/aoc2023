use std::{collections::HashSet, env, fs};

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let map: Vec<Vec<Node>> = input
        .lines()
        .map(|e| e.chars().map(|e| e.into()).collect())
        .collect();

    let start_position = map
        .iter()
        .enumerate()
        .filter_map(|(row, nodes)| {
            nodes
                .iter()
                .position(|e| *e == Node::Visited)
                .map(|col| (row, col))
        })
        .next();

    let Some(start_position) = start_position else {
        panic!("Start position not found");
    };

    let mut visited: HashSet<Position> = HashSet::new();
    let mut result: HashSet<Position> = HashSet::new();
    let mut other_result: HashSet<Position> = HashSet::new();
    let mut queue: Vec<Position> = vec![start_position];

    for _ in 0..64 {
        std::mem::swap(&mut result, &mut other_result);

        let mut next_queue = vec![];

        for position in queue {
            for next_position in next_position(&map, &position) {
                if visited.insert(next_position) {
                    next_queue.push(next_position);
                }
                result.insert(next_position);
            }
        }

        queue = next_queue;
    }

    let part_1_result = result.len();

    println!("Part 1 result {part_1_result}");
}

type Position = (usize, usize);

fn next_position(map: &Vec<Vec<Node>>, position: &Position) -> Vec<Position> {
    let mut result = vec![];
    if position.0 > 0 {
        if map[position.0 - 1][position.1] != Node::Rock {
            result.push((position.0 - 1, position.1))
        }
    }
    if position.1 > 0 {
        if map[position.0][position.1 - 1] != Node::Rock {
            result.push((position.0, position.1 - 1))
        }
    }
    if position.0 + 1 < map.len() {
        if map[position.0 + 1][position.1] != Node::Rock {
            result.push((position.0 + 1, position.1))
        }
    }
    if position.1 + 1 < map[position.0].len() {
        if map[position.0][position.1 + 1] != Node::Rock {
            result.push((position.0, position.1 + 1))
        }
    }

    result
}

#[derive(Debug, PartialEq, Clone)]
enum Node {
    Plot,
    Rock,
    Visited,
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Plot,
            '#' => Self::Rock,
            'S' => Self::Visited,
            _ => panic!("Unexpected node {value}"),
        }
    }
}
