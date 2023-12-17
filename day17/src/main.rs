use std::{cmp::Reverse, collections::BinaryHeap, env, fs};

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect();

    if map.is_empty() {
        panic!("Map must not be empty");
    }

    let bounds = (map.len(), map[0].len());

    // println!("{map:?}");
    // println!("{bounds:?}");

    let mut heap = BinaryHeap::new();

    heap.push((Reverse(0), 0, 0, Direction::Vertical));
    heap.push((Reverse(0), 0, 0, Direction::Horizontal));

    let mut distance_horizontal: Vec<Vec<Option<usize>>> = vec![vec![None; bounds.1]; bounds.0];
    distance_horizontal[0][0] = Some(0);
    let mut distance_vertical: Vec<Vec<Option<usize>>> = vec![vec![None; bounds.1]; bounds.0];
    distance_vertical[0][0] = Some(0);

    while let Some((Reverse(cost), row, col, direction)) = heap.pop() {
        // println!("popped {cost} {row} {col} {direction:?}");

        if row == bounds.0 - 1 && col == bounds.1 - 1 {
            // println!("{heap:#?}");
            println!("Done {cost:?} {direction:?}");
            break;
        }

        match direction {
            Direction::Vertical => {
                if let Some(best_cost) = distance_vertical[row][col] {
                    if best_cost < cost {
                        continue;
                    }
                }

                // move right - either by one, two or three
                let mut added_cost = cost;
                for col in (col + 1)..(col + 4).min(bounds.1) {
                    added_cost += map[row][col] as usize;

                    if let Some(best_cost) = distance_horizontal[row][col] {
                        if best_cost < added_cost {
                            continue;
                        }
                    }

                    heap.push((Reverse(added_cost), row, col, Direction::Horizontal));
                    distance_horizontal[row][col] = Some(added_cost);
                }

                // move left - either by one, two or three
                let mut added_cost = cost;
                for col in (col.saturating_sub(3)..col).rev() {
                    added_cost += map[row][col] as usize;

                    if let Some(best_cost) = distance_horizontal[row][col] {
                        if best_cost < added_cost {
                            continue;
                        }
                    }

                    heap.push((Reverse(added_cost), row, col, Direction::Horizontal));
                    distance_horizontal[row][col] = Some(added_cost);
                }
            }
            Direction::Horizontal => {
                if let Some(best_cost) = distance_horizontal[row][col] {
                    if best_cost < cost {
                        continue;
                    }
                }

                // move down - either by one, two or three
                let mut added_cost = cost;
                for row in (row + 1)..(row + 4).min(bounds.0) {
                    added_cost += map[row][col] as usize;

                    if let Some(best_cost) = distance_vertical[row][col] {
                        if best_cost < added_cost {
                            continue;
                        }
                    }

                    heap.push((Reverse(added_cost), row, col, Direction::Vertical));
                    distance_vertical[row][col] = Some(added_cost);
                }

                // move up - either by one, two or three
                let mut added_cost = cost;
                for row in (row.saturating_sub(3)..row).rev() {
                    added_cost += map[row][col] as usize;

                    if let Some(best_cost) = distance_vertical[row][col] {
                        if best_cost < added_cost {
                            continue;
                        }
                    }

                    heap.push((Reverse(added_cost), row, col, Direction::Vertical));
                    distance_vertical[row][col] = Some(added_cost);
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Direction {
    Horizontal,
    Vertical,
}
