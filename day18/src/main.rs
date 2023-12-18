#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::num::ParseIntError;
use std::time::Instant;
use util::grid::Grid;
use util::point::*;

type In = Vec<(Offset, usize, String)>;
type Out = usize;
const PART1_RESULT: Out = 62;
const PART2_RESULT: Out = 952408144115;

fn parse_input(input: &mut impl Read) -> std::io::Result<In> {
    let data = BufReader::new(input)
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let f = l.split_whitespace().collect::<Vec<_>>();
            (
                match f[0] {
                    "U" => UP,
                    "D" => DOWN,
                    "L" => LEFT,
                    "R" => RIGHT,
                    _ => panic!("Invalid Direction"),
                },
                f[1].parse::<usize>().unwrap(),
                f[2][2..8].to_string(),
            )
        })
        .collect::<Vec<_>>();
    Ok(data)
}

// bruteforce fill for part 1
fn area(points: &Vec<Point>) -> usize {
    let start = Point::new(
        points.iter().map(|p| p.x).min().unwrap(),
        points.iter().map(|p| p.y).min().unwrap(),
    ) + Offset::new(-1, -1);
    let end = Point::new(
        points.iter().map(|p| p.x).max().unwrap(),
        points.iter().map(|p| p.y).max().unwrap(),
    ) + Offset::new(1, 1);
    let mut g = Grid::empty(&start, &end, '.');
    points
        .windows(2)
        .for_each(|p| g.draw_line(&p[0], &p[1], '#').unwrap());
    // Fill outside
    g.fill(&g.start.clone(), &vec!['#'], Some(&'o'));
    g.find(&'.').len() + g.find(&'#').len()
}

// points should be closed loop
fn shoelace(points: &Vec<Point>) -> usize {
    assert!(points.len() > 0);
    assert_eq!(points[0], points[points.len() - 1]);
    let a = points
        .windows(2)
        .fold(0, |acc, p| acc + (p[0].x * p[1].y - p[1].x * p[0].y));
    let l = points.windows(2).fold(0, |acc, p| {
        acc + if (p[0].x == p[1].x) {
            i64::max(p[0].y, p[1].y) - i64::min(p[0].y, p[1].y)
        } else {
            i64::max(p[0].x, p[1].x) - i64::min(p[0].x, p[1].x)
        }
    });
    (a.abs() + l) as usize / 2 + 1
}

fn part1(input: &In) -> Out {
    let mut p = Point::new(0, 0);
    let mut points = vec![Point::new(0, 0)];
    input.iter().for_each(|&(d, n, _)| {
        p = p + (d * (n as i64));
        points.push(p);
    });
    shoelace(&points)
}

fn part2(input: &In) -> Out {
    let mut p = Point::new(0, 0);
    let mut points = vec![Point::new(0, 0)];
    input.iter().for_each(|(_, _, h)| {
        let n = i64::from_str_radix(&h[0..5], 16).unwrap();
        let d = match h.chars().last().unwrap() {
            '0' => RIGHT,
            '1' => DOWN,
            '2' => LEFT,
            '3' => UP,
            _ => panic!("Invalid direction"),
        };
        p = p + (d * n);
        points.push(p);
    });
    shoelace(&points)
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
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
