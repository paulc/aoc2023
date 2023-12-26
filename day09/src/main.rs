#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::time::Instant;

type In = Vec<Vec<i32>>;
type Out = i32;
const PART1_RESULT: Out = 114;
const PART2_RESULT: Out = 2;

fn parse_input(input: &mut impl Read) -> In {
    BufReader::new(input)
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn extrapolate(line: &Vec<i32>) -> i32 {
    let mut stack: Vec<Vec<i32>> = vec![line.clone()];
    while stack.last().unwrap().iter().any(|&i| i != 0) {
        let last = stack.last().unwrap();
        stack.push(
            last.iter()
                .zip(last.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect::<Vec<_>>(),
        );
    }
    let mut add: i32 = 0;
    for i in (0..stack.len()).rev() {
        let next = stack[i].last().unwrap() + add;
        stack.get_mut(i).unwrap().push(next);
        add = next;
    }
    stack[0].last().unwrap().clone()
}

fn predict(line: &Vec<i32>) -> i32 {
    let mut stack: Vec<VecDeque<i32>> = vec![VecDeque::from(line.clone())];
    while stack.last().unwrap().iter().any(|&i| i != 0) {
        let last = stack.last().unwrap();
        stack.push(
            last.iter()
                .zip(last.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect::<VecDeque<_>>(),
        );
    }
    let mut front: i32 = 0;
    for i in (0..stack.len()).rev() {
        let front_new = stack[i].front().unwrap() - front;
        stack.get_mut(i).unwrap().push_front(front_new);
        front = front_new;
    }
    stack[0].front().unwrap().clone()
}

fn part1(input: &In) -> Out {
    input.iter().map(|l| extrapolate(l)).sum()
}

fn part2(input: &In) -> Out {
    input.iter().map(|l| predict(l)).sum()
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
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
