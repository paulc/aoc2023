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
use std::time::Instant;
use util::combinations::combinations;

mod part1;
mod part2;

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

fn part1(input: &In) -> Out {
    let mut hands = input
        .iter()
        .map(|(cards, bid)| (part1::Hand::from(cards), *bid))
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
        .map(|(cards, bid)| (part2::Hand::from(cards), *bid))
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
    let p1 = Instant::now();
    println!(
        "Part1: {:?} ({}s)",
        part1(&input),
        p1.elapsed().as_secs_f32()
    );
    let p2 = Instant::now();
    println!(
        "Part2: {:?} ({}s)",
        part2(&input),
        p2.elapsed().as_secs_f32()
    );
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
