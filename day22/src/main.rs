use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let mut flying_blocks: HashMap<usize, Vec<Block>> = HashMap::new();

    for line in input.lines() {
        let block: Block = line.into();

        let min_z = block.min_z();
        if let Some(blocks) = flying_blocks.get_mut(&min_z) {
            blocks.push(block);
        } else {
            flying_blocks.insert(min_z, vec![block]);
        }
    }

    let mut grounded_blocks: HashMap<usize, Vec<Block>> = HashMap::new();
    let mut disitegratable_supports: HashSet<Block> = HashSet::new();

    for z in &sorted_keys(&flying_blocks) {
        let blocks = flying_blocks.remove(z).unwrap(); // FIXME

        for block in blocks {
            ground_block(&mut grounded_blocks, &mut disitegratable_supports, block);
        }
    }

    // check each block, if it only rests on one support, remove that support from disitegratable_supports

    for blocks in grounded_blocks.values() {
        for block in blocks {
            // blocks on the ground are not supported by other blocks => skip
            if block.min_z() == 1 {
                continue;
            }

            let mut support = None;
            let mut support_counter = 0;

            for lower_block in &grounded_blocks[&(block.min_z() - 1)] {
                if blocks_intersect(block, lower_block) {
                    support = Some(lower_block);
                    support_counter += 1;
                }

                if support_counter > 1 {
                    break;
                }
            }

            assert!(support_counter != 0);
            if support_counter == 1 {
                let support = support.unwrap();
                disitegratable_supports.remove(support);
            }
        }
    }

    let part_1_result = disitegratable_supports.len();
    println!("Part 1 result {part_1_result}");
}

fn ground_block(
    grounded_blocks: &mut HashMap<usize, Vec<Block>>,
    disitegratable_supports: &mut HashSet<Block>,
    mut block: Block,
) {
    loop {
        if !can_drop(grounded_blocks, &block) {
            disitegratable_supports.insert(block.clone());

            let max_z = block.max_z();
            if let Some(blocks) = grounded_blocks.get_mut(&max_z) {
                blocks.push(block);
            } else {
                grounded_blocks.insert(max_z, vec![block]);
            }
            return;
        }

        block.drop();
    }
}

fn can_drop(grounded_blocks: &mut HashMap<usize, Vec<Block>>, block: &Block) -> bool {
    if block.min_z() == 1 {
        return false;
    }

    let Some(lower_level) = grounded_blocks.get(&(block.min_z() - 1)) else {
        return true;
    };

    for lower_block in lower_level {
        if blocks_intersect(block, lower_block) {
            return false;
        }
    }

    true
}

fn sorted_keys(flying_blocks: &HashMap<usize, Vec<Block>>) -> Vec<usize> {
    let mut result: Vec<_> = flying_blocks.keys().map(|e| e.clone()).collect();
    result.sort_unstable();
    result
}

type Point = (usize, usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Block {
    a: Point,
    b: Point,
}

impl Block {
    fn min_z(&self) -> usize {
        self.a.2.min(self.b.2)
    }

    fn max_z(&self) -> usize {
        self.a.2.max(self.b.2)
    }

    fn drop(&mut self) {
        if self.min_z() == 1 {
            return;
        }

        self.a = (self.a.0, self.a.1, self.a.2 - 1);
        self.b = (self.b.0, self.b.1, self.b.2 - 1);
    }

    fn x_range(&self) -> (usize, usize) {
        if self.a.0 <= self.b.0 {
            return (self.a.0, self.b.0);
        }
        (self.b.0, self.a.0)
    }

    fn y_range(&self) -> (usize, usize) {
        if self.a.1 <= self.b.1 {
            return (self.a.1, self.b.1);
        }
        (self.b.1, self.a.1)
    }
}

impl From<&str> for Block {
    fn from(value: &str) -> Self {
        let split: Vec<_> = value.split('~').collect();
        assert!(split.len() == 2);

        Block {
            a: parse_point(split[0]),
            b: parse_point(split[1]),
        }
    }
}

fn parse_point(value: &str) -> Point {
    let values: Vec<_> = value.split(',').filter_map(|e| e.parse().ok()).collect();
    assert!(values.len() == 3);
    (values[0], values[1], values[2])
}

fn blocks_intersect(upper: &Block, lower: &Block) -> bool {
    ranges_intersect(upper.x_range(), lower.x_range())
        && ranges_intersect(upper.y_range(), lower.y_range())
}

fn ranges_intersect(a: (usize, usize), b: (usize, usize)) -> bool {
    if a.0 >= b.0 && a.0 <= b.1 {
        return true;
    }

    if a.1 >= b.0 && a.1 <= b.1 {
        return true;
    }

    a.0 < b.0 && a.1 > b.1
}
