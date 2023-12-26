#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::time::Instant;

type In = Vec<Vec<u8>>;
type Out = u32;
const PART1_RESULT: Out = 142;
const PART2_RESULT: Out = 281;

fn parse_input1(input: &mut impl Read) -> In {
    let data: In = BufReader::new(input)
        .lines()
        .map(|l| l.unwrap().into_bytes().iter().cloned().collect::<Vec<_>>())
        .collect();
    data
}

fn parse_input2(input: &mut impl Read) -> In {
    let data: In = BufReader::new(input)
        .lines()
        .map(|l| {
            l.unwrap()
                // Handle concatenated numbers first
                .replace("oneight", "18")
                .replace("twone", "21")
                .replace("threeight", "38")
                .replace("fiveight", "58")
                .replace("sevenine", "79")
                .replace("eightwo", "82")
                .replace("eighthree", "83")
                .replace("nineight", "98")
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9")
        })
        .map(|l| l.into_bytes().iter().cloned().collect::<Vec<_>>())
        .collect();
    data
}

fn part1(input: &In) -> Out {
    input
        .iter()
        .map(|l| {
            l.iter()
                .filter_map(|b| match b {
                    b'0'..=b'9' => Some(*b - b'0'),
                    _ => None,
                })
                .collect::<Vec<u8>>()
        })
        .map(|l| (*l.first().unwrap() as u32) * 10 + (*l.last().unwrap() as u32))
        .sum()
}

fn part2(input: &In) -> Out {
    part1(input)
}

fn main() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;
    let input1 = parse_input1(&mut f);
    f = File::open("input.txt")?;
    let input2 = parse_input2(&mut f);
    let p1 = Instant::now();
    println!(
        "Part1: {:?} ({}s)",
        part1(&input1),
        p1.elapsed().as_secs_f32()
    );
    let p2 = Instant::now();
    println!(
        "Part2: {:?} ({}s)",
        part2(&input1),
        p2.elapsed().as_secs_f32()
    );
    Ok(())
}

#[test]
fn test_part1() {
    let input = parse_input1(&mut TESTDATA1.trim_matches('\n').as_bytes());
    assert_eq!(part1(&input), PART1_RESULT);
}

#[test]
fn test_part2() {
    let input = parse_input2(&mut TESTDATA2.trim_matches('\n').as_bytes());
    assert_eq!(part2(&input), PART2_RESULT);
}

#[cfg(test)]
const TESTDATA1: &str = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

#[cfg(test)]
const TESTDATA2: &str = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
