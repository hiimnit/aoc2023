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
    let mut part_2_result = 0;

    let blocks = input.split("\n\n");

    for block in blocks {
        let result = solve(block);
        part_1_result += result.0;
        part_2_result += result.1;
    }

    println!("Part 1 result {part_1_result}");
    println!("Part 2 result {part_2_result}");
}

fn solve(block: &str) -> (usize, usize) {
    let block_lines: Vec<_> = block.lines().collect();

    let mut clean: Option<usize> = None;
    let mut smudged: Option<usize> = None;

    for row in 1..block_lines.len() {
        clean = clean.or_else(|| {
            if is_mirrored(&block_lines, row) {
                Some(row * 100)
            } else {
                None
            }
        });
        smudged = smudged.or_else(|| {
            if is_mirrored_with_smudge(&block_lines, row) {
                Some(row * 100)
            } else {
                None
            }
        });

        if let (Some(clean), Some(smudged)) = (clean, smudged) {
            return (clean, smudged);
        }
    }

    let transposed_block_lines: Vec<_> = transpose_block(&block_lines);
    let transposed_block_lines: Vec<&str> =
        transposed_block_lines.iter().map(|e| e.as_ref()).collect();

    for row in 1..transposed_block_lines.len() {
        clean = clean.or_else(|| {
            if is_mirrored(&transposed_block_lines, row) {
                Some(row)
            } else {
                None
            }
        });
        smudged = smudged.or_else(|| {
            if is_mirrored_with_smudge(&transposed_block_lines, row) {
                Some(row)
            } else {
                None
            }
        });

        if let (Some(clean), Some(smudged)) = (clean, smudged) {
            return (clean, smudged);
        }
    }

    panic!("Did not find a mirror in block\n{block}")
}

fn is_mirrored(lines: &Vec<&str>, starting_row: usize) -> bool {
    if starting_row <= 0 || starting_row >= lines.len() {
        return false;
    }

    lines
        .iter()
        .skip(starting_row)
        .zip(lines.iter().take(starting_row).rev())
        .all(|e| e.0 == e.1)
}

fn is_mirrored_with_smudge(lines: &Vec<&str>, starting_row: usize) -> bool {
    if starting_row <= 0 || starting_row >= lines.len() {
        return false;
    }

    let mut smudge_found = false;

    for (left, right) in lines
        .iter()
        .skip(starting_row)
        .zip(lines.iter().take(starting_row).rev())
    {
        if left == right {
            continue;
        }

        if smudge_found {
            return false;
        }

        let differences = left
            .chars()
            .zip(right.chars())
            .filter(|(l, r)| l != r)
            .count();

        if differences != 1 {
            return false;
        }

        smudge_found = true;
    }

    smudge_found
}

fn transpose_block<'a>(lines: &Vec<&'a str>) -> Vec<String> {
    (0..lines[0].len())
        .map(|e| {
            lines
                .iter()
                .map(|line| {
                    line.chars()
                        .nth(e)
                        .expect("Expected all lines to have the same length")
                })
                .collect()
        })
        .collect()
}
