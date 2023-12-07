#![allow(unused)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use util::combinations::combinations;

type In = Vec<(Vec<char>, u32)>;
type Out = u32;
const PART1_RESULT: Out = 6440;
const PART2_RESULT: Out = 5905;

fn parse_input(input: &mut impl Read) -> In {
    let data = BufReader::new(input)
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (h, b) = l.split_once(" ").unwrap();
            let mut cards = h.chars().collect::<Vec<_>>();
            let bid = b.parse::<u32>().unwrap();
            (cards, bid)
        })
        .collect::<Vec<_>>();
    data
}

fn remove(cards: &Vec<char>, card: char) -> Vec<char> {
    cards
        .iter()
        .cloned()
        .filter(|&c| c != card)
        .collect::<Vec<_>>()
}

fn same(cards: &Vec<char>) -> Option<char> {
    let first = cards.first().unwrap();
    for c in cards {
        if c != first {
            return None;
        }
    }
    Some(first.clone())
}

/*
 * PART1
 */

const ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Card(char);

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
struct Cards(Vec<Card>);

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
enum Hand {
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

/*
 * To keep things simple we  just replicate for PART2 with different rules
 */

fn same2(cards: &Vec<char>) -> Option<char> {
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

const ORDER2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Card2(char);

impl PartialOrd for Card2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else {
            let index_self = ORDER2.iter().position(|&c| c == self.0).unwrap();
            let index_other = ORDER2.iter().position(|&c| c == other.0).unwrap();
            index_self.partial_cmp(&index_other)
        }
    }
}

impl Ord for Card2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cards2(Vec<Card2>);

impl Ord for Cards2 {
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in std::iter::zip(self.0.clone(), other.0.clone()) {
            if a != b {
                return a.cmp(&b);
            }
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Cards2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Hand2 {
    High(Cards2),
    OnePair(Cards2),
    TwoPairs(Cards2),
    Threes(Cards2),
    FullHouse(Cards2),
    Fours(Cards2),
    Fives(Cards2),
}

impl From<&Vec<char>> for Hand2 {
    fn from(cards: &Vec<char>) -> Self {
        let hand = Cards2(cards.iter().map(|c| Card2(*c)).collect::<Vec<_>>());
        if let Some(c) = same2(cards) {
            return Hand2::Fives(hand);
        } else {
            let n_jokers = hand.0.iter().filter(|&c| *c == Card2('J')).count();
            for fours in &combinations(cards, 4) {
                if let Some(c) = same2(fours) {
                    match n_jokers {
                        1 => return Hand2::Fives(hand),
                        _ => return Hand2::Fours(hand),
                    }
                }
            }
            for threes in &combinations(cards, 3) {
                if let Some(c) = same2(threes) {
                    match n_jokers {
                        2 => return Hand2::Fives(hand),
                        1 => return Hand2::Fours(hand),
                        _ => {
                            let remainder = remove(cards, c);
                            if let Some(r) = same2(&remainder) {
                                return Hand2::FullHouse(hand);
                            } else {
                                return Hand2::Threes(hand);
                            }
                        }
                    }
                }
            }
            for twos in &combinations(cards, 2) {
                if let Some(c) = same2(twos) {
                    match n_jokers {
                        3 => return Hand2::Fives(hand),
                        2 => return Hand2::Fours(hand),
                        1 => {
                            let remainder = remove(cards, c);
                            for rtwos in &combinations(&remainder, 2) {
                                if let Some(r) = same2(rtwos) {
                                    return Hand2::FullHouse(hand);
                                }
                            }
                        }
                        _ => {
                            let remainder = remove(cards, c);
                            for rtwos in &combinations(&remainder, 2) {
                                if let Some(r) = same2(rtwos) {
                                    return Hand2::FullHouse(hand);
                                }
                            }
                            return Hand2::OnePair(hand);
                        }
                    }
                }
            }
            return match n_jokers {
                5 => Hand2::Fives(hand),
                4 => Hand2::Fives(hand),
                3 => Hand2::Fours(hand),
                2 => Hand2::Threes(hand),
                1 => Hand2::OnePair(hand),
                _ => Hand2::High(hand),
            };
        }
    }
}

impl From<&str> for Hand2 {
    fn from(cards: &str) -> Self {
        let c = cards.chars().collect::<Vec<char>>();
        Hand2::from(&c)
    }
}

impl Display for Hand2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hand2::Fives(c) => write!(
                f,
                "Fives({})",
                c.0.iter()
                    .map(|c| c.0.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
            Hand2::Fours(c) => write!(
                f,
                "Fours({})",
                c.0.iter()
                    .map(|c| c.0.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
            Hand2::FullHouse(c) => write!(
                f,
                "FullHouse({})",
                c.0.iter()
                    .map(|c| c.0.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
            Hand2::Threes(c) => write!(
                f,
                "Threes({})",
                c.0.iter()
                    .map(|c| c.0.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
            Hand2::TwoPairs(c) => write!(
                f,
                "TwoPairs({})",
                c.0.iter()
                    .map(|c| c.0.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
            Hand2::OnePair(c) => write!(
                f,
                "OnePair({})",
                c.0.iter()
                    .map(|c| c.0.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
            Hand2::High(c) => write!(
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

fn part1(input: &In) -> Out {
    let mut hands = input
        .iter()
        .map(|(cards, bid)| (Hand::from(cards), *bid))
        .collect::<Vec<_>>();
    hands.sort_by_key(|(c, _)| c.clone());
    let winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, b))| (i + 1) as u32 * *b)
        .sum();
    winnings
}

fn part2(input: &In) -> Out {
    let mut hands = input
        .iter()
        .map(|(cards, bid)| (Hand2::from(cards), *bid))
        .collect::<Vec<_>>();
    hands.sort_by_key(|(c, _)| c.clone());
    let winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, b))| (i + 1) as u32 * *b)
        .sum();
    winnings
}

fn main() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;
    let input = parse_input(&mut f);
    println!("Part1: {:?}", part1(&input));
    println!("Part2: {:?}", part2(&input));
    Ok(())
}

#[test]
fn test_part1() {
    let input = parse_input(&mut TESTDATA.trim_matches('\n').as_bytes());
    assert_eq!(part1(&input), PART1_RESULT);
}

#[test]
fn test_part2() {
    let input = parse_input(&mut TESTDATA.trim_matches('\n').as_bytes());
    assert_eq!(part2(&input), PART2_RESULT);
}

#[cfg(test)]
const TESTDATA: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
