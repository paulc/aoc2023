#![allow(unused)]

use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
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
type Out = usize;
const PART1_RESULT: Out = 46;
const PART2_RESULT: Out = 51;

fn parse_input(input: &mut impl Read) -> std::io::Result<In> {
    let data = BufReader::new(input)
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Ok(Grid::from(data))
}

fn push(
    next: (Point, Offset),
    visited: &HashSet<(Point, Offset)>,
    q: &mut VecDeque<(Point, Offset)>,
) {
    if !visited.contains(&next) {
        q.push_back(next)
    }
}

fn reflect(d: Offset, mirror: char) -> Offset {
    match (mirror, d) {
        ('/', UP) => RIGHT,
        ('/', DOWN) => LEFT,
        ('/', RIGHT) => UP,
        ('/', LEFT) => DOWN,
        ('\\', UP) => LEFT,
        ('\\', DOWN) => RIGHT,
        ('\\', RIGHT) => DOWN,
        ('\\', LEFT) => UP,
        _ => panic!("Invalid"),
    }
}

fn trace(input: &In, start: (Point, Offset)) -> usize {
    let mut q: VecDeque<(Point, Offset)> = VecDeque::new();
    let mut visited: HashSet<(Point, Offset)> = HashSet::new();
    q.push_back(start.clone());
    visited.insert(start.clone());
    while let Some((p, d)) = q.pop_front() {
        match input.get(&p) {
            Some(c) => {
                visited.insert((p, d).clone());
                match c {
                    '.' => push((p + d, d), &visited, &mut q),
                    '/' => {
                        let d = reflect(d, '/');
                        push((p + d, d), &visited, &mut q)
                    }
                    '\\' => {
                        let d = reflect(d, '\\');
                        push((p + d, d), &visited, &mut q)
                    }
                    '-' => {
                        if d == LEFT || d == RIGHT {
                            push((p + d, d), &visited, &mut q);
                        } else {
                            for d in [LEFT, RIGHT] {
                                push((p + d, d), &visited, &mut q);
                            }
                        }
                    }
                    '|' => {
                        if d == UP || d == DOWN {
                            push((p + d, d), &visited, &mut q);
                        } else {
                            for d in [UP, DOWN] {
                                push((p + d, d), &visited, &mut q);
                            }
                        }
                    }
                    _ => panic!("Invalid tile"),
                }
            }
            None => {} // Outside bounds
        }
    }
    visited.iter().map(|(p, d)| p).collect::<HashSet<_>>().len()
}

fn part1(input: &In) -> Out {
    trace(input, (Point::new(0, 0), RIGHT))
}

fn part2(input: &In) -> Out {
    (0..(input.end.x + 1))
        .into_par_iter()
        .flat_map(|x| {
            [
                trace(input, (Point::new(x, 0), DOWN)),
                trace(input, (Point::new(x, input.end.y), UP)),
            ]
        })
        .chain((0..(input.end.y + 1)).into_par_iter().flat_map(|y| {
            [
                trace(input, (Point::new(0, y), RIGHT)),
                trace(input, (Point::new(input.end.x, y), UP)),
            ]
        }))
        .max()
        .unwrap()
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
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";
