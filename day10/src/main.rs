#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use util::grid::Grid;
use util::point::*;

type In = Grid<char>;
type Out = usize;
const PART1_RESULT: Out = 8;
const PART2_RESULT1: Out = 4;
const PART2_RESULT2: Out = 8;
const PART2_RESULT3: Out = 10;

fn parse_input(input: &mut impl Read) -> In {
    Grid::from(
        BufReader::new(input)
            .lines()
            .map(|l| l.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
}

const PIPES: [(char, (Offset, Offset)); 6] = [
    ('|', (UP, DOWN)),
    ('-', (LEFT, RIGHT)),
    ('L', (UP, RIGHT)),
    ('J', (UP, LEFT)),
    ('7', (DOWN, LEFT)),
    ('F', (DOWN, RIGHT)),
];

fn map_direction(d: Offset, pipe: &char) -> Option<Offset> {
    // Flip input direction
    let d = match (d) {
        LEFT => RIGHT,
        RIGHT => LEFT,
        UP => DOWN,
        DOWN => UP,
        _ => panic!("Invalid Direction"),
    };
    if let Some((_, (a, b))) = PIPES.iter().filter(|&p| *pipe == p.0).last() {
        match (d == *a, d == *b) {
            (true, false) => Some(*b),
            (false, true) => Some(*a),
            _ => None,
        }
    } else {
        None
    }
}

fn debug_pos(p: Point, o: Offset) -> String {
    format!(
        "({},{}) -> {}",
        p.x,
        p.y,
        match o {
            LEFT => "LEFT",
            RIGHT => "RIGHT",
            UP => "UP",
            DOWN => "DOWN",
            _ => "?",
        }
    )
}

fn find_start(input: &In) -> (Point, Vec<Offset>) {
    let start = input.find(&'S').first().unwrap().clone();
    let direction = ADJACENT
        .iter()
        .filter(|&o| {
            if let Some(p) = input.get(start + *o) {
                map_direction(*o, p).is_some()
            } else {
                false
            }
        })
        .cloned()
        .collect::<Vec<_>>();
    (start, direction)
}

fn find_path(input: &In, start: Point, direction: Offset) -> Vec<Point> {
    let mut out: Vec<Point> = Vec::new();
    let mut p = start;
    let mut d = direction;
    loop {
        p = p + d;
        out.push(p.clone());
        if p == start {
            break;
        }
        d = map_direction(d, input.get(p).unwrap()).unwrap();
    }
    out
}

fn part1(input: &In) -> Out {
    let (start, direction) = find_start(input);
    (find_path(input, start, direction[0]).len() + 1) / 2
}

fn part2(input: &In) -> Out {
    let (start, direction) = find_start(input);

    let mut clean = Grid::empty(input.start, input.end, '.');
    find_path(input, start, direction[0]).iter().for_each(|&p| {
        clean.set(p, *input.get(p).unwrap()).unwrap();
    });
    // Get correct start character
    clean.set(
        start,
        match (direction[0], direction[1]) {
            (UP, RIGHT) | (RIGHT, UP) => 'L',
            (UP, LEFT) | (LEFT, UP) => 'J',
            (UP, DOWN) | (DOWN, UP) => '|',
            (RIGHT, DOWN) | (DOWN, RIGHT) => 'F',
            (RIGHT, LEFT) | (LEFT, RIGHT) => '-',
            (DOWN, LEFT) | (LEFT, DOWN) => '7',
            _ => panic!("Invalid start direction"),
        },
    );
    let mut count = 0;
    for y in 0..input.size.dy {
        let mut inside = false;
        let mut prev: Option<char> = None;
        for x in 0..input.size.dx {
            match (prev, clean.get(Point::new(x, y)).unwrap()) {
                (_, '|') => inside = !inside,
                (None, 'F') => prev = Some('F'),
                (Some('F'), '7') => prev = None,
                (Some('F'), 'J') => {
                    inside = !inside;
                    prev = None
                }
                (None, 'L') => prev = Some('L'),
                (Some('L'), 'J') => prev = None,
                (Some('L'), '7') => {
                    inside = !inside;
                    prev = None
                }
                (_, '.') => {
                    if inside {
                        // clean.set(Point::new(x, y), 'I').unwrap();
                        count += 1
                    }
                }
                _ => {}
            }
        }
    }
    println!("{}", clean);
    count
}

fn main() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;
    let input = parse_input(&mut f);
    println!("Part1: {:?}", part1(&input));
    println!("Part2: {:?}", part2(&input));
    Ok(())
}

#[test]
fn test_part1() {
    let input = parse_input(&mut TESTDATA.trim_matches('\n').as_bytes());
    assert_eq!(part1(&input), PART1_RESULT);
}

#[test]
fn test_part2() {
    let input1 = parse_input(&mut TESTDATA2_1.trim_matches('\n').as_bytes());
    let input2 = parse_input(&mut TESTDATA2_2.trim_matches('\n').as_bytes());
    let input3 = parse_input(&mut TESTDATA2_3.trim_matches('\n').as_bytes());
    assert_eq!(part2(&input1), PART2_RESULT1);
    assert_eq!(part2(&input2), PART2_RESULT2);
    assert_eq!(part2(&input3), PART2_RESULT3);
}

#[cfg(test)]
const TESTDATA: &str = "
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

#[cfg(test)]
const TESTDATA2_1: &str = "
..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........
";

#[cfg(test)]
const TESTDATA2_2: &str = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

#[cfg(test)]
const TESTDATA2_3: &str = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
