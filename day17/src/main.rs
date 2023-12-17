use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
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

    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect();

    if map.is_empty() {
        panic!("Map must not be empty");
    }

    let bounds = (map.len(), map[0].len());

    let part_1_result = solve(&map, &bounds);

    println!("Part 1 result {part_1_result:?}");

    let part_2_result = solve_ultra(&map, &bounds);

    println!("Part 2 result {part_2_result:?}");
}

fn solve(map: &Vec<Vec<u8>>, bounds: &(usize, usize)) -> Option<usize> {
    let mut heap = BinaryHeap::new();

    heap.push((Reverse(0), 0, 0, Direction::Vertical));
    heap.push((Reverse(0), 0, 0, Direction::Horizontal));

    let mut distance_horizontal: Vec<Vec<Option<usize>>> = vec![vec![None; bounds.1]; bounds.0];
    distance_horizontal[0][0] = Some(0);
    let mut distance_vertical: Vec<Vec<Option<usize>>> = vec![vec![None; bounds.1]; bounds.0];
    distance_vertical[0][0] = Some(0);

    let mut visited = HashSet::new();

    while let Some((_, row, col, direction)) = heap.pop() {
        if row == bounds.0 - 1 && col == bounds.1 - 1 {
            // println!("{heap:#?}");
            return match direction {
                Direction::Horizontal => distance_horizontal[bounds.0 - 1][bounds.1 - 1],
                Direction::Vertical => distance_vertical[bounds.0 - 1][bounds.1 - 1],
            };
        }

        if !visited.insert((row, col, direction)) {
            continue;
        }

        match direction {
            Direction::Vertical => {
                let best_cost = distance_vertical[row][col].expect("Distance must not be None");

                // move right - either by one, two or three
                let mut added_cost = best_cost;
                for col in (col + 1)..(col + 4).min(bounds.1) {
                    added_cost += map[row][col] as usize;

                    if let Some(best_cost) = distance_horizontal[row][col] {
                        if best_cost < added_cost {
                            continue;
                        }
                    }

                    distance_horizontal[row][col] = Some(added_cost);
                    let estimate = added_cost + bounds.0 - row + bounds.1 - col;
                    heap.push((Reverse(estimate), row, col, Direction::Horizontal));
                }

                // move left - either by one, two or three
                let mut added_cost = best_cost;
                for col in (col.saturating_sub(3)..col).rev() {
                    added_cost += map[row][col] as usize;

                    if let Some(best_cost) = distance_horizontal[row][col] {
                        if best_cost < added_cost {
                            continue;
                        }
                    }

                    distance_horizontal[row][col] = Some(added_cost);
                    let estimate = added_cost + bounds.0 - row + bounds.1 - col;
                    heap.push((Reverse(estimate), row, col, Direction::Horizontal));
                }
            }
            Direction::Horizontal => {
                let best_cost = distance_horizontal[row][col].expect("Distance must not be None");

                // move down - either by one, two or three
                let mut added_cost = best_cost;
                for row in (row + 1)..(row + 4).min(bounds.0) {
                    added_cost += map[row][col] as usize;

                    if let Some(best_cost) = distance_vertical[row][col] {
                        if best_cost < added_cost {
                            continue;
                        }
                    }

                    distance_vertical[row][col] = Some(added_cost);
                    let estimate = added_cost + bounds.0 - row + bounds.1 - col;
                    heap.push((Reverse(estimate), row, col, Direction::Vertical));
                }

                // move up - either by one, two or three
                let mut added_cost = best_cost;
                for row in (row.saturating_sub(3)..row).rev() {
                    added_cost += map[row][col] as usize;

                    if let Some(best_cost) = distance_vertical[row][col] {
                        if best_cost < added_cost {
                            continue;
                        }
                    }

                    distance_vertical[row][col] = Some(added_cost);
                    let estimate = added_cost + bounds.0 - row + bounds.1 - col;
                    heap.push((Reverse(estimate), row, col, Direction::Vertical));
                }
            }
        }
    }

    None
}

