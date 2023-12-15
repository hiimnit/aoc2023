use std::{env, fs};

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let part_1_result: usize = input.split(',').map(hash).sum();

    println!("Part 1 result {part_1_result}");

    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    for instruction in input.split(',') {
        let instruction = Instruction::from(instruction);

        let current_box = boxes
            .get_mut(label_hash(&instruction))
            .expect("Hash output out of range");

        match instruction {
            Instruction::Replace(label, focal_length) => {
                if let Some(position) = current_box.iter().position(|e| e.0 == label) {
                    current_box[position] = (label, focal_length);
                } else {
                    current_box.push((label, focal_length));
                }
            }
            Instruction::Remove(label) => {
                if let Some(position) = current_box.iter().position(|e| e.0 == label) {
                    current_box.remove(position);
                }
            }
        }
    }

    let part_2_result = boxes
        .iter()
        .enumerate()
        .fold(0, |acc, (box_index, box_contents)| {
            acc + box_contents
                .iter()
                .enumerate()
                .fold(0, |acc, (lense_index, lense)| {
                    acc + (box_index + 1) * (lense_index + 1) * lense.1
                })
        });

    println!("Part 2 result {part_2_result}");
}

fn label_hash(instruction: &Instruction) -> usize {
    match instruction {
        Instruction::Replace(label, _) | Instruction::Remove(label) => hash(label),
    }
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}

#[derive(Debug, Clone, Copy)]
enum Instruction<'a> {
    Replace(&'a str, usize),
    Remove(&'a str),
}

impl<'a> Instruction<'a> {
    fn from(input: &'a str) -> Self {
        if input.ends_with('-') {
            return Instruction::Remove(&input[..input.len() - 1]);
        }

        let mut parts = input.split('=');
        let label = parts.next().expect("Expected a label before =");
        let focal_length = parts
            .next()
            .expect("Expected a focal length after =")
            .parse()
            .expect("Expected focal length to be a valid usize");

        if parts.next().is_some() {
            panic!("Unexpected chars after focal length");
        }

        Instruction::Replace(label, focal_length)
    }
}
