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

type In = Vec<Vec<Vec<char>>>;
type Out = usize;
const PART1_RESULT: Out = 405;
const PART2_RESULT: Out = 400;

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

// Check 1D vector for reflections & return reflection point
fn check_reflection(v: &Vec<usize>, ignore: Option<usize>) -> Option<usize> {
    let len = v.len();
    for i in (1..len) {
        let l = &v[(if i > len / 2 { i - (len - i) } else { 0 })..i];
        let r = &mut v.clone()[i..(i + i).min(v.len())];
        r.reverse();
        if l == r {
            if let Some(ignore) = ignore {
                if i == ignore {
                    continue;
                }
            }
            return Some(i);
        }
    }
    None
}

// Flatten 2D binary vector into 2 x 1D usize vectors (rows/columns)
fn flatten_rows_cols(v: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let rows = v
        .iter()
        .map(|r| {
            r.iter()
                .fold(0, |acc, &c| if c == '#' { (acc * 2) + 1 } else { acc * 2 })
        })
        .collect::<Vec<_>>();
    let cols = (0..v[0].len())
        .map(|x| {
            (0..v.len())
                .map(|y| v[y][x])
                .fold(0, |acc, c| if c == '#' { (acc * 2) + 1 } else { acc * 2 })
        })
        .collect::<Vec<_>>();
    (rows, cols)
}

// Simple reflection
fn find_reflection(p: &Vec<Vec<char>>) -> Option<usize> {
    // Flatten grid into 2 x 1D arrays (binary vector -> usize)
    let (rows, cols) = flatten_rows_cols(p);
    // Check rows
    if let Some(r) = check_reflection(&rows, None) {
        return Some(100 * r);
    }
    // Check cols
    if let Some(c) = check_reflection(&cols, None) {
        return Some(c);
    }
    None
}

// Reflection with smudge
fn find_reflection2(p: &Vec<Vec<char>>) -> Option<usize> {
    // Flatten grid into 2 x 1D arrays (binary vector -> usize)
    let (mut rows, mut cols) = flatten_rows_cols(p);
    // For each element in the 1D flattened vector we
    // XOR each bit and test for reflection (ignoring
    // any simple reflection)
    // (should avoid any reallocations)
    let ignore = check_reflection(&rows, None);
    let bits = p[0].len();
    for i in (0..rows.len()) {
        for b in (0..=bits) {
            rows[i] ^= 1 << b;
            if let Some(r) = check_reflection(&rows, ignore) {
                return Some(100 * r);
            }
            // Make sure we reset bit
            rows[i] ^= 1 << b;
        }
    }
    // Do the same for cols
    let ignore = check_reflection(&cols, None);
    let bits = p.len();
    for i in (0..cols.len()) {
        for b in (0..=bits) {
            cols[i] ^= 1 << b;
            if let Some(c) = check_reflection(&cols, ignore) {
                return Some(c);
            }
            cols[i] ^= 1 << b;
        }
    }
    None
}

fn print_grid(g: &Vec<Vec<char>>) -> String {
    g.iter()
        .map(|r| r.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn part1(input: &In) -> Out {
    input.iter().map(|p| find_reflection(p).unwrap()).sum()
}

fn part2(input: &In) -> Out {
    input.iter().map(|p| find_reflection2(p).unwrap()).sum()
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
