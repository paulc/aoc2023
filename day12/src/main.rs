#![allow(unused)]

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use util::combinations::combinations;

type In = Vec<(Vec<char>, Vec<usize>)>;
type Out = usize;
const PART1_RESULT: Out = 21;
const PART2_RESULT: Out = 525152;

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

fn hash_state(a: &[char], b: &[usize]) -> (u64, u64) {
    let mut h1 = DefaultHasher::new();
    let mut h2 = DefaultHasher::new();
    for &c in a {
        c.hash(&mut h1);
    }
    for &i in a {
        i.hash(&mut h2);
    }
    (h1.finish(), h2.finish())
}

fn check(springs: &[char], groups: &[usize], cache: &mut HashMap<(u64, u64), usize>) -> usize {
    // println!("CHECK: {:?} {:?}", springs, groups);

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
        // println!("{:?}", "NOT FOUND (TOO SHORT)");
        return 0;
    }

    // Check cache
    /*
    if let Some(&result) = cache.get(&hash_state(springs, groups)) {
        println!("Found Cache: {:?} {:?} = {}", springs, groups, result);
        return result;
    }
    */

    let next_len = groups[0];
    let mut result: usize = 0;
    if springs[0] == '.' {
        // Step forward one position
        result += check(&springs[1..], &groups, cache);
    } else if springs[0] == '#' || springs[0] == '?' {
        // Check if we can match full group
        /*
        println!(
            "Check Group:: {:?} {} {}",
            springs[1..next_len].iter().all(|&c| c != '.')
                && (springs.len() == next_len || springs[next_len] != '#'),
            springs.len(),
            next_len
        );
        */
        if springs[1..next_len].iter().all(|&c| c != '.')
            && (springs.len() == next_len || springs[next_len] != '#')
        {
            // If we can step forward full group
            result += check(
                &springs[(next_len + 1).min(springs.len())..],
                &groups[1..],
                cache,
            );
        }
        if springs[0] == '?' {
            result += check(&springs[1..], &groups, cache);
        }
    }
    /*
    if result > 0 {
        cache.insert(hash_state(springs, groups), result);
    }
    */
    result
}

fn part1(input: &In) -> Out {
    let mut cache: HashMap<(u64, u64), usize> = HashMap::new();
    input
        .iter()
        .map(|s| check(s.0.as_slice(), s.1.as_slice(), &mut cache))
        .sum()
}

fn part2(input: &In) -> Out {
    let mut cache: HashMap<(u64, u64), usize> = HashMap::new();
    input
        .iter()
        .map(|(s, g)| {
            let s = s
                .iter()
                .cloned()
                .chain(vec!['?'].into_iter())
                .cycle()
                .take((s.len() + 1) * 5 - 1)
                .collect::<Vec<_>>();
            let g = g
                .iter()
                .cycle()
                .take(g.len() * 5)
                .cloned()
                .collect::<Vec<_>>();
            (s, g)
        })
        .map(|(s, g)| check(s.as_slice(), g.as_slice(), &mut cache))
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
