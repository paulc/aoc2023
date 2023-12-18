#![allow(unused)]

use std::collections::BinaryHeap;
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

fn available(prev: Offset, count: u32, min_straight: u32, max_straight: u32) -> Vec<Offset> {
    if count < min_straight {
        vec![prev]
    } else if count == max_straight {
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
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    p: Point,
    prev: Offset,
    count: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct StateCost {
    state: State,
    cost: u32,
}

impl Ord for StateCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // Compare the u32 field in reverse order
    }
}

impl PartialOrd for StateCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn astar(g: &Grid<u8>, min_straight: u32, max_straight: u32) -> Option<(u32, Vec<Point>)> {
    let mut open: BinaryHeap<StateCost> = BinaryHeap::new();
    let mut from: HashMap<State, State> = HashMap::new();
    let mut score: HashMap<State, u32> = HashMap::new();
    // Initial valid directions are R/D (need to add both states)
    for d in [RIGHT, DOWN] {
        let state = State {
            p: g.start,
            prev: d,
            count: 0,
        };
        open.push(StateCost {
            state: state.clone(),
            cost: g.start.manhattan(&g.end),
        });
        score.insert(state.clone(), 0);
    }
    while let Some(current) = open.pop() {
        if current.state.p == g.end {
            // We cant break here as we might get a better state
            continue;
        }
        for d in available(
            current.state.prev,
            current.state.count,
            min_straight,
            max_straight,
        ) {
            let p2 = current.state.p + d;
            if g.check_bounds(p2) {
                let next = State {
                    p: p2,
                    prev: d,
                    count: if d != current.state.prev {
                        1
                    } else {
                        current.state.count + 1
                    },
                };
                let tentative = score[&current.state] + *g.get(next.p).unwrap() as u32;
                if tentative < *score.get(&next).unwrap_or(&u32::MAX) {
                    from.insert(next.clone(), current.state.clone());
                    score.insert(next.clone(), tentative);
                    open.push(StateCost {
                        state: next,
                        cost: tentative + current.state.p.manhattan(&g.end),
                    });
                }
            }
        }
    }
    // Filter valid states
    let mut states = score
        .iter()
        .filter_map(|(s, v)| {
            if s.p == g.end && s.count >= min_straight {
                Some((s.clone(), v.clone()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if states.is_empty() {
        None
    } else {
        // Get best
        states.sort_by_key(|&(_, v)| v);
        let best = states.first().unwrap().clone();
        // Trace path
        let mut current = best.0.clone();
        let mut path = vec![current.clone()];
        while let Some(prev) = from.get(&current) {
            path.push(prev.clone());
            current = prev.clone();
        }
        Some((best.1, path.iter().map(|s| s.p).collect::<Vec<_>>()))
    }
}

// Simple BFS - much slower than astar
fn search(g: &Grid<u8>, min_straight: u32, max_straight: u32) -> u32 {
    let mut costs: Vec<usize> = Vec::new();
    let mut visited: HashMap<(Point, Offset, u32), u32> = HashMap::new();
    let mut q: VecDeque<(Point, Offset, u32, u32)> = VecDeque::new();
    q.push_back((g.start.clone(), RIGHT, 0, 0));
    q.push_back((g.start.clone(), DOWN, 0, 0));
    visited.insert((g.start.clone(), RIGHT, 0), 0);
    while let Some((p, prev, count, loss)) = q.pop_front() {
        if p == g.end {
            continue;
        }
        for d in available(prev, count, min_straight, max_straight) {
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
    visited
        .iter()
        .filter_map(|((p, _, c), v)| {
            if *p == g.end && *c >= min_straight {
                Some(v)
            } else {
                None
            }
        })
        .min()
        .unwrap()
        .clone()
}

fn part1(input: &In) -> Out {
    let (loss, path) = astar(input, 1, 3).unwrap();
    loss
}

fn part2(input: &In) -> Out {
    let (loss, path) = astar(input, 4, 10).unwrap();
    loss
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
