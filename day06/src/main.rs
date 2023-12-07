use std::{env, fs};

fn solve(times: &str, distances: &str) -> i64 {
    let times: Vec<i64> = times
        .split(' ')
        .filter(|e| *e != "")
        .filter_map(|e| e.parse().ok())
        .collect();
    let distances: Vec<i64> = distances
        .split(' ')
        .filter(|e| *e != "")
        .filter_map(|e| e.parse().ok())
        .collect();

    let mut part_1_results = vec![];

    for (time, distance) in times.iter().zip(&distances) {
        let discriminant = time * time - 4 * distance;
        if discriminant <= 0 {
            continue;
        }

        let mut x_1 = ((*time as f64) - (discriminant as f64).sqrt()) / 2.0;
        let mut x_2 = ((*time as f64) + (discriminant as f64).sqrt()) / 2.0;

        if x_1 == x_1.ceil() {
            x_1 += 1.0;
        }
        if x_2 == x_2.floor() {
            x_2 -= 1.0;
        }

        let difference = (x_2.floor() as i64) - (x_1.ceil() as i64) + 1;

        part_1_results.push(difference);
    }

    if part_1_results.is_empty() {
        return 0;
    }

    part_1_results.iter().product::<i64>()
}

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let mut lines = input.lines();
    assert!(lines.clone().count() == 2, "Expected two lines in input");

    let times = lines.next().expect("Expected two lines in input");
    assert!(
        times.starts_with("Time:"),
        "Expected first line to start with 'Time:'"
    );
    let distances = lines.next().expect("Expected two lines in input");
    assert!(
        distances.starts_with("Distance:"),
        "Expected first line to start with 'Distance:'"
    );

    let part_1_result = solve(&times[5..], &distances[9..]);
    let part_2_result = solve(
        &times[5..].replace(' ', ""),
        &distances[9..].replace(' ', ""),
    );

    println!("Part 1 result {part_1_result}");
    println!("Part 2 result {part_2_result}");
}
