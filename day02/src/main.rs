use std::{env::args, fs};

fn main() {
    let mut args = args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Input file path expected as first argument");
    let input_file_content = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {}", input_file_path));

    let mut part_1_result = 0;
    let mut part_2_result = 0;

    for line in input_file_content.lines() {
        let split = line.split(": ").collect::<Vec<_>>();
        assert!(split.len() == 2);

        let game_id: i32 = split[0]
            .split(' ')
            .last()
            .expect("Game id must not be empty")
            .parse()
            .expect("Game id must be an integer");
        let draws = split[1];

        // RGB counts
        let mut counts = vec![0, 0, 0];

        for draw in draws.split("; ") {
            for die in draw.split(", ") {
                let die = die.split(' ').collect::<Vec<_>>();
                assert!(die.len() == 2);

                let count: i32 = die[0]
                    .parse()
                    .expect("First part of dice draw must be an integer");
                let color = die[1];

                let color_index = match color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    other => panic!("Unexpected color {}", other),
                };

                counts[color_index] = counts[color_index].max(count);
            }
        }

        if counts[0] <= 12 && counts[1] <= 13 && counts[2] <= 14 {
            part_1_result += game_id;
        }

        part_2_result += counts.iter().fold(1, |acc, e| acc * e);
    }

    println!("Part 1 result {}", part_1_result);
    println!("Part 2 result {}", part_2_result);
}
