use std::{env, fs, vec};

fn check_symbol_safe(schema: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let Some(row) = schema.get(row) else {
        return false;
    };

    let Some(cell) = row.get(col) else {
        return false;
    };

    *cell != '.' && !cell.is_numeric()
}

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args.next().expect("Input file path");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}."));

    let mut schema: Vec<_> = vec![];
    for line in input.lines() {
        schema.push(line.chars().collect::<Vec<_>>());
    }

    let mut part_1_result = 0;

    let mut parsed_numbers: Vec<Vec<(i32, usize, usize)>> = vec![];
    let mut parsed_gears: Vec<(usize, usize)> = vec![];

    for i in 0..schema.len() {
        let mut number: Option<(i32, usize)> = None;
        let mut symbol_found = false;

        let mut parsed_numbers_row = vec![];

        for j in 0..schema[i].len() {
            if schema[i][j].is_numeric() {
                if let Some((value, start_position)) = number {
                    number = Some((
                        value * 10 + (schema[i][j] as u8 - '0' as u8) as i32,
                        start_position,
                    ));
                } else {
                    number = Some((
                        (schema[i][j] as u8 - '0' as u8) as i32,
                        if j > 0 { j - 1 } else { j },
                    ));

                    if j > 0 {
                        if i > 0 {
                            symbol_found = symbol_found || check_symbol_safe(&schema, i - 1, j - 1);
                        }
                        symbol_found = symbol_found
                            || check_symbol_safe(&schema, i, j - 1)
                            || check_symbol_safe(&schema, i + 1, j - 1);
                    }
                }
                if number.is_some() && !symbol_found {
                    if i > 0 {
                        symbol_found = symbol_found || check_symbol_safe(&schema, i - 1, j);
                    }
                    symbol_found = symbol_found || check_symbol_safe(&schema, i + 1, j);
                }
            } else {
                if schema[i][j] == '*' {
                    parsed_gears.push((i, j));
                }
                if number.is_some() && !symbol_found {
                    if i > 0 {
                        symbol_found = symbol_found || check_symbol_safe(&schema, i - 1, j);
                    }
                    symbol_found = symbol_found
                        || check_symbol_safe(&schema, i, j)
                        || check_symbol_safe(&schema, i + 1, j);
                }
                if symbol_found {
                    if let Some((value, start_position)) = number {
                        part_1_result += value;

                        parsed_numbers_row.push((value, start_position, j));
                    }
                }

                number = None;
                symbol_found = false;
            }
        }

        if symbol_found {
            if let Some((value, start_position)) = number {
                part_1_result += value;

                parsed_numbers_row.push((value, start_position, schema[i].len() - 1));
            }
        }

        parsed_numbers.push(parsed_numbers_row);
    }

    let mut part_2_result = 0;

    for (i, j) in parsed_gears {
        if let Some(number) = get_gear_ratio(&parsed_numbers, i, j) {
            part_2_result += number;
        }
    }

    println!("Part 1 result: {part_1_result}");
    println!("Part 2 result: {part_2_result}");
}

fn get_gear_ratio(
    parsed_numbers: &Vec<Vec<(i32, usize, usize)>>,
    i: usize,
    j: usize,
) -> Option<i32> {
    let mut result = vec![];

    for line in (i - 1).max(0)..=(i + 1).min(parsed_numbers.len() - 1) {
        result.extend(
            parsed_numbers[line]
                .iter()
                .filter(|(_value, start, end)| (start..=end).contains(&&j))
                .map(|e| e.0),
        );
    }

    match result[..] {
        [first, second] => Some(first * second),
        _ => None,
    }
}
