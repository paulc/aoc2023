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
const PART2_RESULT: Out = 0;

fn parse_input(input: &mut impl Read) -> std::io::Result<In> {
    let data = BufReader::new(input)
        .lines()
        .flat_map(|l| {
            let l = l.unwrap();
            let (a, b) = l.split_once(": ").unwrap();
            b.trim()
                .split_whitespace()
                .map(|c| (a.to_string(), c.to_string(), 0 as u32))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok(data)
}

fn reachable(g: &Graph<String>, start: &String) -> usize {
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
    visited.len()
}

fn part1(input: &In, partition: &[(&str, &str); 3]) -> Out {
    /*
    // Visualise graph using Graphviz to extract nodes to partition
    // (neato -Tsvg input.dot > input.svg)
    println!(
        "graph g {{\n{}\n}}",
        input
            .iter()
            .map(|(a, b, _)| format!("{} -- {};", a, b))
            .collect::<Vec<_>>()
            .join("\n")
    );
    */
    let partitioned = input
        .iter()
        .filter(|(a, b, _)| {
            !partition
                .iter()
                .any(|(x, y)| (x == a && y == b) || (x == b && y == a))
        })
        .cloned()
        .collect::<Vec<_>>();
    let g = Graph::new_from_bidirectional_edges(partitioned);
    let n1 = reachable(&g, &partition[0].0.to_string());
    let n2 = reachable(&g, &partition[0].1.to_string());
    n1 * n2
}

fn part2(input: &In) -> Out {
    PART2_RESULT
}

fn main() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;
    let input = parse_input(&mut f)?;
    let p1_split = [("xsl", "tpb"), ("qpg", "lrd"), ("bmx", "zlv")];
    let p1 = Instant::now();
    eprintln!(
        "Part1: {:?} ({}s)",
        part1(&input, &p1_split),
        p1.elapsed().as_secs_f32()
    );
    let p2 = Instant::now();
    eprintln!(
        "Part2: {:?} ({}s)",
        part2(&input),
        p2.elapsed().as_secs_f32()
    );
    Ok(())
}

#[test]
fn test_part1() {
    let input = parse_input(&mut TESTDATA.trim_matches('\n').as_bytes()).unwrap();
    let p1_split = [("hfx", "pzl"), ("bvb", "cmg"), ("nvd", "jqt")];
    assert_eq!(part1(&input, &p1_split), PART1_RESULT);
}

#[test]
fn test_part2() {
    let input = parse_input(&mut TESTDATA.trim_matches('\n').as_bytes()).unwrap();
    assert_eq!(part2(&input), PART2_RESULT);
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