fn solve_ultra(map: &Vec<Vec<u8>>, bounds: &(usize, usize)) -> Option<usize> {
    let mut heap = BinaryHeap::new();

    heap.push((Reverse(0), 0, 0, Direction::Vertical));
    heap.push((Reverse(0), 0, 0, Direction::Horizontal));

    let mut distance_horizontal: Vec<Vec<Option<usize>>> = vec![vec![None; bounds.1]; bounds.0];
    distance_horizontal[0][0] = Some(0);
    let mut distance_vertical: Vec<Vec<Option<usize>>> = vec![vec![None; bounds.1]; bounds.0];
    distance_vertical[0][0] = Some(0);

    let mut visited = HashSet::new();

    while let Some((_, row, col, direction)) = heap.pop() {
        if row == bounds.0 - 1 && col == bounds.1 - 1 {
            // println!("{heap:#?}");
            return match direction {
                Direction::Horizontal => distance_horizontal[bounds.0 - 1][bounds.1 - 1],
                Direction::Vertical => distance_vertical[bounds.0 - 1][bounds.1 - 1],
            };
        }

        if !visited.insert((row, col, direction)) {
            continue;
        }

        match direction {
            Direction::Vertical => {
                let best_cost = distance_vertical[row][col].expect("Distance must not be None");

                // move right - by at least 4 and at most 10
                if col + 4 < bounds.1 {
                    let mut added_cost = best_cost
                        + map[row][col + 1..col + 4]
                            .iter()
                            .fold(0, |acc, &e| acc + e as usize);
                    for col in (col + 4)..(col + 11).min(bounds.1) {
                        added_cost += map[row][col] as usize;

                        if let Some(best_cost) = distance_horizontal[row][col] {
                            if best_cost < added_cost {
                                continue;
                            }
                        }

                        distance_horizontal[row][col] = Some(added_cost);
                        let estimate = added_cost + bounds.0 - row + bounds.1 - col;
                        heap.push((Reverse(estimate), row, col, Direction::Horizontal));
                    }
                }

                // move left - by at least 4 and at most 10
                if col > 3 {
                    let mut added_cost = best_cost
                        + map[row][col - 3..col]
                            .iter()
                            .fold(0, |acc, &e| acc + e as usize);
                    for col in (col.saturating_sub(10)..col.saturating_sub(3)).rev() {
                        added_cost += map[row][col] as usize;

                        if let Some(best_cost) = distance_horizontal[row][col] {
                            if best_cost < added_cost {
                                continue;
                            }
                        }

                        distance_horizontal[row][col] = Some(added_cost);
                        let estimate = added_cost + bounds.0 - row + bounds.1 - col;
                        heap.push((Reverse(estimate), row, col, Direction::Horizontal));
                    }
                }
            }
            Direction::Horizontal => {
                let best_cost = distance_horizontal[row][col].expect("Distance must not be None");

                // move down - by at least 4 and at most 10
                if row + 4 < bounds.0 {
                    let mut added_cost = best_cost
                        + map[row + 1..row + 4]
                            .iter()
                            .fold(0, |acc, e| acc + e[col] as usize);
                    for row in (row + 4)..(row + 11).min(bounds.0) {
                        added_cost += map[row][col] as usize;

                        if let Some(best_cost) = distance_vertical[row][col] {
                            if best_cost < added_cost {
                                continue;
                            }
                        }

                        distance_vertical[row][col] = Some(added_cost);
                        let estimate = added_cost + bounds.0 - row + bounds.1 - col;
                        heap.push((Reverse(estimate), row, col, Direction::Vertical));
                    }
                }

                // move up - by at least 4 and at most 10
                if row > 3 {
                    let mut added_cost = best_cost
                        + map[row - 3..row]
                            .iter()
                            .fold(0, |acc, e| acc + e[col] as usize);
                    for row in (row.saturating_sub(10)..row.saturating_sub(3)).rev() {
                        added_cost += map[row][col] as usize;

                        if let Some(best_cost) = distance_vertical[row][col] {
                            if best_cost < added_cost {
                                continue;
                            }
                        }

                        distance_vertical[row][col] = Some(added_cost);
                        let estimate = added_cost + bounds.0 - row + bounds.1 - col;
                        heap.push((Reverse(estimate), row, col, Direction::Vertical));
                    }
                }
            }
        }
    }

    None
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
enum Direction {
    Horizontal,
    Vertical,
}
