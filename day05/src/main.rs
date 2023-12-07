use std::{collections::HashMap, env, fs};

#[derive(Debug)]
struct Mapping {
    converts_to: String,
    rules: Vec<(i64, i64, i64)>,
}

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let mut blocks = input.split("\n\n");

    let seeds = blocks.next().expect("Expected block of seeds");
    let seeds = seeds.split(": ").last().expect("Expected block of seeds");
    let seeds: Vec<i64> = seeds.split(' ').filter_map(|e| e.parse().ok()).collect();

    let mut map: HashMap<String, usize> = HashMap::new();
    let mut mappings: Vec<Mapping> = vec![];

    for block in blocks {
        let mut lines = block.lines();

        let mapping_header = lines.next().expect("Expected block header");
        let mapping_header = mapping_header
            .split(' ')
            .next()
            .expect("Expected block header");
        let mapping_header: Vec<_> = mapping_header.split("-to-").collect();
        assert!(
            mapping_header.len() == 2,
            "Mapping header must contain two elements"
        );

        let mut rules = vec![];

        for line in lines {
            let mapping: Vec<i64> = line.split(' ').filter_map(|e| e.parse().ok()).collect();

            match mapping[..] {
                [dest, source, range] => rules.push((dest, source, range)),
                _ => panic!("Expected mapping to contain three integers"),
            }
        }

        rules.sort_unstable_by_key(|e| e.1);

        map.insert(mapping_header[0].to_string(), mappings.len());
        mappings.push(Mapping {
            converts_to: mapping_header[1].to_string(),
            rules,
        });
    }

    let mut part_1_result = i64::MAX;

    for seed in &seeds {
        let mut next_mapping = map.get("seed");
        let mut id = seed.clone();

        while let Some(mapping_index) = next_mapping {
            let mapping = &mappings[*mapping_index];

            id = convert_by_rules(&mapping.rules, id);

            next_mapping = map.get(&mapping.converts_to);
        }

        part_1_result = part_1_result.min(id);
    }

    println!("Part 1 result: {part_1_result}");

    let mut part_2_result = i64::MAX;

    for i in (0..seeds.len()).step_by(2) {
        // for seed in &seeds {
        let mut next_mapping = map.get("seed");
        let mut ranges = vec![(seeds[i], seeds[i + 1])];

        while let Some(mapping_index) = next_mapping {
            let mapping = &mappings[*mapping_index];

            let mut next_ranges = vec![];

            for range in &ranges {
                next_ranges.extend_from_slice(&convert_range_by_rules(&mapping.rules, *range));
            }

            next_mapping = map.get(&mapping.converts_to);
            ranges = next_ranges;
        }

        part_2_result = part_2_result.min(
            ranges
                .iter()
                .map(|e| e.0)
                .min()
                .expect("Range must contain at least one element"),
        );
    }

    println!("Part 2 result: {part_2_result}");
}

fn convert_by_rules(rules: &Vec<(i64, i64, i64)>, id: i64) -> i64 {
    for &(destination, source, range) in rules {
        if (source..source + range).contains(&id) {
            return destination + id - source;
        }
    }

    id
}

fn convert_range_by_rules(rules: &Vec<(i64, i64, i64)>, mut range: (i64, i64)) -> Vec<(i64, i64)> {
    let mut result = vec![];

    for (rule_destination, rule_start, rule_len) in rules {
        let (mut range_start, mut range_len) = range.clone();

        if range_start > *rule_start + *rule_len - 1 {
            // range starts after the rule, continue to the next one
            continue;
        }

        if range_start + range_len - 1 < *rule_start {
            // range ends before the rule, we are done
            result.push(range);
            return result;
        }

        // ranges intersect, split
        if range_start < *rule_start {
            let distance = *rule_start - range_start;
            result.push((range_start, distance));
            (range_start, range_len) = (*rule_start, range_len - distance);
        }

        if range_start + range_len <= *rule_start + *rule_len {
            let distance = range_start - *rule_start;
            let converted = (*rule_destination + distance, range_len);

            result.push(converted);
            return result;
        }

        let distance = range_start - *rule_start;
        let converted = (*rule_destination + distance, *rule_len - distance);
        result.push(converted);

        range = (*rule_start + *rule_len, range_len - (*rule_len - distance));
    }

    result.push(range);

    result
}
