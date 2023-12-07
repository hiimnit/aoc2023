use std::{cmp::Ordering, env, fs, vec};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Hand {
    hand_type: HandType,
    cards: Vec<usize>,
}

#[derive(Debug, Copy, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl Hand {
    fn new(cards: &str) -> Self {
        let converted_cards = cards
            .chars()
            .map(|e| {
                CARDS
                    .iter()
                    .position(|c| *c == e)
                    .expect(&format!("Unexpected card {e}"))
            })
            .collect();

        Self {
            hand_type: HandType::new(&converted_cards),
            cards: converted_cards,
        }
    }
}

const CARDS: [char; 13] = [
    // 'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

impl HandType {
    fn new(cards: &Vec<usize>) -> Self {
        let mut groups: Vec<_> = cards
            .iter()
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
            [(k, 5)] => HandType::FiveOfAKind,
            [(k1, 4), (k2, 1)] => HandType::FourOfAKind,
            [(k1, 3), (k2, 2)] => HandType::FullHouse,
            [(k1, 3), (k2, 1), (k3, 1)] => HandType::ThreeOfAKind,
            [(k1, 2), (k2, 2), (k3, 1)] => HandType::TwoPairs,
            [(k1, 2), (k2, 1), (k3, 1), (k4, 1)] => HandType::OnePair,
            [(k1, 1), (k2, 1), (k3, 1), (k4, 1), (k5, 1)] => HandType::HighCard,
            _ => panic!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.hand_type, other.hand_type) {
            (HandType::FiveOfAKind, HandType::FiveOfAKind) => self.cards.cmp(&other.cards),
            (HandType::FiveOfAKind, _) => Ordering::Greater,
            (_, HandType::FiveOfAKind) => Ordering::Less,

            (HandType::FourOfAKind, HandType::FourOfAKind) => self.cards.cmp(&other.cards),
            (HandType::FourOfAKind, _) => Ordering::Greater,
            (_, HandType::FourOfAKind) => Ordering::Less,

            (HandType::FullHouse, HandType::FullHouse) => self.cards.cmp(&other.cards),
            (HandType::FullHouse, _) => Ordering::Greater,
            (_, HandType::FullHouse) => Ordering::Less,

            (HandType::ThreeOfAKind, HandType::ThreeOfAKind) => self.cards.cmp(&other.cards),
            (HandType::ThreeOfAKind, _) => Ordering::Greater,
            (_, HandType::ThreeOfAKind) => Ordering::Less,

            (HandType::TwoPairs, HandType::TwoPairs) => self.cards.cmp(&other.cards),
            (HandType::TwoPairs, _) => Ordering::Greater,
            (_, HandType::TwoPairs) => Ordering::Less,

            (HandType::OnePair, HandType::OnePair) => self.cards.cmp(&other.cards),
            (HandType::OnePair, _) => Ordering::Greater,
            (_, HandType::OnePair) => Ordering::Less,

            (HandType::HighCard, HandType::HighCard) => self.cards.cmp(&other.cards),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        match (self.hand_type, other.hand_type) {
            (HandType::FiveOfAKind, HandType::FiveOfAKind) => self.cards.eq(&other.cards),
            (HandType::FourOfAKind, HandType::FourOfAKind) => self.cards.eq(&other.cards),
            (HandType::FullHouse, HandType::FullHouse) => self.cards.eq(&other.cards),
            (HandType::ThreeOfAKind, HandType::ThreeOfAKind) => self.cards.eq(&other.cards),
            (HandType::TwoPairs, HandType::TwoPairs) => self.cards.eq(&other.cards),
            (HandType::OnePair, HandType::OnePair) => self.cards.eq(&other.cards),
            (HandType::HighCard, HandType::HighCard) => self.cards.eq(&other.cards),
            _ => false,
        }
    }
}

impl Eq for Hand {}

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
        let cards = Hand::new(line.next().expect("Expected cards before the first space."));
        println!("{cards:?}");
        let bid: usize = line
            .next()
            .expect("Expected bid after the first space.")
            .parse()
            .expect("Expected bid to be a usize.");

        hands.push((cards, bid));
    }

    hands.sort_by_cached_key(|e| e.0.clone());

    let part_1_result: usize = hands.iter().enumerate().map(|(i, e)| (i + 1) * e.1).sum();

    for hand in hands {
        println!("{hand:?}");
    }

    println!("Part 1 result {part_1_result}");
}
