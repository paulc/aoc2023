#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;

type In = Vec<(usize, (HashSet<u32>, HashSet<u32>))>;
type Out = u32;
const PART1_RESULT: Out = 13;
const PART2_RESULT: Out = 30;

fn parse_input(input: &mut impl Read) -> In {
    let data = BufReader::new(input)
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (l, r) = l.split_once("|").unwrap();
            (
                l.split_whitespace()
                    .skip(2)
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<HashSet<u32>>(),
                r.split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<HashSet<u32>>(),
            )
        })
        .enumerate()
        .collect::<Vec<_>>();
    data
}

fn part1(input: &In) -> Out {
    input
        .iter()
        .map(
            |(_, (win, numbers))| match win.intersection(numbers).count() {
                0 => 0,
                1 => 1,
                n => 2u32.pow(n as u32 - 1),
            },
        )
        .sum()
}

fn part2(input: &In) -> Out {
    let mut cards: Vec<usize> = (0..input.len()).map(|_| 1).collect();
    input.iter().for_each(|(n, (win, numbers))| {
        let n_win = win.intersection(numbers).count();
        let n_current = cards.get(*n).unwrap().clone();
        (n + 1..n + n_win + 1).for_each(|i| {
            if let Some(count) = cards.get_mut(i) {
                *count += n_current;
            }
        });
    });
    cards.iter().sum::<usize>() as u32
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
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
