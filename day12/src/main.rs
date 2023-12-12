#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use util::combinations::combinations;

type In = Vec<(Vec<char>, Vec<usize>)>;
type Out = usize;
const PART1_RESULT: Out = 21;
const PART2_RESULT: Out = 0;

/*

#[derive(Debug)]
struct Spring {
    springs: Vec<char>,
    groups: Vec<usize>,
    unknown: Vec<usize>,
    total: usize,
    known: usize,
}

impl Spring {
    fn find_groups(springs: &Vec<char>) -> Vec<usize> {
        let mut out = vec![];
        let mut count = 0;
        springs.iter().for_each(|&s| {
            if s == '#' {
                count += 1;
            } else {
                if count > 0 {
                    out.push(count);
                }
                count = 0;
            }
        });
        // Last element
        if count > 0 {
            out.push(count);
        }
        out
    }
    fn check(&self) -> usize {
        combinations(&self.unknown, self.total - self.known)
            .iter()
            .filter(|c| {
                let mut springs = self.springs.clone();
                c.iter().for_each(|&i| springs[i] = '#');
                Spring::find_groups(&springs) == self.groups
            })
            .count()
    }
}

fn parse_input1(input: &mut impl Read) -> In {
    let data = BufReader::new(input)
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (l, r) = l.split_once(" ").unwrap();
            let mut unknown: Vec<usize> = vec![];
            let mut known: usize = 0;
            let springs = l
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if c == '?' {
                        unknown.push(i);
                        '.'
                    } else {
                        if c == '#' {
                            known += 1
                        }
                        c
                    }
                })
                .collect::<Vec<_>>();
            let groups = r
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let total: usize = groups.iter().sum();
            Spring {
                springs,
                groups,
                unknown,
                total,
                known,
            }
        })
        .collect::<Vec<_>>();
    data
}

*/

fn parse_input(input: &mut impl Read) -> In {
    let data = BufReader::new(input)
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (l, r) = l.split_once(" ").unwrap();
            let springs = l.chars().collect::<Vec<_>>();
            let groups = r
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (springs, groups)
        })
        .collect::<Vec<_>>();
    data
}

fn check(springs: &[char], groups: &[usize]) -> usize {
    // println!("{:?} {:?}", springs, groups);
    // No groups left - remaining springs must be empty
    if groups.is_empty() {
        if springs.contains(&'#') {
            // println!("{:?}", "NOT FOUND");
            return 0;
        } else {
            // println!("{:?}", "FOUND");
            return 1;
        }
    }
    // Check if we have enough spring positions left for groups
    if springs.len() < groups.iter().sum() {
        // println!("{:?}", "NOT FOUND");
        return 0;
    }
    let next_len = groups[0];
    let mut result: usize = 0;
    if springs[0] == '.' {
        // Step forward one position
        result += check(&springs[1..], &groups);
    } else if springs[0] == '#' || springs[0] == '?' {
        // Check if we can match full group
        /*
            println!(
                "Check Group:: {:?} {} {}",
                springs[1..next_len].iter().all(|&c| c != '.'),
                springs.len(),
                next_len
            );
        */
        if springs[1..next_len].iter().all(|&c| c != '.')
            && (springs.len() == next_len || springs[next_len] != '#')
        {
            // If we can step forward full group
            result += check(&springs[(next_len + 1).min(springs.len())..], &groups[1..]);
        }
        if springs[0] == '?' {
            result += check(&springs[1..], &groups);
        }
    }
    result
}

fn part1(input: &In) -> Out {
    input
        .iter()
        .map(|s| check(s.0.as_slice(), s.1.as_slice()))
        .sum()
}

fn part2(input: &In) -> Out {
    input
        .iter()
        .map(|s| check(s.0.as_slice(), s.1.as_slice()))
        .sum()
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
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
