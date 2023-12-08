use crate::remove;
use std::cmp::Ordering;
use util::combinations::combinations;

const ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn same(cards: &Vec<char>) -> Option<char> {
    let first = cards.first().unwrap();
    for c in cards {
        if c != first {
            return None;
        }
    }
    Some(first.clone())
}

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
            for fours in &combinations(cards, 4) {
                if let Some(c) = same(fours) {
                    return Hand::Fours(hand);
                }
            }
            for threes in &combinations(cards, 3) {
                if let Some(c) = same(threes) {
                    let remainder = remove(cards, c);
                    if let Some(r) = same(&remainder) {
                        return Hand::FullHouse(hand);
                    } else {
                        return Hand::Threes(hand);
                    }
                }
            }
            for twos in &combinations(cards, 2) {
                if let Some(c) = same(twos) {
                    let remainder = remove(cards, c);
                    for rtwos in &combinations(&remainder, 2) {
                        if let Some(r) = same(rtwos) {
                            return Hand::TwoPairs(hand);
                        }
                    }
                    return Hand::OnePair(hand);
                }
            }
            Hand::High(hand)
        }
    }
}

impl From<&str> for Hand {
    fn from(cards: &str) -> Self {
        let c = cards.chars().collect::<Vec<char>>();
        Hand::from(&c)
    }
}
