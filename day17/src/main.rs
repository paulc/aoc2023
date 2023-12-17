#![allow(unused)]

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

type In = Grid<u8>;
type Out = u32;
const PART1_RESULT: Out = 102;
const PART2_RESULT: Out = 94;

fn parse_input(input: &mut impl Read) -> std::io::Result<In> {
    let data = BufReader::new(input)
        .lines()
        .map(|l| l.unwrap().bytes().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Ok(Grid::from(data))
}

fn dir(o: Offset) -> String {
    match o {
        LEFT => "LEFT",
        RIGHT => "RIGHT",
        UP => "UP",
        DOWN => "DOWN",
        _ => "?",
    }
    .to_string()
}

fn search(g: &Grid<u8>, start: Point, target: Point) -> u32 {
    let mut costs: Vec<usize> = Vec::new();
    let mut visited: HashMap<(Point, Offset, u32), u32> = HashMap::new();
    let mut q: VecDeque<(Point, Offset, u32, u32)> = VecDeque::new();
    q.push_back((start.clone(), RIGHT, 0, 0));
    visited.insert((start.clone(), RIGHT, 0), 0);
    while let Some((p, prev, count, loss)) = q.pop_front() {
        let available = if count == 3 {
            match prev {
                UP | DOWN => vec![LEFT, RIGHT],
                LEFT | RIGHT => vec![UP, DOWN],
                _ => panic!("Invalid Direction"),
            }
        } else {
            match prev {
                UP => vec![UP, LEFT, RIGHT],
                DOWN => vec![DOWN, LEFT, RIGHT],
                RIGHT => vec![UP, DOWN, RIGHT],
                LEFT => vec![UP, DOWN, LEFT],
                _ => panic!("Invalid Direction"),
            }
        };
        for d in available {
            let p2 = p + d;
            if g.check_bounds(p2) {
                // println!("{} ({},{}) = {} ==> {}", p, dir(prev), count, loss, dir(d));
                let (prev, count) = if d != prev { (d, 1) } else { (d, count + 1) };
                let loss = loss + *g.get(p2).unwrap() as u32;
                if let Some(min_loss) = visited.get_mut(&(p2, prev, count)) {
                    if loss < *min_loss {
                        *min_loss = loss;
                        q.push_back((p2.clone(), prev, count, loss));
                    }
                } else {
                    visited.insert((p2.clone(), prev, count), loss);
                    q.push_back((p2.clone(), prev, count, loss));
                }
            }
        }
    }
    visited
        .iter()
        .filter_map(|((p, _, _), v)| if *p == g.end { Some(v) } else { None })
        .min()
        .unwrap()
        .clone()
}

fn search2(g: &Grid<u8>, start: Point, target: Point) -> u32 {
    let mut costs: Vec<usize> = Vec::new();
    let mut visited: HashMap<(Point, Offset, u32), u32> = HashMap::new();
    let mut q: VecDeque<(Point, Offset, u32, u32)> = VecDeque::new();
    q.push_back((start.clone(), RIGHT, 0, 0));
    visited.insert((start.clone(), RIGHT, 0), 0);
    while let Some((p, prev, count, loss)) = q.pop_front() {
        let available = if count < 4 {
            vec![prev]
        } else if count == 10 {
            match prev {
                UP | DOWN => vec![LEFT, RIGHT],
                LEFT | RIGHT => vec![UP, DOWN],
                _ => panic!("Invalid Direction"),
            }
        } else {
            match prev {
                UP => vec![UP, LEFT, RIGHT],
                DOWN => vec![DOWN, LEFT, RIGHT],
                RIGHT => vec![UP, DOWN, RIGHT],
                LEFT => vec![UP, DOWN, LEFT],
                _ => panic!("Invalid Direction"),
            }
        };
        /*
        println!(
            "{} ({},{}) = {} ==> {:?}",
            p,
            dir(prev),
            count,
            loss,
            available.iter().map(|&d| dir(d)).collect::<Vec<_>>()
        );
        */
        for d in available {
            let p2 = p + d;
            if g.check_bounds(p2) {
                let (prev, count) = if d != prev { (d, 1) } else { (d, count + 1) };
                let loss = loss + *g.get(p2).unwrap() as u32;
                if let Some(min_loss) = visited.get_mut(&(p2, prev, count)) {
                    if loss < *min_loss {
                        *min_loss = loss;
                        q.push_back((p2.clone(), prev, count, loss));
                    }
                } else {
                    visited.insert((p2.clone(), prev, count), loss);
                    q.push_back((p2.clone(), prev, count, loss));
                }
            }
        }
    }
    let visited = visited
        .iter()
        .filter_map(|((p, _, c), v)| {
            if *p == g.end && *c >= 4 {
                Some(*v)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    println!("{:?}", visited);
    *visited.iter().min().unwrap()
}

fn part1(input: &In) -> Out {
    search(input, input.start, input.end)
}

fn part2(input: &In) -> Out {
    search2(input, input.start, input.end)
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
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
