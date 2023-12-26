#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::iter::zip;
use std::time::Instant;

type In = Vec<(u64, u64)>;
type Out = u64;
const PART1_RESULT: Out = 288;
const PART2_RESULT: Out = 71503;

fn parse_input1(input: &mut impl Read) -> In {
    let mut i = BufReader::new(input).lines();
    let times = i
        .next()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .filter_map(|w| w.parse::<u64>().ok())
                .collect::<Vec<_>>()
        })
        .unwrap();
    let distance = i
        .next()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .filter_map(|w| w.parse::<u64>().ok())
                .collect::<Vec<_>>()
        })
        .unwrap();
    zip(times, distance).collect::<Vec<_>>()
}

fn parse_input2(input: &mut impl Read) -> In {
    let mut i = BufReader::new(input).lines();
    let times = i
        .next()
        .map(|l| {
            l.unwrap()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .fold(0_u64, |acc, d| acc * 10 + d as u64)
        })
        .unwrap();
    let distance = i
        .next()
        .map(|l| {
            l.unwrap()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .fold(0_u64, |acc, d| acc * 10 + d as u64)
        })
        .unwrap();
    vec![(times, distance)]
}

fn part1(input: &In) -> Out {
    let mut result: Vec<u64> = Vec::new();
    for (time, distance) in input {
        result.push(0);
        for h in 1..*time {
            if h * (time - h) > *distance {
                let mut r = result.last_mut().unwrap();
                *r = *r + 1;
            }
        }
    }
    result.iter().fold(1_u64, |acc, x| acc * x)
}

fn part2(input: &In) -> Out {
    let (time, distance) = input.get(0).unwrap();
    let mut result = 0;
    for h in 1..*time {
        if h * (time - h) > *distance {
            result += 1;
        }
    }
    result
}

fn main() -> std::io::Result<()> {
    let mut f1 = File::open("input.txt")?;
    let input1 = parse_input1(&mut f1);
    let mut f2 = File::open("input.txt")?;
    let input2 = parse_input2(&mut f2);
    let p1 = Instant::now();
    println!(
        "Part1: {:?} ({}s)",
        part1(&input1),
        p1.elapsed().as_secs_f32()
    );
    let p2 = Instant::now();
    println!(
        "Part2: {:?} ({}s)",
        part2(&input2),
        p2.elapsed().as_secs_f32()
    );
    Ok(())
}

#[test]
fn test_part1() {
    let input = parse_input1(&mut TESTDATA.trim_matches('\n').as_bytes());
    assert_eq!(part1(&input), PART1_RESULT);
}

#[test]
fn test_part2() {
    let input = parse_input2(&mut TESTDATA.trim_matches('\n').as_bytes());
    assert_eq!(part2(&input), PART2_RESULT);
}

#[cfg(test)]
const TESTDATA: &str = "
Time:      7  15   30
Distance:  9  40  200
";
