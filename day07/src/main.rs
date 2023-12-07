use std::{cmp::Ordering, env, fs, vec};

use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum HandType {
    FiveOfAKind(usize),
    FourOfAKind(usize, usize),
    FullHouse(usize, usize),
    ThreeOfAKind(usize, usize, usize),
    TwoPairs(usize, usize, usize),
    OnePair(usize, usize, usize, usize),
    HighCard(usize, usize, usize, usize, usize),
}

const CARDS: [char; 13] = [
    // 'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

impl HandType {
    fn new(cards: &str) -> Self {
        println!("{cards}");

        let temp = cards.chars().map(|e| {
            CARDS
                .iter()
                .position(|c| *c == e)
                .expect(&format!("Unexpected card {e}"))
        });

        let mut groups: Vec<_> = temp
            .sorted()
            .group_by(|e| e.clone())
            .into_iter()
            .map(|(k, v)| (k, v.count()))
            .collect();

        groups.sort_by(|a, b| match a.1.cmp(&b.1) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => b.0.cmp(&a.0),
            Ordering::Greater => Ordering::Less,
        });

        println!("{groups:?}");

        match groups[..] {
            [(k, 5)] => HandType::FiveOfAKind(k),
            [(k1, 4), (k2, 1)] => HandType::FourOfAKind(k1, k2),
            [(k1, 3), (k2, 2)] => HandType::FullHouse(k1, k2),
            [(k1, 3), (k2, 1), (k3, 1)] => HandType::ThreeOfAKind(k1, k2, k3),
            [(k1, 2), (k2, 2), (k3, 1)] => HandType::TwoPairs(k1, k2, k3),
            [(k1, 2), (k2, 1), (k3, 1), (k4, 1)] => HandType::OnePair(k1, k2, k3, k4),
            [(k1, 1), (k2, 1), (k3, 1), (k4, 1), (k5, 1)] => HandType::HighCard(k1, k2, k3, k4, k5),
            _ => panic!(),
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (HandType::FiveOfAKind(a), HandType::FiveOfAKind(b)) => a.cmp(b),
            (HandType::FiveOfAKind(_), _) => Ordering::Greater,
            (_, HandType::FiveOfAKind(_)) => Ordering::Less,

            (HandType::FourOfAKind(a1, a2), HandType::FourOfAKind(b1, b2)) => {
                (a1, a2).cmp(&(b1, b2))
            }
            (HandType::FourOfAKind(_, _), _) => Ordering::Greater,
            (_, HandType::FourOfAKind(_, _)) => Ordering::Less,

            (HandType::FullHouse(a1, a2), HandType::FullHouse(b1, b2)) => (a1, a2).cmp(&(b1, b2)),
            (HandType::FullHouse(_, _), _) => Ordering::Greater,
            (_, HandType::FullHouse(_, _)) => Ordering::Less,

            (HandType::ThreeOfAKind(a1, a2, a3), HandType::ThreeOfAKind(b1, b2, b3)) => {
                (a1, a2, a3).cmp(&(b1, b2, b3))
            }
            (HandType::ThreeOfAKind(_, _, _), _) => Ordering::Greater,
            (_, HandType::ThreeOfAKind(_, _, _)) => Ordering::Less,

            (HandType::TwoPairs(a1, a2, a3), HandType::TwoPairs(b1, b2, b3)) => {
                (a1, a2, a3).cmp(&(b1, b2, b3))
            }
            (HandType::TwoPairs(_, _, _), _) => Ordering::Greater,
            (_, HandType::TwoPairs(_, _, _)) => Ordering::Less,

            (HandType::OnePair(a1, a2, a3, a4), HandType::OnePair(b1, b2, b3, b4)) => {
                (a1, a2, a3, a4).cmp(&(b1, b2, b3, b4))
            }
            (HandType::OnePair(_, _, _, _), _) => Ordering::Greater,
            (_, HandType::OnePair(_, _, _, _)) => Ordering::Less,

            (HandType::HighCard(a1, a2, a3, a4, a5), HandType::HighCard(b1, b2, b3, b4, b5)) => {
                (a1, a2, a3, a4, a5).cmp(&(b1, b2, b3, b4, b5))
            }
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HandType::FiveOfAKind(a), HandType::FiveOfAKind(b)) => a == b,
            (HandType::FourOfAKind(a1, a2), HandType::FourOfAKind(b1, b2)) => a1 == b1 && a2 == b2,
            (HandType::FullHouse(a1, a2), HandType::FullHouse(b1, b2)) => a1 == b1 && a2 == b2,
            (HandType::ThreeOfAKind(a1, a2, a3), HandType::ThreeOfAKind(b1, b2, b3)) => {
                a1 == b1 && a2 == b2 && a3 == b3
            }
            (HandType::TwoPairs(a1, a2, a3), HandType::TwoPairs(b1, b2, b3)) => {
                a1 == b1 && a2 == b2 && a3 == b3
            }
            (HandType::OnePair(a1, a2, a3, a4), HandType::OnePair(b1, b2, b3, b4)) => {
                a1 == b1 && a2 == b2 && a3 == b3 && a4 == b4
            }
            (HandType::HighCard(a1, a2, a3, a4, a5), HandType::HighCard(b1, b2, b3, b4, b5)) => {
                a1 == b1 && a2 == b2 && a3 == b3 && a4 == b4 && a5 == b5
            }
            _ => false,
        }
    }
}

impl Eq for HandType {}

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let mut hands = vec![];

    for line in input.lines() {
        let mut line = line.split(' ');
        let cards = HandType::new(line.next().expect("Expected cards before the first space."));
        println!("{cards:?}");
        let bid: usize = line
            .next()
            .expect("Expected bid after the first space.")
            .parse()
            .expect("Expected bid to be a usize.");

        hands.push((cards, bid));
    }

    hands.sort_by_key(|e| e.0);

    let part_1_result: usize = hands.iter().enumerate().map(|(i, e)| (i + 1) * e.1).sum();

    for hand in hands {
        println!("{hand:?}");
    }

    println!("Part 1 result {part_1_result}");
}
