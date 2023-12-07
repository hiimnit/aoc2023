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
    fn new(cards: &str, card_order: [char; 13], use_jokers: bool) -> Self {
        let converted_cards = cards
            .chars()
            .map(|e| {
                card_order
                    .iter()
                    .position(|c| *c == e)
                    .expect(&format!("Unexpected card {e}"))
            })
            .collect();

        Self {
            hand_type: HandType::new(&converted_cards, use_jokers),
            cards: converted_cards,
        }
    }
}

const PART_1_CARD_ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const PART_2_CARD_ORDER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

impl HandType {
    fn new(cards: &Vec<usize>, use_jokers: bool) -> Self {
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

        let hand_type = match groups[..] {
            [(_, 5)] => HandType::FiveOfAKind,
            [(_, 4), (_, 1)] => HandType::FourOfAKind,
            [(_, 3), (_, 2)] => HandType::FullHouse,
            [(_, 3), (_, 1), (_, 1)] => HandType::ThreeOfAKind,
            [(_, 2), (_, 2), (_, 1)] => HandType::TwoPairs,
            [(_, 2), (_, 1), (_, 1), (_, 1)] => HandType::OnePair,
            [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => HandType::HighCard,
            _ => panic!(),
        };

        if !use_jokers {
            return hand_type;
        }

        if let Some((_, joker_count)) = groups.iter().find(|e| *e.0 == 0) {
            return match (hand_type, joker_count) {
                (HandType::FiveOfAKind, 5) => HandType::FiveOfAKind,
                (HandType::FourOfAKind, 4) => HandType::FiveOfAKind,
                (HandType::FourOfAKind, 1) => HandType::FiveOfAKind,
                (HandType::FullHouse, 3) => HandType::FiveOfAKind,
                (HandType::FullHouse, 2) => HandType::FiveOfAKind,
                (HandType::ThreeOfAKind, 3) => HandType::FourOfAKind,
                (HandType::ThreeOfAKind, 1) => HandType::FourOfAKind,
                (HandType::TwoPairs, 2) => HandType::FourOfAKind,
                (HandType::TwoPairs, 1) => HandType::FullHouse,
                (HandType::OnePair, 2) => HandType::ThreeOfAKind,
                (HandType::OnePair, 1) => HandType::ThreeOfAKind,
                (HandType::HighCard, 1) => HandType::OnePair,
                _ => panic!(),
            };
        }

        hand_type
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.hand_type, other.hand_type) {
            (HandType::FiveOfAKind, HandType::FiveOfAKind)
            | (HandType::FourOfAKind, HandType::FourOfAKind)
            | (HandType::FullHouse, HandType::FullHouse)
            | (HandType::ThreeOfAKind, HandType::ThreeOfAKind)
            | (HandType::TwoPairs, HandType::TwoPairs)
            | (HandType::OnePair, HandType::OnePair)
            | (HandType::HighCard, HandType::HighCard) => self.cards.cmp(&other.cards),

            (HandType::FiveOfAKind, _) => Ordering::Greater,
            (_, HandType::FiveOfAKind) => Ordering::Less,

            (HandType::FourOfAKind, _) => Ordering::Greater,
            (_, HandType::FourOfAKind) => Ordering::Less,

            (HandType::FullHouse, _) => Ordering::Greater,
            (_, HandType::FullHouse) => Ordering::Less,

            (HandType::ThreeOfAKind, _) => Ordering::Greater,
            (_, HandType::ThreeOfAKind) => Ordering::Less,

            (HandType::TwoPairs, _) => Ordering::Greater,
            (_, HandType::TwoPairs) => Ordering::Less,

            (HandType::OnePair, _) => Ordering::Greater,
            (_, HandType::OnePair) => Ordering::Less,
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

    let part_1_result: usize = solve(&input, PART_1_CARD_ORDER, false);

    println!("Part 1 result {part_1_result}");

    let part_2_result: usize = solve(&input, PART_2_CARD_ORDER, true);

    println!("Part 2 result {part_2_result}");
}

fn solve(input: &str, card_order: [char; 13], use_jokers: bool) -> usize {
    let mut hands = vec![];

    for line in input.lines() {
        let mut line = line.split(' ');
        let cards = Hand::new(
            line.next().expect("Expected cards before the first space."),
            card_order,
            use_jokers,
        );
        let bid: usize = line
            .next()
            .expect("Expected bid after the first space.")
            .parse()
            .expect("Expected bid to be a usize.");

        hands.push((cards, bid));
    }

    hands.sort_by_cached_key(|e| e.0.clone());

    hands.iter().enumerate().map(|(i, e)| (i + 1) * e.1).sum()
}
