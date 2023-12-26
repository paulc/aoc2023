#![allow(unused)]

use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, Clone)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

type In = Vec<Game>;
type Out = usize;
const PART1_RESULT: Out = 8;
const PART2_RESULT: Out = 2286;

fn parse_input(input: &mut impl Read) -> In {
    let data = BufReader::new(input)
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (id, revealed) = l.split_once(": ").unwrap();
            let id = id.split_once(" ").unwrap().1.parse::<usize>().unwrap();
            let mut draws: Vec<Draw> = Vec::new();
            for r in revealed.split("; ") {
                let mut d = Draw {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                for c in r.split(", ") {
                    match c.split_once(" ") {
                        Some((n, "red")) => d.red += n.parse::<usize>().unwrap(),
                        Some((n, "blue")) => d.blue += n.parse::<usize>().unwrap(),
                        Some((n, "green")) => d.green += n.parse::<usize>().unwrap(),
                        _ => panic!("Invalid draw: {}", r),
                    }
                }
                draws.push(d);
            }
            Game { id, draws }
        })
        .collect::<Vec<_>>();
    data
}

fn part1(input: &In) -> Out {
    let mut result: usize = 0;
    for Game { id, draws } in input {
        let mut possible = true;
        for Draw { red, green, blue } in draws {
            if *red > 12 || *green > 13 || *blue > 14 {
                possible = false;
                break;
            }
        }
        if possible {
            result += id;
        }
    }
    result
}

fn part2(input: &In) -> Out {
    let mut result: usize = 0;
    for Game { id, draws } in input {
        let mut needed = Draw {
            red: 0,
            green: 0,
            blue: 0,
        };
        for Draw { red, green, blue } in draws {
            needed.red = max(needed.red, *red);
            needed.blue = max(needed.blue, *blue);
            needed.green = max(needed.green, *green);
        }
        result += needed.red * needed.blue * needed.green;
    }
    result
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
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
