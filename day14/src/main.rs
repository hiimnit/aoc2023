use std::{
    collections::HashMap,
    env,
    fmt::Write,
    fs,
};

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Node {
    Rock,
    Block,
    None,
}

impl Node {
    fn from(c: &char) -> Self {
        match c {
            'O' => Self::Rock,
            '#' => Self::Block,
            '.' => Self::None,
            _ => panic!("Unexpected node {c}"),
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Node::Rock => 'O',
            Node::Block => '#',
            Node::None => ' ',
        })
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let mut map: Vec<Vec<Node>> = input
        .lines()
        .map(|e| e.chars().map(|e| Node::from(&e)).collect())
        .collect();

    if map.is_empty() {
        panic!("Map must not be empty");
    }

    let (rows, cols) = (map.len(), map[0].len());

    tilt_north(&mut map, rows, cols);

    let part_1_result = evaluate(&map);

    println!("Part 1 result {part_1_result}");

    // complete the first spin cycle
    tilt_west(&mut map, rows, cols);
    tilt_south(&mut map, rows, cols);
    tilt_east(&mut map, rows, cols);

    let mut set = HashMap::new();

    set.insert(map.clone(), 0);

    let mut loop_params = None;

    // spin until we find a loop or until the end
    for i in 1..1_000_000_000 {
        tilt_north(&mut map, rows, cols);
        tilt_west(&mut map, rows, cols);
        tilt_south(&mut map, rows, cols);
        tilt_east(&mut map, rows, cols);

        if let Some(start) = set.get(&map.clone()) {
            // loop found
            loop_params = Some((*start, i - *start));

            break;
        } else {
            set.insert(map.clone(), i);
        }
    }

    let Some((loop_start, loop_len)) = loop_params else {
        // loop was not found, evaluate current state
        let part_2_result = evaluate(&map);
        println!("Part 2 result {part_2_result}");
        
        return;
    };

    // first cycle of the next loop is already done, so start at 1
    for _ in 1..((1_000_000_000 - loop_start) % loop_len) {
        tilt_north(&mut map, rows, cols);
        tilt_west(&mut map, rows, cols);
        tilt_south(&mut map, rows, cols);
        tilt_east(&mut map, rows, cols);
    }

    pretty_print_map(&map);

    let part_2_result = evaluate(&map);

    println!("Part 2 result {part_2_result}");
}

fn pretty_print_map(map: &Vec<Vec<Node>>) {
    for line in map {
        for node in line {
            print!("{node}")
        }
        println!("")
    }
}

fn tilt_north(map: &mut Vec<Vec<Node>>, rows: usize, cols: usize) {
    for col in 0..cols {
        let mut stop = 0;

        for row in 0..rows {
            match map[row][col] {
                Node::Rock => {
                    if stop != row {
                        map[stop][col] = Node::Rock;
                        map[row][col] = Node::None;
                    }
                    stop += 1;
                }
                Node::Block => {
                    stop = row + 1;
                }
                Node::None => {}
            }
        }
    }
}

fn tilt_south(map: &mut Vec<Vec<Node>>, rows: usize, cols: usize) {
    for col in 0..cols {
        let mut stop = rows - 1;

        for row in (0..rows).rev() {
            match map[row][col] {
                Node::Rock => {
                    if stop != row {
                        map[stop][col] = Node::Rock;
                        map[row][col] = Node::None;
                    }
                    stop = stop.saturating_sub(1);
                }
                Node::Block => {
                    stop = row.saturating_sub(1);
                }
                Node::None => {}
            }
        }
    }
}

fn tilt_west(map: &mut Vec<Vec<Node>>, rows: usize, cols: usize) {
    for row in 0..rows {
        let mut stop = 0;

        for col in 0..cols {
            match map[row][col] {
                Node::Rock => {
                    if stop != col {
                        map[row][stop] = Node::Rock;
                        map[row][col] = Node::None;
                    }
                    stop += 1;
                }
                Node::Block => {
                    stop = col + 1;
                }
                Node::None => {}
            }
        }
    }
}

fn tilt_east(map: &mut Vec<Vec<Node>>, rows: usize, cols: usize) {
    for row in 0..rows {
        let mut stop = cols - 1;

        for col in (0..cols).rev() {
            match map[row][col] {
                Node::Rock => {
                    if stop != col {
                        map[row][stop] = Node::Rock;
                        map[row][col] = Node::None;
                    }
                    stop = stop.saturating_sub(1);
                }
                Node::Block => {
                    stop = col.saturating_sub(1);
                }
                Node::None => {}
            }
        }
    }
}

fn evaluate(map: &Vec<Vec<Node>>) -> usize {
    map.iter()
        .enumerate()
        .map(|(row, line)| line.iter().filter(|e| **e == Node::Rock).count() * (map.len() - row))
        .sum()
}
