#![allow(unused)]

use num::integer::lcm;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;

type In = (Vec<char>, HashMap<String, (String, String)>);
type Out = usize;
const PART1_RESULT: Out = 2;
const PART2_RESULT: Out = 6;

fn parse_input(input: &mut impl Read) -> In {
    let mut lines = BufReader::new(input).lines().map(|l| l.unwrap());
    let turns = lines.next().unwrap().chars().collect::<Vec<_>>();
    let nodes = lines
        .skip(1)
        .map(|l| {
            l.split([' ', ',', ')', '(', '='])
                .filter_map(|s| {
                    if !s.is_empty() {
                        Some(s.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0].clone(), (v[1].clone(), v[2].clone())))
        .collect::<HashMap<_, _>>();
    (turns, nodes)
}

fn part1((turns, nodes): &In) -> Out {
    let mut current = "AAA".to_string();
    let mut result: usize = 0;
    for (i, &d) in turns.iter().cycle().enumerate() {
        current = if d == 'L' {
            nodes[&current].0.clone()
        } else {
            nodes[&current].1.clone()
        };
        if current == "ZZZ" {
            result = i + 1;
            break;
        }
    }
    result
}

fn part2((turns, nodes): &In) -> Out {
    let mut result: usize = 0;
    let mut current = nodes
        .keys()
        .filter(|&k| k.chars().last().unwrap() == 'A')
        .cloned()
        .collect::<Vec<_>>();
    let mut cycle: Vec<usize> = vec![0; current.len()];
    for (i, &d) in turns.iter().cycle().enumerate() {
        current = current
            .iter()
            .map(|c| {
                if d == 'L' {
                    nodes[c].0.clone()
                } else {
                    nodes[c].1.clone()
                }
            })
            .collect::<Vec<_>>();
        current.iter().enumerate().for_each(|(j, c)| {
            if c.chars().last().unwrap() == 'Z' {
                cycle[j] = i + 1;
            }
        });
        if cycle.iter().all(|&i| i != 0) {
            result = cycle.iter().fold(1_usize, |acc, &i| lcm(acc, i));
            break;
        }
    }
    result
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
    let input = parse_input(&mut TESTDATA1.trim_matches('\n').as_bytes());
    assert_eq!(part1(&input), PART1_RESULT);
}

#[test]
fn test_part2() {
    let input = parse_input(&mut TESTDATA2.trim_matches('\n').as_bytes());
    assert_eq!(part2(&input), PART2_RESULT);
}

#[cfg(test)]
const TESTDATA1: &str = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

#[cfg(test)]
const TESTDATA2: &str = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
