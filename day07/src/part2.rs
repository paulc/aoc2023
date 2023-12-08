use crate::remove;
use std::cmp::Ordering;
use std::fmt::Display;
use util::combinations::combinations;

fn same(cards: &Vec<char>) -> Option<char> {
    let first = cards.first().unwrap();
    if first == &'J' {
        // We ignore Jokers
        return None;
    }
    for c in cards {
        if c != first {
            return None;
        }
    }
    Some(first.clone())
}

const ORDER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Card(char);

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else {
            let index_self = ORDER.iter().position(|&c| c == self.0).unwrap();
            let index_other = ORDER.iter().position(|&c| c == other.0).unwrap();
            index_self.partial_cmp(&index_other)
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cards(Vec<Card>);

impl Ord for Cards {
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in std::iter::zip(self.0.clone(), other.0.clone()) {
            if a != b {
                return a.cmp(&b);
            }
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hand {
    High(Cards),
    OnePair(Cards),
    TwoPairs(Cards),
    Threes(Cards),
    FullHouse(Cards),
    Fours(Cards),
    Fives(Cards),
}

impl From<&Vec<char>> for Hand {
    fn from(cards: &Vec<char>) -> Self {
        let hand = Cards(cards.iter().map(|c| Card(*c)).collect::<Vec<_>>());
        if let Some(c) = same(cards) {
            return Hand::Fives(hand);
        } else {
            let n_jokers = hand.0.iter().filter(|&c| *c == Card('J')).count();
            for fours in &combinations(cards, 4) {
                if let Some(c) = same(fours) {
                    match n_jokers {
                        1 => return Hand::Fives(hand),
                        0 => return Hand::Fours(hand),
                        _ => panic!("Wrong number of Jokers"),
                    }
                }
            }
            for threes in &combinations(cards, 3) {
                if let Some(c) = same(threes) {
                    match n_jokers {
                        2 => return Hand::Fives(hand),
                        1 => return Hand::Fours(hand),
                        0 => {
                            let remainder = remove(cards, c);
                            if let Some(r) = same(&remainder) {
                                return Hand::FullHouse(hand);
                            } else {
                                return Hand::Threes(hand);
                            }
                        }
                        _ => panic!("Wrong number of Jokers"),
                    }
                }
            }
            for twos in &combinations(cards, 2) {
                if let Some(c) = same(twos) {
                    match n_jokers {
                        3 => return Hand::Fives(hand),
                        2 => return Hand::Fours(hand),
                        1 => {
                            let remainder = remove(cards, c);
                            for rtwos in &combinations(&remainder, 2) {
                                if let Some(r) = same(rtwos) {
                                    return Hand::FullHouse(hand);
                                }
                            }
                            return Hand::Threes(hand);
                        }
                        0 => {
                            let remainder = remove(cards, c);
                            for rtwos in &combinations(&remainder, 2) {
                                if let Some(r) = same(rtwos) {
                                    return Hand::TwoPairs(hand);
                                }
                            }
                            return Hand::OnePair(hand);
                        }
                        _ => panic!("Wrong number of Jokers"),
                    }
                }
            }
            return match n_jokers {
                5 => Hand::Fives(hand),
                4 => Hand::Fives(hand),
                3 => Hand::Fours(hand),
                2 => Hand::Threes(hand),
                1 => Hand::OnePair(hand),
                0 => Hand::High(hand),
                _ => panic!("Wrong number of Jokers"),
            };
        }
    }
}

impl From<&str> for Hand {
    fn from(cards: &str) -> Self {
        let c = cards.chars().collect::<Vec<char>>();
        Hand::from(&c)
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hand::Fives(c) => write!(
                f,
                "Fives({})",
                c.0.iter()
                    .map(|c| c.0.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
            Hand::Fours(c) => write!(
                f,
                "Fours({})",
                c.0.iter()
                    .map(|c| c.0.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
            Hand::FullHouse(c) => write!(
                f,
                "FullHouse({})",
                c.0.iter()
                    .map(|c| c.0.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
            Hand::Threes(c) => write!(
                f,
                "Threes({})",
                c.0.iter()
                    .map(|c| c.0.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
            Hand::TwoPairs(c) => write!(
                f,
                "TwoPairs({})",
                c.0.iter()
                    .map(|c| c.0.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
            Hand::OnePair(c) => write!(
                f,
                "OnePair({})",
                c.0.iter()
                    .map(|c| c.0.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
            Hand::High(c) => write!(
                f,
                "High({})",
                c.0.iter()
                    .map(|c| c.0.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
        }
    }
}
