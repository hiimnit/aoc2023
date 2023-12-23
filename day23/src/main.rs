use std::{
    collections::{HashMap, HashSet},
    env, fs,
    io::Empty,
};

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

    if map.is_empty() {
        panic!("Unexpected empty map");
    }

    let start = map
        .first()
        .map(|e| e.iter().position(|e| *e == Node::Empty))
        .expect("Start not found")
        .expect("Start not found");

    let end = map
        .last()
        .map(|e| e.iter().position(|e| *e == Node::Empty))
        .expect("End not found")
        .expect("End not found");

    let mut visited: HashSet<Position> = HashSet::new();
    let part_1_result = find_longest_path(&map, &(map.len() - 1, end), (0, start), &mut visited);

    println!("Part 1 result {part_1_result:?}");
}

fn find_longest_path(
    map: &Map,
    end: &Position,
    position: Position,
    visited: &mut HashSet<Position>,
) -> Option<usize> {
    if position.0 == end.0 && position.1 == end.1 {
        return Some(0);
    }

    visited.insert(position);

    let next_steps = next_steps(map, &position, visited);

    if next_steps.len() == 1 {
        let position = *next_steps.first().unwrap();
        return find_longest_path(map, end, position, visited).map(|e| e + 1);
    }

    next_steps
        .into_iter()
        .filter_map(|e| {
            let mut visited = visited.clone();
            find_longest_path(map, end, e, &mut visited)
        })
        .max()
        .map(|e| e + 1)
}

type Map = Vec<Vec<Node>>;
type Position = (usize, usize);

fn next_steps(map: &Map, position: &Position, visited: &HashSet<Position>) -> Vec<Position> {
    let mut result = vec![];

    if position.0 > 0 {
        add_position(
            map,
            visited,
            &mut result,
            (position.0 - 1, position.1),
            Direction::Up,
        );
    }

    if position.1 + 1 < map[position.0].len() {
        add_position(
            map,
            visited,
            &mut result,
            (position.0, position.1 + 1),
            Direction::Right,
        );
    }

    if position.0 + 1 < map.len() {
        add_position(
            map,
            visited,
            &mut result,
            (position.0 + 1, position.1),
            Direction::Down,
        );
    }

    if position.1 > 0 {
        add_position(
            map,
            visited,
            &mut result,
            (position.0, position.1 - 1),
            Direction::Left,
        );
    }

    result
}

fn add_position(
    map: &Map,
    visited: &HashSet<Position>,
    next_positions: &mut Vec<Position>,
    position: Position,
    direction: Direction,
) {
    if !can_go_to(&map[position.0][position.1], direction) {
        return;
    }

    if visited.contains(&position) {
        return;
    }

    next_positions.push(position)
}

fn can_go_to(node: &Node, from_direction: Direction) -> bool {
    match node {
        Node::Wall => false,
        Node::Empty => true,
        Node::Slope(slope_direction) => *slope_direction == from_direction,
    }
}

#[derive(Debug, PartialEq)]
enum Node {
    Empty,
    Wall,
    Slope(Direction),
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '^' => Self::Slope(Direction::Up),
            '>' => Self::Slope(Direction::Right),
            'v' => Self::Slope(Direction::Down),
            '<' => Self::Slope(Direction::Left),
            _ => panic!("Unexpected node {value}"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
