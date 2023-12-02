use std::{env::args, fs};

const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
const WORDS_AND_DIGITS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn first_digit(input: &str, items: &[&str]) -> usize {
    items
        .iter()
        .enumerate()
        .map(|(i, &word)| (input.find(word), i % 9 + 1))
        .filter(|e| e.0.is_some())
        .map(|(i, digit)| (i.unwrap(), digit))
        .min_by_key(|e| e.0)
        .expect("There must be at least one digit in input")
        .1
}

fn last_digit(input: &str, items: &[&str]) -> usize {
    items
        .iter()
        .enumerate()
        .map(|(i, &word)| (input.rfind(word), i % 9 + 1))
        .filter(|e| e.0.is_some())
        .map(|(i, digit)| (i.unwrap(), digit))
        .max_by_key(|e| e.0)
        .expect("There must be at least one digit in input")
        .1
}

fn main() {
    let mut args = args();
    let input_file = args.nth(1).expect("Input file");

    let file = fs::read_to_string(input_file).expect("Failed to read file");

    let mut part_1_result = 0;
    let mut part_2_result = 0;

    for line in file.lines() {
        part_1_result += first_digit(line, &DIGITS) * 10 + last_digit(line, &DIGITS);
        part_2_result +=
            first_digit(line, &WORDS_AND_DIGITS) * 10 + last_digit(line, &WORDS_AND_DIGITS);
    }

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}
