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
use util::point::*;

type In = Grid<char>;
type Out = i32;
const PART1_RESULT: Out = 136;
const PART2_RESULT: Out = 64;

fn parse_input(input: &mut impl Read) -> In {
    let data = BufReader::new(input)
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Grid::from(data)
}

fn tilt(input: &Grid<char>, d: Offset) -> Grid<char> {
    let mut tilted = input.clone();
    let mut rocks = tilted.find(&'O');
    match d {
        UP => {}
        DOWN => rocks.reverse(),
        LEFT => rocks.sort_by_key(|&r| r.x),
        RIGHT => {
            rocks.sort_by_key(|&r| r.x);
            rocks.reverse()
        }
        _ => panic!("Invalid Direction"),
    }
    rocks.iter().for_each(|&p| {
        let mut p1 = p;
        let mut p2 = p + d;
        while tilted.check_bounds(p2) && tilted.get(p2).unwrap() == &'.' {
            tilted.set(p1, '.');
            tilted.set(p2, 'O');
            p1 = p2;
            p2 = p2 + d;
        }
    });
    tilted
}

fn cycle(input: &Grid<char>) -> Grid<char> {
    let c1 = tilt(input, UP);
    let c2 = tilt(&c1, LEFT);
    let c3 = tilt(&c2, DOWN);
    tilt(&c3, RIGHT)
}

fn part1(input: &In) -> Out {
    let mut tilted = tilt(input, UP);
    tilted
        .find(&'O')
        .iter()
        .map(|&p| tilted.size.dy - p.y)
        .sum::<i32>()
}

fn part2(input: &In) -> Out {
    let mut seen: HashMap<Vec<char>, usize> = HashMap::new();
    let mut count: usize = 0;
    let mut tilted = input.clone();
    seen.insert(input.data.clone(), 0);
    loop {
        tilted = cycle(&tilted);
        count += 1;
        let k = tilted.data.clone();
        if let Some(start) = seen.get(&k) {
            // println!("Found cycle: start={} count={}", start, count);
            let cycle_len = count - start;
            let cycles_left = 1000000000 - count;
            let remainder = cycles_left % cycle_len;
            for _ in 0..remainder {
                tilted = cycle(&tilted);
            }
            break;
        }
        seen.insert(k, count);
    }
    tilted
        .find(&'O')
        .iter()
        .map(|&p| tilted.size.dy - p.y)
        .sum::<i32>()
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
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
