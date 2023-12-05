#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::ops::Range;

type In = (Vec<i64>, Vec<Vec<(i64, i64, Range<i64>)>>);
type Out = i64;
const PART1_RESULT: Out = 35;
const PART2_RESULT: Out = 46;

fn parse_input(input: &mut impl Read) -> In {
    let mut seeds: Vec<i64> = Vec::new();
    let mut chain: Vec<Vec<(i64, i64, Range<i64>)>> = Vec::new();
    BufReader::new(input).lines().for_each(|l| {
        let l = l.unwrap();
        if l.contains("seeds:") {
            seeds = l
                .split_whitespace()
                .filter_map(|w| w.parse::<i64>().ok())
                .collect::<Vec<_>>();
        } else if l.contains("map:") {
            // Sort last list
            if let Some(mut last) = chain.last_mut() {
                last.sort_by_key(|s| s.0);
            }
            // Create new map
            chain.push(Vec::new());
        } else if !l.is_empty() {
            if let [dest, source, length] = l
                .split_whitespace()
                .filter_map(|w| w.parse::<i64>().ok())
                .collect::<Vec<_>>()
                .as_slice()
            {
                chain.last_mut().unwrap().push((
                    *source,
                    *dest - *source,
                    *source..*source + *length,
                ));
            }
        }
    });
    (seeds, chain)
}

fn map_single(input: i64, chain: &Vec<Vec<(i64, i64, Range<i64>)>>) -> i64 {
    let mut v = input;
    for map in chain.iter() {
        for (start, offset, range) in map.iter() {
            if range.contains(&v) {
                v = v + offset;
                break;
            }
        }
    }
    v
}

fn map_range(
    (seed_start, seed_end): (i64, i64),
    map: &Vec<(i64, i64, Range<i64>)>,
) -> Vec<(i64, i64)> {
    let mut out: Vec<(i64, i64)> = Vec::new();
    let mut seed_start = seed_start;
    let mut end = false;
    for (start, offset, range) in map.iter() {
        println!(
            "MAP: {:?} SEEDS: {}-{} OFFSET: {} :: {} {}",
            range,
            seed_start,
            seed_end,
            offset,
            range.contains(&seed_start),
            range.contains(&seed_end)
        );
        match (range.contains(&seed_start), range.contains(&seed_end)) {
            (true, true) => {
                out.push((seed_start + offset, seed_end + offset));
                end = true;
                break;
            }
            (true, false) => {
                out.push((seed_start + offset, range.end - 1 + offset));
                seed_start = range.end;
            }
            (false, true) => {
                out.push((seed_start, range.start - 1));
                out.push((range.start + offset, seed_end + offset));
                end = true;
                break;
            }
            (false, false) => {
                if seed_end < range.start {
                    out.push((seed_start, seed_end));
                    end = true;
                    break;
                } else {
                    // Check against next map block
                }
            }
        }
    }
    if !end {
        out.push((seed_start, seed_end));
    }
    out.sort_by_key(|p| p.0);
    println!("OUT >>> {:?}\n", out);
    out
}

fn count(v: &Vec<(i64, i64)>) -> i64 {
    let mut count: i64 = 0;
    v.iter().for_each(|(a, b)| count += (b - a + 1));
    count
}

fn part1((seeds, chain): &In) -> Out {
    seeds.iter().map(|s| map_single(*s, chain)).min().unwrap()
}

fn part2((seeds, chain): &In) -> Out {
    let mut seeds: Vec<(i64, i64)> = seeds
        .as_slice()
        .chunks_exact(2)
        .map(|c| (c[0], c[0] + c[1] - 1))
        .collect();
    seeds.sort_by_key(|p| p.0);
    println!("{:?} {}", seeds, count(&seeds));
    for map in chain.iter() {
        seeds = seeds
            .iter()
            .flat_map(|s| map_range(s.clone(), map))
            .collect::<Vec<_>>();
        seeds.sort_by_key(|p| p.0);
    }
    println!("{:?} {}", seeds, count(&seeds));
    seeds.first().unwrap().0
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
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
