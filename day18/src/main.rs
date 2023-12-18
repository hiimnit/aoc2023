use std::{env, fmt::Write, fs, vec};

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(' ').collect();
            match parts[..] {
                [direction, steps, color] => Instruction::from(direction, steps, color),
                _ => panic!("Unexpected instruction format: {line}"),
            }
        })
        .collect();

    let mut map: Vec<Vec<Node>> = vec![vec![Node::Filled]];
    let (mut current_row, mut current_col) = (0usize, 0usize);
    let (mut rows, mut cols) = (1usize, 1usize);

    for Instruction {
        direction,
        steps,
        color: _,
    } in &instructions
    {
        (current_row, current_col) = match direction {
            Direction::Up => {
                if *steps > current_row {
                    expand_up(&mut map, steps - current_row);
                    rows += steps - current_row;
                    current_row = *steps;
                }

                for i in current_row - steps..=current_row {
                    map[i][current_col] = Node::Filled;
                }

                (current_row - steps, current_col)
            }
            Direction::Right => {
                if current_col + steps >= cols {
                    expand_right(&mut map, current_col + steps - cols + 1);
                    cols = current_col + steps + 1;
                }

                for i in current_col + 1..=current_col + steps {
                    map[current_row][i] = Node::Filled;
                }

                (current_row, current_col + steps)
            }
            Direction::Down => {
                if current_row + steps >= rows {
                    expand_down(&mut map, current_row + steps - rows + 1);
                    rows = current_row + steps + 1;
                }

                for i in current_row..=current_row + steps {
                    map[i][current_col] = Node::Filled;
                }

                (current_row + steps, current_col)
            }
            Direction::Left => {
                if *steps > current_col {
                    expand_left(&mut map, steps - current_col);
                    cols += steps - current_col;
                    current_col = *steps;
                }

                for i in current_col - steps..current_col {
                    map[current_row][i] = Node::Filled;
                }

                (current_row, current_col - steps)
            }
        };
    }

    // TODO store rows of ranges
    // TODO process by blocks - expand/contract ranges

    let mut part_1_result = fill(&mut map);

    for line in &map {
        for node in line {
            print!("{node}");
        }
        println!();
    }

    println!("Part 1 result {part_1_result}");
}

fn fill(map: &mut Vec<Vec<Node>>) -> usize {
    expand_up(map, 1);
    expand_right(map, 1);
    expand_down(map, 1);
    expand_left(map, 1);

    let border = count_filled(&map);

    let mut queue = vec![(0, 0)];

    while let Some(position) = queue.pop() {
        if map[position.0][position.1] == Node::Filled {
            continue;
        }

        map[position.0][position.1] = Node::Filled;
        if position.0 > 0 {
            queue.push((position.0 - 1, position.1));
        }
        if position.1 > 0 {
            queue.push((position.0, position.1 - 1));
        }
        if position.0 + 1 < map.len() {
            queue.push((position.0 + 1, position.1));
        }
        if position.1 + 1 < map[0].len() {
            queue.push((position.0, position.1 + 1));
        }
    }

    (map.len() * map[0].len()) - count_filled(map) + border
}

fn count_filled(map: &Vec<Vec<Node>>) -> usize {
    map.iter()
        .map(|line| line.iter().filter(|&e| *e == Node::Filled).count())
        .sum()
}

fn expand_up(map: &mut Vec<Vec<Node>>, rows: usize) {
    let cols = map.get(0).map(|e| e.len()).unwrap_or(0);

    for _ in 0..rows {
        map.insert(0, vec![Node::Empty; cols]);
    }
}

fn expand_right(map: &mut Vec<Vec<Node>>, cols: usize) {
    if map.is_empty() {
        return;
    }

    for line in map {
        line.extend([Node::Empty].repeat(cols));
    }
}

fn expand_down(map: &mut Vec<Vec<Node>>, rows: usize) {
    let cols = map.get(0).map(|e| e.len()).unwrap_or(0);

    for _ in 0..rows {
        map.push(vec![Node::Empty; cols]);
    }
}

fn expand_left(map: &mut Vec<Vec<Node>>, cols: usize) {
    if map.is_empty() {
        return;
    }

    for i in 0..map.len() {
        let mut extended_line = vec![Node::Empty; cols];
        extended_line.append(&mut map[i]);
        map[i] = extended_line;
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: usize,
    color: String,
}

impl Instruction {
    fn from(direction: &str, steps: &str, color: &str) -> Self {
        Self {
            direction: direction.into(),
            steps: steps
                .parse()
                .expect(&format!("Steps {steps} must be a valid usize")),
            color: color.into(),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("Unexpected direction {value}"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Node {
    // Vertical,
    Filled,
    #[default]
    Empty,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            // Node::Vertical => '|',
            Node::Filled => '#',
            Node::Empty => '.',
        })
    }
}
