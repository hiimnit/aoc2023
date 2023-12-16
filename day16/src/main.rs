use std::{collections::HashSet, env, fmt::Write, fs};

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let map: Vec<Vec<Node>> = input
        .lines()
        .map(|line| line.chars().map(|c| Node::from(&c)).collect())
        .collect();

    if map.is_empty() {
        panic!("Map must not be empty");
    }

    let bounds = (map.len(), map[0].len());

    let part_1_result = solve(
        &map,
        &bounds,
        Beam {
            position: (0, 0),
            direction: Direction::East,
        },
    );

    println!("Part 1 result {part_1_result}");

    let part_2_result = generate_all_beams(&bounds)
        .into_iter()
        .map(|e| solve(&map, &bounds, e))
        .max()
        .expect("No beams generated");

    println!("Part 2 result {part_2_result}");
}

fn generate_all_beams(bounds: &(usize, usize)) -> Vec<Beam> {
    [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ]
    .into_iter()
    .fold(vec![], |mut acc, direction| {
        acc.extend(match direction {
            Direction::South => (0..bounds.1)
                .map(|e| Beam {
                    position: (0, e),
                    direction,
                })
                .collect::<Vec<_>>(),
            Direction::North => (0..bounds.1)
                .map(|e| Beam {
                    position: (bounds.0 - 1, e),
                    direction,
                })
                .collect::<Vec<_>>(),
            Direction::West => (0..bounds.0)
                .map(|e| Beam {
                    position: (e, bounds.1 - 1),
                    direction,
                })
                .collect::<Vec<_>>(),
            Direction::East => (0..bounds.0)
                .map(|e| Beam {
                    position: (e, 0),
                    direction,
                })
                .collect::<Vec<_>>(),
        });

        acc
    })
}

fn solve(map: &Vec<Vec<Node>>, bounds: &(usize, usize), starting_beam: Beam) -> usize {
    let mut energized_positions: HashSet<(usize, usize)> = HashSet::new();
    energized_positions.insert(starting_beam.position);

    let mut beam_queue = BeamQueue {
        beams: vec![starting_beam],
        visited_beams: HashSet::new(),
    };

    while let Some(mut beam) = beam_queue.pop() {
        loop {
            energized_positions.insert(beam.position);

            match &map[beam.position.0][beam.position.1] {
                Node::Mirror(mirror) => {
                    beam.direction = match (mirror, &beam.direction) {
                        (Mirror::Left, Direction::North) => Direction::West,
                        (Mirror::Left, Direction::West) => Direction::North,
                        (Mirror::Left, Direction::South) => Direction::East,
                        (Mirror::Left, Direction::East) => Direction::South,
                        (Mirror::Right, Direction::North) => Direction::East,
                        (Mirror::Right, Direction::West) => Direction::South,
                        (Mirror::Right, Direction::South) => Direction::West,
                        (Mirror::Right, Direction::East) => Direction::North,
                    };
                }
                Node::Splitter(splitter) => {
                    match (splitter, &beam.direction) {
                        (Splitter::Vertical, Direction::West)
                        | (Splitter::Vertical, Direction::East) => {
                            beam_queue.push(Beam {
                                position: beam.position.clone(),
                                direction: Direction::North,
                            });
                            beam_queue.push(Beam {
                                position: beam.position.clone(),
                                direction: Direction::South,
                            });
                            break;
                        }
                        (Splitter::Horizontal, Direction::North)
                        | (Splitter::Horizontal, Direction::South) => {
                            beam_queue.push(Beam {
                                position: beam.position.clone(),
                                direction: Direction::West,
                            });
                            beam_queue.push(Beam {
                                position: beam.position.clone(),
                                direction: Direction::East,
                            });
                            break;
                        }
                        (Splitter::Horizontal, Direction::West)
                        | (Splitter::Horizontal, Direction::East)
                        | (Splitter::Vertical, Direction::North)
                        | (Splitter::Vertical, Direction::South) => {}
                    };
                }
                Node::Empty => {}
            }

            let Some(next) = next_position(beam, bounds) else {
                break;
            };

            beam = next;
        }
    }

    energized_positions.len()
}

#[derive(Debug)]
struct BeamQueue {
    beams: Vec<Beam>,
    visited_beams: HashSet<Beam>,
}

impl BeamQueue {
    fn push(self: &mut Self, beam: Beam) {
        if self.visited_beams.contains(&beam) {
            return;
        }

        self.visited_beams.insert(beam.clone());
        self.beams.push(beam);
    }

    fn pop(self: &mut Self) -> Option<Beam> {
        self.beams.pop()
    }
}

fn next_position(beam: Beam, bounds: &(usize, usize)) -> Option<Beam> {
    match beam.direction {
        Direction::North => {
            if beam.position.0 == 0 {
                return None;
            }

            Some(Beam {
                direction: beam.direction,
                position: (beam.position.0 - 1, beam.position.1),
            })
        }
        Direction::West => {
            if beam.position.1 == 0 {
                return None;
            }

            Some(Beam {
                direction: beam.direction,
                position: (beam.position.0, beam.position.1 - 1),
            })
        }
        Direction::South => {
            if beam.position.0 + 1 >= bounds.0 {
                return None;
            }

            Some(Beam {
                direction: beam.direction,
                position: (beam.position.0 + 1, beam.position.1),
            })
        }
        Direction::East => {
            if beam.position.1 + 1 >= bounds.1 {
                return None;
            }

            Some(Beam {
                direction: beam.direction,
                position: (beam.position.0, beam.position.1 + 1),
            })
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    position: (usize, usize),
    direction: Direction,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug)]
enum Node {
    Mirror(Mirror),
    Splitter(Splitter),
    Empty,
}

impl Node {
    fn from(c: &char) -> Self {
        match c {
            '/' => Self::Mirror(Mirror::Right),
            '\\' => Self::Mirror(Mirror::Left),
            '-' => Self::Splitter(Splitter::Horizontal),
            '|' => Self::Splitter(Splitter::Vertical),
            '.' => Self::Empty,
            _ => panic!("Unexpected node {c}"),
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Node::Mirror(mirror) => match mirror {
                Mirror::Left => '\\',
                Mirror::Right => '/',
            },
            Node::Splitter(splitter) => match splitter {
                Splitter::Vertical => '|',
                Splitter::Horizontal => '-',
            },
            Node::Empty => ' ',
        })
    }
}

#[derive(Debug)]
enum Mirror {
    Left,
    Right,
}

#[derive(Debug)]
enum Splitter {
    Vertical,
    Horizontal,
}
