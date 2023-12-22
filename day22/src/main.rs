#![allow(unused)]

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::time::Instant;

type In = Vec<Brick>;
type Out = i32;
const PART1_RESULT: Out = 5;
const PART2_RESULT: Out = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct P3 {
    x: i32,
    y: i32,
    z: i32,
}

impl P3 {
    fn drop(&self) -> P3 {
        P3 {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }
}

fn p3(x: i32, y: i32, z: i32) -> P3 {
    P3 { x, y, z }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    blocks: Vec<P3>,
    bottom: Vec<P3>,
    minz: i32,
}

impl Brick {
    fn new(p1: P3, p2: P3) -> Brick {
        let mut blocks: Vec<P3> = vec![];
        let mut bottom: Vec<P3> = vec![];
        let minz = p1.z.min(p2.z);
        for x in p1.x.min(p2.x)..=p1.x.max(p2.x) {
            for y in p1.y.min(p2.y)..=p1.y.max(p2.y) {
                for z in p1.z.min(p2.z)..=p1.z.max(p2.z) {
                    blocks.push(P3 { x, y, z });
                    if z == minz {
                        bottom.push(P3 { x, y, z });
                    }
                }
            }
        }
        Brick {
            blocks,
            bottom,
            minz,
        }
    }
    fn drop(&self) -> Brick {
        Brick {
            blocks: self.blocks.iter().map(|b| b.drop()).collect::<Vec<_>>(),
            bottom: self.bottom.iter().map(|b| b.drop()).collect::<Vec<_>>(),
            minz: self.minz - 1,
        }
    }
}

// Order bricks by lowest z value
impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.minz.partial_cmp(&self.minz)
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.minz.cmp(&self.minz)
    }
}

fn parse_input(input: &mut impl Read) -> std::io::Result<In> {
    let data = BufReader::new(input)
        .lines()
        .map(|l| {
            l.unwrap()
                .split([',', '~'])
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|p| Brick::new(p3(p[0], p[1], p[2]), p3(p[3], p[4], p[5])))
        .collect::<Vec<_>>();
    Ok(data)
}

fn drop(brick: &Brick, occupied: &HashSet<P3>) -> Option<(Brick, HashSet<P3>)> {
    let mut new = brick.clone();
    let mut dropped = false;
    while new.minz > 1 && !new.bottom.iter().any(|p| occupied.contains(&p.drop())) {
        dropped = true;
        new = new.drop();
    }
    if dropped {
        let mut occupied = occupied.clone();
        brick.blocks.iter().for_each(|b| {
            occupied.remove(b);
        });
        new.blocks.iter().for_each(|b| {
            occupied.insert(*b);
        });
        Some((new, occupied))
    } else {
        None
    }
}

fn part1(input: &In) -> Out {
    let mut occupied: HashSet<P3> =
        HashSet::from_iter(input.iter().flat_map(|b| b.blocks.iter().cloned()));
    let mut moved: Vec<Brick> = vec![];
    // Drop blocks in height order
    let mut blocks: BinaryHeap<Brick> = BinaryHeap::from_iter(input.iter().cloned());
    while let Some(b) = blocks.pop() {
        if let Some((b2, o2)) = drop(&b, &occupied) {
            moved.push(b2);
            occupied = o2;
        } else {
            moved.push(b);
        }
    }
    // Disentegrate blocks
    let mut count = 0;
    for b in &moved {
        let mut occupied = occupied.clone();
        b.blocks.iter().for_each(|b| {
            occupied.remove(b);
        });
        if moved
            .iter()
            .filter(|&b2| b2 != b)
            .all(|b2| drop(b2, &occupied).is_none())
        {
            count += 1;
        }
    }
    count
}

fn part2(input: &In) -> Out {
    PART2_RESULT
}

fn main() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;
    let input = parse_input(&mut f)?;
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
    let input = parse_input(&mut TESTDATA.trim_matches('\n').as_bytes()).unwrap();
    assert_eq!(part1(&input), PART1_RESULT);
}

#[test]
fn test_part2() {
    let input = parse_input(&mut TESTDATA.trim_matches('\n').as_bytes()).unwrap();
    assert_eq!(part2(&input), PART2_RESULT);
}

#[cfg(test)]
const TESTDATA: &str = r"
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";
