#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;

type In = Vec<Vec<Vec<char>>>;
type Out = usize;
const PART1_RESULT: Out = 405;
const PART2_RESULT: Out = 0;

fn parse_input(input: &mut impl Read) -> In {
    let mut out: Vec<Vec<Vec<char>>> = vec![];
    let mut grid: Vec<Vec<char>> = vec![];
    BufReader::new(input).lines().for_each(|l| {
        let l = l.unwrap();
        if l.is_empty() {
            out.push(grid.clone());
            grid.clear();
        } else {
            grid.push(l.chars().collect::<Vec<_>>())
        }
    });
    out.push(grid.clone());
    out
}

fn check_reflection(v: &Vec<usize>) -> Option<usize> {
    let len = v.len();
    for i in (1..len) {
        let l = &v[(if i > len / 2 { i - (len - i) } else { 0 })..i];
        let r = &mut v.clone()[i..(i + i).min(v.len())];
        r.reverse();
        if l == r {
            return Some(i);
        }
    }
    None
}

fn find_reflection(p: &Vec<Vec<char>>) -> Option<usize> {
    let rows = p
        .iter()
        .map(|r| {
            r.iter()
                .fold(0, |acc, &c| if c == '#' { (acc * 2) + 1 } else { acc * 2 })
        })
        .collect::<Vec<_>>();

    if let Some(r) = check_reflection(&rows) {
        return Some(100 * r);
    }
    let cols = (0..p[0].len())
        .map(|x| {
            (0..p.len())
                .map(|y| p[y][x])
                .fold(0, |acc, c| if c == '#' { (acc * 2) + 1 } else { acc * 2 })
        })
        .collect::<Vec<_>>();

    if let Some(c) = check_reflection(&cols) {
        return Some(c);
    }
    None
}

fn part1(input: &In) -> Out {
    input.iter().map(|p| find_reflection(p).unwrap()).sum()
}

fn part2(input: &In) -> Out {
    PART2_RESULT
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
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
