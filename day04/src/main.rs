use std::{env, fs};

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let mut part_1_result = 0;

    let mut card_counter = vec![1; input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        let colon_position = line
            .find(": ")
            .expect("Expected card format to include ':'");

        let mut numbers = (&line[colon_position + 2..]).clone().split("|");
        let winning_numbers = numbers.next().expect("Expected winning numbers before '|'");
        let drawn_numbers = numbers.next().expect("Expected drawn numbers after '|'");

        let winning_numbers: Vec<i32> = winning_numbers
            .split(' ')
            .filter_map(|e| e.parse().ok())
            .collect();

        let drawn_numbers: Vec<i32> = drawn_numbers
            .split(' ')
            .filter_map(|e| e.parse().ok())
            .collect();

        let found_numbers = drawn_numbers
            .iter()
            .filter(|e| winning_numbers.contains(e))
            .count();

        if found_numbers > 0 {
            part_1_result += 2i32.pow(found_numbers as u32 - 1);
        }

        for j in i + 1..(i + 1 + found_numbers).min(card_counter.len()) {
            card_counter[j] += card_counter[i];
        }
    }

    println!("Part 1 result {part_1_result}");
    println!(
        "Part 2 result {part_2_result}",
        part_2_result = card_counter.iter().sum::<i32>()
    );
}
