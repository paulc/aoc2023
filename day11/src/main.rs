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
use util::combinations::combinations;

type In = Stars;
type Out = usize;
const PART1_RESULT: Out = 374;
const PART2_RESULT1: Out = 1030;
const PART2_RESULT2: Out = 8410;

#[derive(Debug)]
struct Stars(Vec<(usize, usize)>);

fn n_empty(v: usize, s: &Vec<usize>) -> usize {
    s.iter().take_while(|&&i| i < v).count()
}

fn manhattan((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    usize::max(x1, x2) - usize::min(x1, x2) + usize::max(y1, y2) - usize::min(y1, y2)
}

impl Stars {
    fn find_empty(&self) -> (Vec<usize>, Vec<usize>) {
        let mut occupied_x: Vec<usize> = vec![];
        let mut occupied_y: Vec<usize> = vec![];
        self.0.iter().for_each(|(x, y)| {
            occupied_x.push(*x);
            occupied_y.push(*y);
        });
        let empty_x = (0..occupied_x.iter().max().unwrap().clone())
            .filter(|c| !occupied_x.contains(c))
            .collect::<Vec<_>>();
        let empty_y = (0..occupied_y.iter().max().unwrap().clone())
            .filter(|r| !occupied_y.contains(r))
            .collect::<Vec<_>>();
        (empty_x, empty_y)
    }
    fn expand(&self, n: usize) -> Stars {
        let (empty_x, empty_y) = self.find_empty();
        Stars(
            self.0
                .iter()
                .map(|&(x, y)| {
                    (
                        x + n_empty(x, &empty_x) * (n - 1),
                        y + n_empty(y, &empty_y) * (n - 1),
                    )
                })
                .collect::<Vec<_>>(),
        )
    }
}

fn parse_input(input: &mut impl Read) -> In {
    let data = BufReader::new(input)
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.unwrap()
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((x.clone(), y.clone()))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Stars(data)
}

fn part1(input: &In) -> Out {
    let expanded = input.expand(2);
    combinations(&expanded.0, 2)
        .iter()
        .map(|v| manhattan(v[0], v[1]))
        .sum()
}

fn part2(input: &In, n: usize) -> Out {
    let expanded = input.expand(n);
    combinations(&expanded.0, 2)
        .iter()
        .map(|v| manhattan(v[0], v[1]))
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
        part2(&input, 1000000),
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
    assert_eq!(part2(&input, 10), PART2_RESULT1);
    assert_eq!(part2(&input, 100), PART2_RESULT2);
}

#[cfg(test)]
const TESTDATA: &str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
