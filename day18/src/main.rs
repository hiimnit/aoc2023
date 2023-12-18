use std::{collections::HashMap, env, fs, thread::panicking, vec};

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

    let (mut current_row, mut current_col) = (0, 0);

    let mut range_map: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    for Instruction {
        direction,
        steps,
        color: _,
    } in &instructions
    {
        match direction {
            Direction::Up => {
                current_row -= steps;
            }
            Direction::Right => {
                let range = (current_col, current_col + steps);
                if let Some(ranges) = range_map.get_mut(&current_row) {
                    ranges.push(range);
                } else {
                    range_map.insert(current_row, vec![range]);
                }
                current_col += steps;
            }
            Direction::Down => {
                current_row += steps;
            }
            Direction::Left => {
                let range = (current_col - steps, current_col);
                if let Some(ranges) = range_map.get_mut(&current_row) {
                    ranges.push(range);
                } else {
                    range_map.insert(current_row, vec![range]);
                }
                current_col -= steps;
            }
        }
    }

    let mut rows: Vec<_> = range_map.keys().collect();
    rows.sort();

    let mut open_ranges: Vec<(i32, i32)> = vec![];
    let mut last_row = 0;
    let mut part_1_result = 0;

    for row in rows {
        let mut ranges = range_map[row].clone();

        ranges.sort();

        if open_ranges.is_empty() {
            open_ranges.extend(ranges);
            last_row = *row;
            continue;
        }

        for (start, end) in &open_ranges {
            part_1_result += ((end - start).abs() + 1) * (row - last_row).abs();
        }

        let mut next_open_ranges = vec![];

        while !ranges.is_empty() && !open_ranges.is_empty() {
            let first_range = ranges[0];
            let first_open_range = open_ranges[0];

            if first_range.1 < first_open_range.0 {
                ranges.remove(0);
                next_open_ranges.push(first_range);
            } else if first_range.1 == first_open_range.0 {
                ranges.remove(0);
                open_ranges[0] = (first_range.0, first_open_range.1);
            } else if first_range.0 == first_open_range.0 {
                ranges.remove(0);
                part_1_result += (first_range.1 - first_range.0).abs();

                if first_range.1 == first_open_range.1 {
                    part_1_result += 1;
                    open_ranges.remove(0);
                } else {
                    open_ranges[0] = (first_range.1, first_open_range.1);
                }
            } else if first_range.1 == first_open_range.1 {
                ranges.remove(0);
                part_1_result += (first_range.1 - first_range.0).abs();

                if first_range.0 == first_open_range.0 {
                    part_1_result += 1;
                    open_ranges.remove(0);
                } else {
                    open_ranges[0] = (first_open_range.0, first_range.0);
                }
            } else if first_open_range.1 == first_range.0 {
                open_ranges.remove(0);
                ranges[0] = (first_open_range.0, first_range.1);
            } else if first_open_range.1 < first_range.0 {
                open_ranges.remove(0);
                next_open_ranges.push(first_open_range);
            } else if first_open_range.0 < first_range.0 && first_open_range.1 > first_range.1 {
                ranges.remove(0);

                next_open_ranges.push((first_open_range.0, first_range.0));
                part_1_result += (first_range.1 - first_range.0).abs() - 1;

                open_ranges[0] = (first_range.1, first_open_range.1);
            } else {
                panic!();
            }
        }

        next_open_ranges.extend(ranges);
        next_open_ranges.extend(open_ranges);

        open_ranges = next_open_ranges;
        last_row = *row;
    }

    println!("Part 1 result {part_1_result}");
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: i32,
    color: String,
}

impl Instruction {
    fn from(direction: &str, steps: &str, color: &str) -> Self {
        Self {
            direction: direction.into(),
            steps: steps
                .parse()
                .expect(&format!("Steps {steps} must be a valid i32")),
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
