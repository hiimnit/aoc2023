use std::{collections::HashMap, env, fs, vec};

use num::Integer;

#[derive(Debug)]
enum Instruction {
    R,
    L,
}

impl Instruction {
    fn new(c: &char) -> Self {
        match c {
            'L' => Self::L,
            'R' => Self::R,
            _ => panic!("Unexpected instruction {c}"),
        }
    }
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

    let instructions = blocks
        .next()
        .expect("Expected instructions as the first block");

    let instructions: Vec<_> = instructions.chars().map(|e| Instruction::new(&e)).collect();

    let nodes = blocks.next().expect("Expected nodes as the second block");
    let nodes: Vec<_> = nodes
        .lines()
        .map(|e| e.split(" = ").collect::<Vec<_>>())
        .collect();

    let mut node_map: HashMap<&str, usize> = HashMap::new();
    let mut node_links = vec![];

    for node in &nodes {
        match node[..] {
            [code, links] => {
                node_map.insert(code, node_links.len());

                let links = links.replace("(", "").replace(")", "");
                let links: Vec<String> = links.split(", ").map(|e| e.to_string()).collect();
                match &links[..] {
                    [left, right] => node_links.push((left.clone(), right.clone())),
                    _ => panic!("Unexpected link format {links:?}"),
                }
            }
            _ => panic!("Unexpected node format {node:?}"),
        }
    }

    let part_1_result = solve(&node_map, &node_links, &instructions, "AAA");

    println!("Part 1 result {part_1_result}");

    let part_2_result: i64 = node_map
        .keys()
        .filter(|e| e.ends_with('A'))
        .map(|e| solve(&node_map, &node_links, &instructions, e))
        .fold(1, |acc, e| acc.lcm(&e));

    println!("Part 2 result {part_2_result}");
}

fn solve(
    node_map: &HashMap<&str, usize>,
    node_links: &Vec<(String, String)>,
    instructions: &Vec<Instruction>,
    start_position: &str,
) -> i64 {
    let mut counter = 0;
    let mut position = *node_map.get(start_position).unwrap();

    for instruction in instructions.iter().cycle() {
        counter += 1;

        let next_node = match instruction {
            Instruction::L => &node_links[position].0,
            Instruction::R => &node_links[position].1,
        };
        if next_node.ends_with('Z') {
            break;
        }
        position = *node_map
            .get(next_node.as_str())
            .expect(&format!("Expected {next_node} in node map."));
    }

    counter
}
