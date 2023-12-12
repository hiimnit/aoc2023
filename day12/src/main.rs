use std::{collections::HashMap, env, fs};

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

    for line in input.lines() {
        let [springs_str, damage_records] = line.split(' ').collect::<Vec<_>>()[..] else {
            panic!("Unexpected line format on line {line}");
        };

        let damage_records: Vec<usize> = damage_records
            .split(',')
            .filter_map(|e| e.parse().ok())
            .collect();

        let springs: Vec<_> = springs_str.split('.').filter(|&e| e != "").collect();

        let mut memo: HashMap<(usize, usize, usize), Option<usize>> = HashMap::new();
        part_1_result += solve(&springs, 0, 0, &damage_records, 0, &mut memo)
            .expect("Could not find solution for {springs:?} and {damage_records:?}");

        let damage_records: Vec<usize> = damage_records.repeat(5);
        let springs = [springs_str].repeat(5).join("?");
        let springs: Vec<_> = springs.split('.').filter(|&e| e != "").collect();
        let mut memo: HashMap<(usize, usize, usize), Option<usize>> = HashMap::new();

        part_2_result += solve(&springs, 0, 0, &damage_records, 0, &mut memo)
            .expect("Could not find solution for {springs:?} and {damage_records:?}");
    }

    println!("Part 1 result {part_1_result}");
    println!("Part 2 result {part_2_result}");
}

fn solve(
    springs: &Vec<&str>,
    mut current_spring: usize,
    mut current_char: usize,
    damages: &Vec<usize>,
    current_damage: usize,
    memo: &mut HashMap<(usize, usize, usize), Option<usize>>,
) -> Option<usize> {
    if current_spring < springs.len() && current_char >= springs[current_spring].len() {
        current_spring += 1;
        current_char = 0;
    }

    if let Some(&result) = memo.get(&(current_spring, current_char, current_damage)) {
        return result;
    }

    // out of springs and out of damages
    match (
        current_spring >= springs.len(),
        current_damage >= damages.len(),
    ) {
        (true, true) => return Some(1),
        (true, false) => return None,
        (false, true) => {
            if springs[current_spring][current_char..]
                .chars()
                .all(|e| e == '?')
                && springs[current_spring + 1..]
                    .iter()
                    .all(|e| e.chars().all(|e| e == '?'))
            {
                return Some(1);
            }
            return None;
        }
        (false, false) => {}
    }

    let next_char = springs[current_spring]
        .chars()
        .nth(current_char)
        .expect("Next char not found");

    match next_char {
        '?' => {
            let as_space = solve(
                springs,
                current_spring,
                current_char + 1,
                damages,
                current_damage,
                memo,
            );
            // println!("current_spring: {current_spring} current_char: {current_char} as_space: {as_space:?}");
            let as_damaged = match_damages(
                springs,
                current_spring,
                current_char + 1,
                damages,
                current_damage,
                memo,
            );
            // println!("current_spring: {current_spring} current_char: {current_char} as_damaged: {as_damaged:?}");
            let result = match (as_space, as_damaged) {
                (Some(a), Some(b)) => Some(a + b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            };

            memo.insert((current_spring, current_char, current_damage), result);

            result
        }
        '#' => {
            let result = match_damages(
                springs,
                current_spring,
                current_char + 1,
                damages,
                current_damage,
                memo,
            );

            memo.insert((current_spring, current_char, current_damage), result);

            result
        }
        c => panic!("Unexpected char {c}"),
    }
}

fn match_damages(
    springs: &Vec<&str>,
    mut current_spring: usize,
    mut current_char: usize,
    damages: &Vec<usize>,
    current_damage: usize,
    memo: &mut HashMap<(usize, usize, usize), Option<usize>>,
) -> Option<usize> {
    let mut damage_len = damages[current_damage] - 1;

    while damage_len > 0 {
        match springs[current_spring].chars().nth(current_char) {
            Some('#') | Some('?') => {}
            None => return None,
            Some(c) => panic!("Unexpected char {c}"),
        }

        current_char += 1;
        damage_len -= 1;
    }

    match springs[current_spring].chars().nth(current_char) {
        Some('?') => {
            current_char += 1;
        }
        None => {
            current_spring += 1;
            current_char = 0;
        }
        Some('#') => return None,
        Some(c) => panic!("Unexpected char {c}"),
    }

    return solve(
        springs,
        current_spring,
        current_char,
        damages,
        current_damage + 1,
        memo,
    );
}
