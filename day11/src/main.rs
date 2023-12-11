use std::{env, fs, vec};

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let bounds = (
        input.lines().count(),
        input.lines().next().expect("Input must not be empty").len(),
    );

    let mut galaxies = vec![];
    let mut empty_rows = vec![1; bounds.0];
    let mut empty_cols = vec![1; bounds.1];

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((row, col));
                empty_rows[row] = 0;
                empty_cols[col] = 0;
            }
        }
    }

    let mut part_1_result: i64 = 0;
    let mut part_2_result: i64 = 0;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            part_1_result += distance(galaxies[i], galaxies[j], &empty_rows, &empty_cols, 1);
            part_2_result += distance(galaxies[i], galaxies[j], &empty_rows, &empty_cols, 999_999);
        }
    }

    println!("Part 1 result {part_1_result}");
    println!("Part 2 result {part_2_result}");
}

fn distance(
    a: (usize, usize),
    b: (usize, usize),
    empty_rows: &Vec<i64>,
    empty_cols: &Vec<i64>,
    coef: i64,
) -> i64 {
    (a.0 as i64 - b.0 as i64).abs()
        + (a.1 as i64 - b.1 as i64).abs()
        + empty_rows[a.0.min(b.0)..b.0.max(a.0)].iter().sum::<i64>() * coef
        + empty_cols[a.1.min(b.1)..b.1.max(a.1)].iter().sum::<i64>() * coef
}
