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

type In = ();
type Out = u32;
const PART1_RESULT: Out = 0;
const PART2_RESULT: Out = 0;

fn parse_input(input: &mut impl Read) -> In {
    let data = BufReader::new(input).lines().collect::<Vec<_>>();
    ()
}

fn part1(input: &In) -> Out {
    PART1_RESULT
}

fn part2(input: &In) -> Out {
    PART2_RESULT
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
";
