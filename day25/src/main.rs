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
use util::simplegraph::Graph;

#[derive(Debug, Clone)]
struct T();

type In = Vec<(String, String, u32)>;
type Out = usize;
const PART1_RESULT: Out = 54;
// Testdata: ("hfx", "pzl"), ("bvb", "cmg"), ("nvd", "jqt")
// Puzzle:   ("xsl", "tpb"), ("qpg", "lrd"), ("bmx", "zlv")

fn parse_input(input: &mut impl Read) -> std::io::Result<In> {
    let data = BufReader::new(input)
        .lines()
        .flat_map(|l| {
            let l = l.unwrap();
            let (a, b) = l.split_once(": ").unwrap();
            b.trim()
                .split_whitespace()
                .map(|c| (a.to_string(), c.to_string(), 1 as u32))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok(data)
}

fn reachable(g: &Graph<String>, start: &String) -> HashSet<String> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut q: Vec<String> = vec![start.clone()];
    while let Some(v) = q.pop() {
        if !visited.contains(&v) {
            visited.insert(v.clone());
            g.edges(&v)
                .unwrap()
                .iter()
                .for_each(|e| q.push(e.key().clone()));
        }
    }
    visited
}

// Returns sorted list of most visited nodes in 'long' paths
fn find_most_visited_paths(g: &Graph<String>, nodes: &Vec<&String>) -> Vec<(String, usize)> {
    // Node instance counter
    let mut counter: HashMap<String, usize> = HashMap::new();
    for start in nodes {
        // Get paths from start node to all other nodes
        let (costs, from) = g.astar_all(start, |_| 1);
        // Find longest path
        let max = costs.iter().fold(0, |acc, (_, &v)| acc.max(v));
        // Filter costs to only include paths > max - 2
        // (assume that long paths will be across partition)
        let long = costs
            .iter()
            .filter(|(_, &v)| v > max - 2)
            .collect::<Vec<_>>();
        // For each long destination calculate path and
        // count instances (we assume that partition nodes
        // will be visited more often)
        for v in &long {
            let mut current = v.0.clone();
            let mut path = vec![current.clone()];
            while let Some(prev) = from.get(&current) {
                *counter.entry(prev.clone()).or_insert(0) += 1;
                current = prev.clone();
            }
        }
    }
    // Sort output
    let mut out = counter.into_iter().collect::<Vec<_>>();
    out.sort_by_key(|(k, v)| v.clone());
    out.reverse();
    out
}

fn part1(input: &In) -> Out {
    let g = Graph::new_from_bidirectional_edges(input.clone());
    let vertices = g.vertices().collect::<Vec<_>>();
    // Get most visited paths from sample of vertices - 20 seems to work
    let most_visited =
        find_most_visited_paths(&g, &vertices.into_iter().take(20).collect::<Vec<_>>());
    let partition = most_visited
        .into_iter()
        .take(6)
        .map(|(n, _)| n)
        .collect::<Vec<_>>();
    let partitioned = input
        .iter()
        .filter(|(a, b, _)| !(partition.contains(a) && partition.contains(b)))
        .cloned()
        .collect::<Vec<_>>();
    // Check reachable nodes from each partition
    let g = Graph::new_from_bidirectional_edges(partitioned);
    let p1 = reachable(&g, &partition[0]);
    // Find node outside p1 partition
    let n2 = partition
        .iter()
        .filter(|&n| !p1.contains(n))
        .next()
        .unwrap();
    let p2 = reachable(&g, n2);
    p1.len() * p2.len()
}

fn main() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;
    let input = parse_input(&mut f)?;
    let p1 = Instant::now();
    eprintln!(
        "Part1: {:?} ({}s)",
        part1(&input),
        p1.elapsed().as_secs_f32()
    );
    Ok(())
}

#[test]
fn test_part1() {
    let input = parse_input(&mut TESTDATA.trim_matches('\n').as_bytes()).unwrap();
    assert_eq!(part1(&input), PART1_RESULT);
}

#[cfg(test)]
const TESTDATA: &str = r"
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";
