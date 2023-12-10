use std::{
    collections::{HashSet, VecDeque},
    env,
    fmt::{Debug, Display, Write},
    fs,
    ops::Div,
};

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|e| Pipe::new(e)).collect())
        .collect();

    let bounds = (map.len(), map[0].len());

    let Some(loop_result) = find_loop(&map, &bounds) else {
        panic!("Loop not found")
    };

    let clean_map: Vec<Vec<_>> = build_clean_map(&map, &loop_result);

    for line in &clean_map {
        println!("{}", line.iter().map(|e| e.to_char()).collect::<String>())
    }

    let filled_fields = fill(&clean_map, &bounds);

    for (row, line) in clean_map.iter().enumerate() {
        for (col, pipe) in line.iter().enumerate() {
            if filled_fields.contains(&(row, col)) {
                print!("x")
            } else {
                print!("{}", pipe.to_char())
            }
        }
        println!("")
    }

    let part_1_result = (loop_result.used_positions.len() as f32).div(2.0).ceil() as usize;
    let part_2_result =
        bounds.0 * bounds.1 - loop_result.used_positions.len() - filled_fields.len();

    println!("Part 1 result {}", part_1_result);
    println!("Part 2 result {}", part_2_result);
}

fn build_clean_map(map: &Vec<Vec<Pipe>>, find_loop_result: &FindLoopResult) -> Vec<Vec<Pipe>> {
    let mut clean_map: Vec<Vec<_>> = vec![];

    for i in 0..map.len() {
        let mut row = vec![];
        for j in 0..map[i].len() {
            // todo
            row.push(if find_loop_result.used_positions.contains(&(i, j)) {
                map[i][j]
            } else {
                Pipe::None
            });
        }
        clean_map.push(row);
    }

    clean_map[find_loop_result.start_position.0][find_loop_result.start_position.1] = match (
        find_loop_result.start_direction,
        find_loop_result.end_direction,
    ) {
        (Direction::Up, Direction::Up) | (Direction::Down, Direction::Down) => Pipe::Vertical,
        (Direction::Up, Direction::Left) => Pipe::UpRight,
        (Direction::Up, Direction::Right) => Pipe::UpLeft,
        (Direction::Down, Direction::Left) => Pipe::DownRight,
        (Direction::Down, Direction::Right) => Pipe::DownLeft,
        (Direction::Left, Direction::Up) => Pipe::DownLeft,
        (Direction::Left, Direction::Down) => Pipe::UpLeft,
        (Direction::Left, Direction::Left) | (Direction::Right, Direction::Right) => {
            Pipe::Horizontal
        }
        (Direction::Right, Direction::Up) => Pipe::DownRight,
        (Direction::Right, Direction::Down) => Pipe::UpRight,
        _ => panic!("Unexpected start directions"),
    };

    clean_map
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(
        self: &Self,
        position: &(usize, usize),
        bounds: &(usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if position.0 == 0 {
                    return None;
                }
                Some((position.0 - 1, position.1))
            }
            Direction::Down => {
                if position.0 + 1 >= bounds.0 {
                    return None;
                }
                Some((position.0 + 1, position.1))
            }
            Direction::Left => {
                if position.1 == 0 {
                    return None;
                }
                Some((position.0, position.1 - 1))
            }
            Direction::Right => {
                if position.1 + 1 >= bounds.1 {
                    return None;
                }
                Some((position.0, position.1 + 1))
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Pipe {
    Vertical,   // |
    Horizontal, // -
    UpRight,    // L
    UpLeft,     // J
    DownLeft,   // 7
    DownRight,  // F
    None,       // .
    Start,      // S
}

impl Pipe {
    fn new(c: char) -> Self {
        match c {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::UpRight,
            'J' => Pipe::UpLeft,
            '7' => Pipe::DownLeft,
            'F' => Pipe::DownRight,
            '.' => Pipe::None,
            'S' => Pipe::Start,
            _ => panic!("Unknown char {c}"),
        }
    }

    fn next(self: &Self, from_direction: &Direction) -> Option<Direction> {
        match (self, from_direction) {
            (Pipe::Vertical, Direction::Up) => Some(Direction::Up),
            (Pipe::Vertical, Direction::Down) => Some(Direction::Down),
            (Pipe::Horizontal, Direction::Left) => Some(Direction::Left),
            (Pipe::Horizontal, Direction::Right) => Some(Direction::Right),
            (Pipe::UpRight, Direction::Down) => Some(Direction::Right),
            (Pipe::UpRight, Direction::Left) => Some(Direction::Up),
            (Pipe::UpLeft, Direction::Down) => Some(Direction::Left),
            (Pipe::UpLeft, Direction::Right) => Some(Direction::Up),
            (Pipe::DownLeft, Direction::Up) => Some(Direction::Left),
            (Pipe::DownLeft, Direction::Right) => Some(Direction::Down),
            (Pipe::DownRight, Direction::Up) => Some(Direction::Right),
            (Pipe::DownRight, Direction::Left) => Some(Direction::Down),
            _ => None,
        }
    }

    fn to_char(self: &Self) -> char {
        match self {
            Pipe::Vertical => '│',
            Pipe::Horizontal => '─',
            Pipe::UpRight => '└',
            Pipe::UpLeft => '┘',
            Pipe::DownLeft => '┐',
            Pipe::DownRight => '┌',
            Pipe::None => ' ',
            Pipe::Start => '*',
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_char())
    }
}

struct FindLoopResult {
    start_position: (usize, usize),
    used_positions: HashSet<(usize, usize)>,
    start_direction: Direction,
    end_direction: Direction,
}

fn find_loop(map: &Vec<Vec<Pipe>>, bounds: &(usize, usize)) -> Option<FindLoopResult> {
    let Some(start_position) = find_start(map) else {
        return None;
    };

    for direction in [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ] {
        if let Some((used_positions, end_direction)) =
            walk_map(map, start_position, direction.clone(), bounds)
        {
            // looks like there is only one valid loop
            return Some(FindLoopResult {
                start_position,
                used_positions,
                start_direction: direction,
                end_direction,
            });
        }
    }

    None
}

fn find_start(map: &Vec<Vec<Pipe>>) -> Option<(usize, usize)> {
    for (row_number, row) in map.iter().enumerate() {
        if let Some(col) = row.iter().position(|e| *e == Pipe::Start) {
            return Some((row_number, col));
        }
    }

    None
}

fn walk_map(
    map: &Vec<Vec<Pipe>>,
    start_position: (usize, usize),
    start_direction: Direction,
    bounds: &(usize, usize),
) -> Option<(HashSet<(usize, usize)>, Direction)> {
    let mut direction = start_direction;
    let mut position = start_position;
    let mut positions = HashSet::new();

    loop {
        let Some(next_position) = direction.next(&position, &bounds) else {
            return None;
        };

        positions.insert(next_position);

        let pipe = &map[next_position.0][next_position.1];
        if *pipe == Pipe::Start {
            return Some((positions, direction));
        }

        let Some(next_direction) = pipe.next(&direction) else {
            return None;
        };

        direction = next_direction;
        position = next_position;
    }
}

fn fill(map: &Vec<Vec<Pipe>>, bounds: &(usize, usize)) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    let mut filled_fields: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((0, 0));

    loop {
        let Some(position) = queue.pop_front() else {
            break
        };

        if !visited.insert(position) {
            continue;
        }

        let [up_left, up_right, down_right, down_left] = get_neigbors(&position, bounds);

        if let Some((row, col)) = up_left {
            if map[row][col] == Pipe::None {
                filled_fields.insert((row, col));
            }
        }
        if let Some((row, col)) = up_right {
            if map[row][col] == Pipe::None {
                filled_fields.insert((row, col));
            }
        }
        if let Some((row, col)) = down_right {
            if map[row][col] == Pipe::None {
                filled_fields.insert((row, col));
            }
        }
        if let Some((row, col)) = down_left {
            if map[row][col] == Pipe::None {
                filled_fields.insert((row, col));
            }
        }

        // Direction::Up
        if !connect_horizontally(get_field(map, &up_left), get_field(map, &up_right)) {
            // println!("{:?}, {:?}", up_left, up_right);
            queue.push_back((position.0 - 1, position.1));
        }

        // Direction::Right
        if !connect_vertically(get_field(map, &up_right), get_field(map, &down_right)) {
            queue.push_back((position.0, position.1 + 1));
        }

        // Direction::Down
        if !connect_horizontally(get_field(map, &down_left), get_field(map, &down_right)) {
            queue.push_back((position.0 + 1, position.1));
        }

        // Direction::Left
        if !connect_vertically(get_field(map, &up_left), get_field(map, &down_left)) {
            queue.push_back((position.0, position.1 - 1));
        }
    }

    filled_fields
}

fn get_field(map: &Vec<Vec<Pipe>>, position: &Option<(usize, usize)>) -> Option<Pipe> {
    let Some(position) = position else {
        return None;
    };

    map.get(position.0)?.get(position.1).copied()
}

fn get_neigbors(position: &(usize, usize), bounds: &(usize, usize)) -> [Option<(usize, usize)>; 4] {
    [
        if position.0 > 0 && position.1 > 0 {
            Some((position.0 - 1, position.1 - 1))
        } else {
            None
        },
        if position.0 > 0 && position.1 < bounds.1 {
            Some((position.0 - 1, position.1))
        } else {
            None
        },
        if position.0 < bounds.0 && position.1 < bounds.1 {
            Some((position.0, position.1))
        } else {
            None
        },
        if position.0 < bounds.0 && position.1 > 0 {
            Some((position.0, position.1 - 1))
        } else {
            None
        },
    ]
}

fn connect_horizontally(left_pipe: Option<Pipe>, right_pipe: Option<Pipe>) -> bool {
    match (left_pipe, right_pipe) {
        (None, None) => true,
        (
            Some(Pipe::Horizontal) | Some(Pipe::DownRight) | Some(Pipe::UpRight),
            Some(Pipe::Horizontal) | Some(Pipe::DownLeft) | Some(Pipe::UpLeft),
        ) => true,
        _ => false,
    }
}

fn connect_vertically(upper_pipe: Option<Pipe>, lower_pipe: Option<Pipe>) -> bool {
    match (upper_pipe, lower_pipe) {
        (None, None) => true,
        (
            Some(Pipe::Vertical) | Some(Pipe::DownLeft) | Some(Pipe::DownRight),
            Some(Pipe::Vertical) | Some(Pipe::UpLeft) | Some(Pipe::UpRight),
        ) => true,
        _ => false,
    }
}
