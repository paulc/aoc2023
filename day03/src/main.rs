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
use util::grid::Grid;
use util::point::{Offset, Point};

type In = Grid<char>;
type Out = u32;
const PART1_RESULT: Out = 4361;
const PART2_RESULT: Out = 467835;

fn parse_input(input: &mut impl Read) -> In {
    let data = BufReader::new(input)
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Grid::from(data)
}

fn find_parts(input: &In) -> Vec<(u32, Vec<Point>)> {
    let mut out: Vec<(u32, Vec<Point>)> = Vec::new();
    for y in 0..input.size.dy {
        let mut number: u32 = 0;
        let mut points: Vec<Point> = Vec::new();
        let mut valid = false;
        let mut in_number = false;
        for x in 0..input.size.dx {
            let p = Point::new(x, y);
            let c = input.get(&p).unwrap();
            match c {
                '0'..='9' => {
                    if !in_number {
                        in_number = true;
                    }
                    number = number * 10 + c.to_digit(10).unwrap();
                    points.push(p);
                    'adjacent: for dx in [-1, 0, 1] {
                        for dy in [-1, 0, 1] {
                            if let Some(c) = input.get(&(p + Offset::new(dx, dy))) {
                                match c {
                                    '0'..='9' | '.' => {}
                                    _ => {
                                        valid = true;
                                        break 'adjacent;
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    if in_number && valid {
                        out.push((number, points.clone()));
                    }
                    number = 0;
                    valid = false;
                    in_number = false;
                    points.clear();
                }
            }
        }
        // Handle End of Line
        if in_number && valid {
            out.push((number, points.clone()));
        }
    }
    out
}

fn find_gears(input: &In) -> Vec<Point> {
    let mut gears: Vec<Point> = Vec::new();
    for y in 0..input.size.dy {
        for x in 0..input.size.dx {
            let p = Point::new(x, y);
            if input.get(&p).unwrap() == &'*' {
                gears.push(p);
            }
        }
    }
    gears
}

fn part1(input: &In) -> Out {
    find_parts(input).iter().map(|(n, _)| n).sum()
}

fn part2(input: &In) -> Out {
    let mut out: u32 = 0;
    let parts = find_parts(input);
    for g in find_gears(input) {
        'gears: {
            let mut touch: Vec<u32> = Vec::new();
            // For each part check if touches gear
            for (n, p_cover) in &parts {
                'parts: for p1 in p_cover {
                    for dx in [-1, 0, 1] {
                        for dy in [-1, 0, 1] {
                            let p2 = g + Offset::new(dx, dy);
                            if *p1 == p2 {
                                // Part touches gear
                                touch.push(*n);
                                if touch.len() == 2 {
                                    // Two parts touch
                                    out += touch[0] * touch[1];
                                    break 'gears;
                                } else {
                                    break 'parts;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    out
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
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
