use std::{env, fs};

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let result: (i32, i32) = input
        .lines()
        .map(|line| solve(line))
        .fold((0, 0), |acc, e| (acc.0 + e.0, acc.1 + e.1));

    println!("Part 1 result {}", result.1);
    println!("Part 2 result {}", result.0);
}

fn solve(line: &str) -> (i32, i32) {
    let mut line: Vec<i32> = line.split(' ').filter_map(|e| e.parse().ok()).collect();

    assert!(line.len() > 1, "Can not extrapolate with a single entry.");

    let mut next_number = line[line.len() - 1];
    let mut first_entries = vec![line[0]];

    let mut len = line.len() - 1;

    loop {
        let mut all_zero = true;

        for i in 0..len {
            line[i] = line[i + 1] - line[i];
            all_zero = all_zero && line[i] == 0;
        }

        if all_zero {
            break;
        }

        len -= 1;
        next_number += line[len];

        first_entries.push(line[0]);
    }

    let previous_number = first_entries.iter().rev().fold(0, |acc, e| e - acc);

    (previous_number, next_number)
}
