use std::{
    collections::{HashMap, HashSet},
    env, fs, vec,
};

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
        .map(|e| e.chars().map(|e| e.into()).collect())
        .collect();

    if map.is_empty() {
        panic!("Unexpected empty map");
    }

    let start = map
        .first()
        .map(|e| e.iter().position(|e| *e == Node::Empty))
        .expect("Start not found")
        .expect("Start not found");

    let end = map
        .last()
        .map(|e| e.iter().position(|e| *e == Node::Empty))
        .expect("End not found")
        .expect("End not found");

    let visited: HashSet<Position> = HashSet::new();
    let part_1_result = find_longest_path(&map, &(map.len() - 1, end), (0, start), visited);

    println!("Part 1 result {part_1_result:?}");

    let mut graph = HashMap::new();
    graph.insert((0, start), vec![]);

    build_graph(&map, &(map.len() - 1, end), &(0, start), &mut graph);

    let visited: HashSet<Position> = HashSet::new();
    let part_2_result = find_longest_graph_path(&graph, &(map.len() - 1, end), (0, start), visited);

    println!("Part 2 result {part_2_result:?}");
}

type Map = Vec<Vec<Node>>;
type Position = (usize, usize);
type Graph = HashMap<Position, Vec<(Position, usize)>>;

fn find_longest_path(
    map: &Map,
    end: &Position,
    position: Position,
    mut visited: HashSet<Position>,
) -> Option<usize> {
    if position.0 == end.0 && position.1 == end.1 {
        return Some(0);
    }

    visited.insert(position);

    let next_steps = next_steps(map, &position, &visited, true);

    if next_steps.len() == 1 {
        let &(_, position) = next_steps.first().unwrap();
        return find_longest_path(map, end, position, visited).map(|e| e + 1);
    }

    next_steps
        .into_iter()
        .filter_map(|(_, position)| {
            let visited = visited.clone();
            find_longest_path(map, end, position, visited)
        })
        .max()
        .map(|e| e + 1)
}

fn build_graph(map: &Map, end: &Position, start: &Position, graph: &mut Graph) {
    let mut queue = vec![start.clone()];
    graph.insert(start.clone(), vec![]);
    let mut visited = HashSet::new();
    visited.insert(start.clone());

    while let Some(junction) = queue.pop() {
        visited.insert(junction);

        for (_, position) in next_steps(map, &junction, &visited, false) {
            if let Some((next_junction, distance)) =
                get_next_junction(map, end, position, 1, &mut visited, graph)
            {
                queue.push(next_junction);
                if let Some(neighbors) = graph.get_mut(&next_junction) {
                    neighbors.push((junction, distance));
                } else {
                    graph.insert(next_junction, vec![(junction, distance)]);
                }

                graph
                    .get_mut(&junction)
                    .unwrap()
                    .push((next_junction, distance));

                visited.remove(&next_junction);
            }
        }

        visited.remove(&junction);
    }
}

fn get_next_junction(
    map: &Map,
    end: &Position,
    position: Position,
    distance: usize,
    visited: &mut HashSet<Position>,
    graph: &Graph,
) -> Option<(Position, usize)> {
    if position.0 == end.0 && position.1 == end.1 {
        return Some((end.clone(), distance));
    }

    if graph.contains_key(&position) {
        return Some((position, distance));
    }

    visited.insert(position);

    let next_steps = next_steps(map, &position, visited, false);

    match next_steps.len() {
        0 => None,
        1 => {
            let &(_, position) = next_steps.first().unwrap();
            get_next_junction(map, end, position, distance + 1, visited, graph)
        }
        _ => Some((position, distance)),
    }
}

fn find_longest_graph_path(
    graph: &Graph,
    end: &Position,
    position: Position,
    mut visited: HashSet<Position>,
) -> Option<usize> {
    if position == *end {
        return Some(0);
    }

    visited.insert(position);

    graph[&position]
        .iter()
        .filter_map(|(position, distance)| {
            if visited.contains(position) {
                return None;
            }
            let visited = visited.clone();
            find_longest_graph_path(graph, end, *position, visited).map(|e| e + distance)
        })
        .max()
}

fn next_steps(
    map: &Map,
    position: &Position,
    visited: &HashSet<Position>,
    compare_direction: bool,
) -> Vec<(Direction, Position)> {
    let mut result = vec![];

    if position.0 > 0 {
        add_position(
            map,
            visited,
            &mut result,
            (position.0 - 1, position.1),
            compare_direction,
            Direction::Up,
        );
    }

    if position.1 + 1 < map[position.0].len() {
        add_position(
            map,
            visited,
            &mut result,
            (position.0, position.1 + 1),
            compare_direction,
            Direction::Right,
        );
    }

    if position.0 + 1 < map.len() {
        add_position(
            map,
            visited,
            &mut result,
            (position.0 + 1, position.1),
            compare_direction,
            Direction::Down,
        );
    }

    if position.1 > 0 {
        add_position(
            map,
            visited,
            &mut result,
            (position.0, position.1 - 1),
            compare_direction,
            Direction::Left,
        );
    }

    result
}

fn add_position(
    map: &Map,
    visited: &HashSet<Position>,
    next_positions: &mut Vec<(Direction, Position)>,
    position: Position,
    compare_direction: bool,
    direction: Direction,
) {
    if !can_go_to(&map[position.0][position.1], compare_direction, direction) {
        return;
    }

    if visited.contains(&position) {
        return;
    }

    next_positions.push((direction, position))
}

fn can_go_to(node: &Node, compare_direction: bool, from_direction: Direction) -> bool {
    match node {
        Node::Wall => false,
        Node::Empty => true,
        Node::Slope(slope_direction) => !compare_direction || *slope_direction == from_direction,
    }
}

#[derive(Debug, PartialEq)]
enum Node {
    Empty,
    Wall,
    Slope(Direction),
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '^' => Self::Slope(Direction::Up),
            '>' => Self::Slope(Direction::Right),
            'v' => Self::Slope(Direction::Down),
            '<' => Self::Slope(Direction::Left),
            _ => panic!("Unexpected node {value}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
