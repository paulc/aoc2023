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

#[derive(Debug, Clone)]
struct T();

type In = (Grid<char>, Point);
type Out = usize;
const PART1_RESULT: Out = 16;
const PART2_RESULT: Out = 0;

fn parse_input(input: &mut impl Read) -> std::io::Result<In> {
    let mut map = Grid::from(
        BufReader::new(input)
            .lines()
            .map(|l| l.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    let start = map.find(&'S')[0];
    map.set(&start, '.');
    Ok((map, start))
}

fn step(map: &Grid<char>, start: &Vec<Point>) -> Vec<Point> {
    let mut q = start.clone();
    let mut visited: HashSet<Point> = HashSet::new(); // HashSet::from_iter(q.iter().cloned());
    while let Some(p) = q.pop() {
        map.adjacent(&p)
            .iter()
            .filter(|p| map.get(p).unwrap() == &'.')
            .for_each(|p| {
                if !visited.contains(p) {
                    visited.insert(p.clone());
                }
            });
    }
    visited.into_iter().collect::<Vec<_>>()
}

fn print_steps(map: &Grid<char>, steps: &Vec<Point>) {
    let mut g = map.clone();
    steps.iter().for_each(|p| g.set(p, 'O').unwrap());
    println!("{}", g);
}

fn part1((map, start): &In, count: usize) -> Out {
    let mut start = vec![start.clone()];
    for i in 0..count {
        start = step(map, &start);
    }
    start.len()
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
        part1(&input, 64),
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
    assert_eq!(part1(&input, 6), PART1_RESULT);
}

#[test]
fn test_part2() {
    let input = parse_input(&mut TESTDATA.trim_matches('\n').as_bytes()).unwrap();
    assert_eq!(part2(&input), PART2_RESULT);
}

#[cfg(test)]
const TESTDATA: &str = r"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";
