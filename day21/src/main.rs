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

    let part_1_result = solve(&map, start_position, 64);

    println!("Part 1 result {part_1_result}");

    // map is 131 characters wide
    // we need 26_501_365 steps
    // we start in the middle = 65 steps to the edge
    // (26_501_365 - 65) % 131 == 0
    // nice and even number
    // (26_501_365 - 65) / 131 == 202_300

    let mut visited: HashSet<PositionSigned> = HashSet::new();
    let mut result: HashSet<PositionSigned> = HashSet::new();
    let mut other_result: HashSet<PositionSigned> = HashSet::new();
    let mut queue: Vec<PositionSigned> = vec![(start_position.0 as i32, start_position.1 as i32)];

    for _ in 0..131 + 131 + 131 + 65 {
        std::mem::swap(&mut result, &mut other_result);

        let mut next_queue = vec![];

        for position in queue {
            for next_position in next_position_looped(&map, &position) {
                if visited.insert(next_position) {
                    next_queue.push(next_position);
                }
                result.insert(next_position);
            }
        }

        queue = next_queue;
    }

    let x = solve(&map, (65, 65), 131 + 65);
    let left = solve(&map, (65, 130), 130);
    let right = solve(&map, (65, 0), 130);
    let top = solve(&map, (130, 65), 130);
    let bottom = solve(&map, (0, 65), 130);

    let top_left_small = solve(&map, (130, 130), 64);
    let top_right_small = solve(&map, (130, 0), 64);
    let bottom_left_small = solve(&map, (0, 130), 64);
    let bottom_right_small = solve(&map, (0, 0), 64);

    println!("X {}", x);
    println!("< {}", left);
    println!("> {}", right);
    println!("^ {}", top);
    println!("v {}", bottom);

    println!(
        "33695 = {}",
        x + left
            + right
            + top
            + bottom
            + top_left_small
            + top_right_small
            + bottom_left_small
            + bottom_right_small
    );

    let x = solve(&map, (65, 65), 131 + 131 + 65);
    let x_other = solve(&map, (65, 65), 131 + 131);

    let top_left_large = solve(&map, (130, 130), 131 + 64);
    let top_right_large = solve(&map, (130, 0), 131 + 64);
    let bottom_left_large = solve(&map, (0, 130), 131 + 64);
    let bottom_right_large = solve(&map, (0, 0), 131 + 64);

    println!("x={x} x_other={x_other}");

    println!(
        "93438 = {}",
        x + 4 * x_other
            + left
            + right
            + top
            + bottom
            + 2 * top_left_small
            + 2 * top_right_small
            + 2 * bottom_left_small
            + 2 * bottom_right_small
            + top_left_large
            + top_right_large
            + bottom_left_large
            + bottom_right_large
    );

    println!(
        "183007 = {}",
        4 * x
            + 9 * x_other
            + left
            + right
            + top
            + bottom
            + 3 * top_left_small
            + 3 * top_right_small
            + 3 * bottom_left_small
            + 3 * bottom_right_small
            + 2 * top_left_large
            + 2 * top_right_large
            + 2 * bottom_left_large
            + 2 * bottom_right_large
    );

    let x = solve(&map, (65, 65), 131 + 65);
    let x_other = solve(&map, (65, 65), 131);

    println!(
        "Part 2 result = {}",
        202_300 * 202_300 * x
            + (202_300 - 1) * (202_300 - 1) * x_other
            + left
            + right
            + top
            + bottom
            + 202_300 * top_left_small
            + 202_300 * top_right_small
            + 202_300 * bottom_left_small
            + 202_300 * bottom_right_small
            + (202_300 - 1) * top_left_large
            + (202_300 - 1) * top_right_large
            + (202_300 - 1) * bottom_left_large
            + (202_300 - 1) * bottom_right_large
    );

    let part_2_result = result.len();

    println!("Part 2 result {part_2_result}");
}

fn solve(map: &Vec<Vec<Node>>, start_position: Position, steps: usize) -> usize {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut result: HashSet<Position> = HashSet::new();
    let mut other_result: HashSet<Position> = HashSet::new();
    let mut queue: Vec<Position> = vec![start_position];

    for _ in 0..steps {
        std::mem::swap(&mut result, &mut other_result);

        let mut next_queue = vec![];

        for position in queue {
            for next_position in next_position(map, &position) {
                if visited.insert(next_position) {
                    next_queue.push(next_position);
                }
                result.insert(next_position);
            }
        }

        queue = next_queue;
    }

    result.len()
}

type Position = (usize, usize);
type PositionSigned = (i32, i32);

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

fn next_position_looped(map: &Vec<Vec<Node>>, position: &PositionSigned) -> Vec<PositionSigned> {
    let mut result = vec![];

    let unsigned_pos = (
        position.0.rem_euclid(map.len() as i32) as usize,
        position.1.rem_euclid(map[0].len() as i32) as usize,
    );

    if map[unsigned_pos.0.wrapping_sub(1).min(map.len() - 1)][unsigned_pos.1] != Node::Rock {
        result.push((position.0 - 1, position.1))
    }
    if map[unsigned_pos.0][unsigned_pos
        .1
        .wrapping_sub(1)
        .min(map[unsigned_pos.0].len() - 1)]
        != Node::Rock
    {
        result.push((position.0, position.1 - 1))
    }
    if map[(unsigned_pos.0 + 1) % map.len()][unsigned_pos.1] != Node::Rock {
        result.push((position.0 + 1, position.1))
    }
    if map[unsigned_pos.0][(unsigned_pos.1 + 1) % map[unsigned_pos.0].len()] != Node::Rock {
        result.push((position.0, position.1 + 1))
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
