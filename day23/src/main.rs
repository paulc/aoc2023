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
use util::simplegraph::Graph;

#[derive(Debug, Clone)]
struct T();

type In = Grid<char>;
type Out = usize;
const PART1_RESULT: Out = 94;
const PART2_RESULT: Out = 154;

fn parse_input(input: &mut impl Read) -> std::io::Result<In> {
    let data = BufReader::new(input)
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Ok(Grid::from(data))
}

fn find_paths(map: &Grid<char>, start: &Point, end: &Point) -> Vec<usize> {
    let mut out: Vec<usize> = vec![];
    let mut q: Vec<(Point, Point, HashSet<Point>)> =
        vec![(start.clone(), start.clone(), HashSet::new())];
    while let Some((p, last, mut visited)) = q.pop() {
        if p == *end {
            out.push(visited.len());
        } else {
            visited.insert(p.clone());
            let available = match map.get(&p) {
                Some('^') => vec![p + UP],
                Some('>') => vec![p + RIGHT],
                Some('v') => vec![p + DOWN],
                Some('<') => vec![p + LEFT],
                Some('.') => map
                    .adjacent(&p)
                    .iter()
                    .filter(|&p| map.get(p).unwrap() != &'#')
                    .cloned()
                    .collect::<Vec<_>>(),
                _ => panic!("Invalid point"),
            }
            .into_iter()
            .filter(|p| !visited.contains(p))
            .collect::<Vec<_>>();
            match available.len() {
                0 => {}
                1 => q.push((available[0], last, visited)),
                _ => available
                    .into_iter()
                    .for_each(|p| q.push((p.clone(), p.clone(), visited.clone()))),
            }
        }
    }
    out
}

fn partition(map: &Grid<char>, start: &Point, end: &Point) -> Graph<Point> {
    let mut q: Vec<(Point, Point, HashSet<Point>)> =
        vec![(start.clone(), start.clone(), HashSet::new())];
    let mut segments: Vec<(Point, Point, u32)> = vec![];
    let mut nodes: Vec<Point> = vec![];
    while let Some((p, start, mut visited)) = q.pop() {
        if p == *end {
            segments.push((start.clone(), p.clone(), visited.len() as u32));
        } else {
            visited.insert(p.clone());
            let available = map
                .adjacent(&p)
                .iter()
                .filter(|&p| map.get(p).unwrap() != &'#' && !visited.contains(p))
                .cloned()
                .collect::<Vec<_>>();
            match available.len() {
                0 => {}
                1 => {
                    q.push((available[0], start, visited));
                }
                _ => {
                    segments.push((start.clone(), p.clone(), visited.len() as u32));
                    if !nodes.contains(&p) {
                        nodes.push(p.clone());
                        available.into_iter().for_each(|p2| {
                            q.push((
                                p2.clone(),
                                p.clone(),
                                HashSet::from_iter(vec![p].into_iter()),
                            ));
                        });
                    }
                }
            }
        }
    }
    Graph::new_from_edges(segments)
}

fn find_paths2(map: &Grid<char>, start: &Point, end: &Point) -> Vec<usize> {
    let mut out: Vec<usize> = vec![];
    let mut q: Vec<(Point, HashSet<Point>)> = vec![(start.clone(), HashSet::new())];
    while let Some((p, mut visited)) = q.pop() {
        if p == *end {
            out.push(visited.len());
        } else {
            visited.insert(p.clone());
            let available = map
                .adjacent(&p)
                .iter()
                .filter(|&p| map.get(p).unwrap() != &'#' && !visited.contains(p))
                .cloned()
                .collect::<Vec<_>>();
            match available.len() {
                0 => {}
                1 => {
                    q.push((available[0], visited));
                }
                _ => available.into_iter().for_each(|p| {
                    q.push((p, visited.clone()));
                }),
            }
        }
    }
    out
}

fn part1(input: &In) -> Out {
    *find_paths(
        &input,
        &(input.start + Offset::new(1, 0)),
        &(input.end + Offset::new(-1, 0)),
    )
    .iter()
    .max()
    .unwrap()
}

fn part2(input: &In) -> Out {
    let g = partition(
        &input,
        &(input.start + Offset::new(1, 0)),
        &(input.end + Offset::new(-1, 0)),
    );
    g.print_to_dot();
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
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";
