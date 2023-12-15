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

type In = Vec<Vec<u8>>;
type Out = usize;
const PART1_RESULT: Out = 1320;
const PART2_RESULT: Out = 145;

fn parse_input(input: &mut impl Read) -> In {
    let mut i = BufReader::new(input).lines();
    let mut data: Vec<Vec<u8>> = vec![vec![]];
    i.next().unwrap().unwrap().bytes().for_each(|i| {
        if i == b',' {
            data.push(vec![])
        } else {
            data.last_mut().unwrap().push(i)
        }
    });
    data
}

fn hash(k: &Vec<u8>) -> u8 {
    k.iter().fold(0, |acc, &v| ((acc + v as u32) * 17) % 256) as u8
}

#[derive(Debug)]
enum Operation {
    Add(String, u8, u8), // Key, Hash Value, Lens Value
    Remove(String, u8),  // Key, Hash Value
}

fn operation(v: &Vec<u8>) -> Operation {
    let k = v
        .iter()
        .take_while(|&&c| c != b'-' && c != b'=')
        .cloned()
        .collect::<Vec<_>>();
    let h = hash(&k);
    if v.last().unwrap() == &b'-' {
        Operation::Remove(String::from_utf8(k).unwrap(), h)
    } else {
        Operation::Add(String::from_utf8(k).unwrap(), h, v.last().unwrap() - b'0')
    }
}

fn part1(input: &In) -> Out {
    input
        .iter()
        .map(|s| s.iter().fold(0, |acc, &v| ((acc + v as usize) * 17) % 256))
        .sum()
}

fn part2(input: &In) -> Out {
    let mut boxes: [Vec<(String, u8)>; 256] = (0..256)
        .map(|_| Vec::new())
        .collect::<Vec<Vec<(String, u8)>>>()
        .try_into()
        .unwrap();
    input.iter().for_each(|op| match operation(op) {
        Operation::Add(k, h, v) => {
            let mut b = boxes.get_mut(h as usize).unwrap();
            let mut found = false;
            b.iter_mut().for_each(|i| {
                if i.0 == k {
                    i.1 = v;
                    found = true
                }
            });
            if !found {
                b.push((k, v));
            }
        }
        Operation::Remove(k, h) => {
            let mut b = boxes.get_mut(h as usize).unwrap();
            if let Some(i) = b.iter().position(|i| i.0 == k) {
                b.remove(i);
            }
        }
    });
    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(move |(s, (_, f))| (i + 1) * (s + 1) * f.clone() as usize)
        })
        .sum()
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
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";
