#![allow(unused)]

use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::ops::Range;
use std::time::Instant;
use util::combinations::combinations;

type In = Vec<PV>;
type Out = usize;
const PART1_RESULT: Out = 2;
const PART2_RESULT: Out = 0;

#[derive(Debug, Clone, PartialEq)]
struct P {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct PV {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl PV {
    fn new(px: f64, py: f64, pz: f64, vx: f64, vy: f64, vz: f64) -> Self {
        PV {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        }
    }
    fn pos(&self, t: f64) -> P {
        P {
            x: self.px + self.vx * t,
            y: self.py + self.vy * t,
            z: self.pz + self.vz * t,
        }
    }
    fn time_at(&self, p2: &P) -> f64 {
        // Only correct if line crosses point
        if self.px == p2.x {
            (p2.y - self.py) / self.vy
        } else {
            (p2.x - self.px) / self.vx
        }
    }
    fn intersects_box2d(&self, x_range: &Range<f64>, y_range: &Range<f64>) -> bool {
        let t_x1 = (x_range.start - self.px) / self.vx;
        let t_x2 = (x_range.end - self.px) / self.vx;
        let t_y1 = (y_range.start - self.py) / self.vy;
        let t_y2 = (y_range.end - self.py) / self.vy;
        // println!("{} {} {} {}", t_x1, t_x2, t_y1, t_y2);
        (t_x1 > 0.0 && y_range.contains(&self.pos(t_x1).y))
            || (t_x2 > 0.0 && y_range.contains(&self.pos(t_x2).y))
            || (t_y1 > 0.0 && x_range.contains(&self.pos(t_y1).x))
            || (t_y2 > 0.0 && x_range.contains(&self.pos(t_y2).x))
    }
    fn intersects2d(&self, other: &Self) -> Option<P> {
        // Line 1 (x1,y1) -> (x2,y2)
        // Line 2 (x3,y3) -> (x4,y4)
        let (x1, y1, x3, y3) = (self.px, self.py, other.px, other.py);
        // Calculate second point on lines by going forward arbitrary time
        let P { x: x2, y: y2, z: _ } = self.pos(1000.0);
        let P { x: x4, y: y4, z: _ } = other.pos(1000.0);
        let px = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4))
            / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));
        let py = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4))
            / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));
        if px.is_finite() && py.is_finite() {
            let p = P {
                x: px,
                y: py,
                z: 0.0,
            };
            if self.time_at(&p) < 0.0 || other.time_at(&p) < 0.0 {
                None
            } else {
                Some(p)
            }
        } else {
            None
        }
    }
}

impl Display for PV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{} @ {},{},{}",
            self.px, self.py, self.pz, self.vx, self.vy, self.vz
        )
    }
}

fn parse_input(input: &mut impl Read) -> std::io::Result<In> {
    let data = BufReader::new(input)
        .lines()
        .map(|l| {
            let n = l
                .unwrap()
                .split([',', '@'])
                .filter_map(|s| s.trim().parse::<f64>().ok())
                .collect::<Vec<_>>();
            PV::new(n[0], n[1], n[2], n[3], n[4], n[5])
        })
        .collect::<Vec<_>>();
    Ok(data)
}

fn part1(input: &In, x_range: Range<f64>, y_range: Range<f64>) -> Out {
    // Filter out lines which never cross target
    let lines = input
        .iter()
        .filter(|pv| pv.intersects_box2d(&x_range, &y_range))
        .collect::<Vec<_>>();

    combinations(&lines, 2)
        .par_iter()
        .filter_map(|c| c[0].intersects2d(&c[1]))
        .filter(|p| x_range.contains(&p.x) && y_range.contains(&p.y))
        .count()
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
        part1(
            &input,
            200000000000000.0..400000000000000.0,
            200000000000000.0..400000000000000.0
        ),
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
    assert_eq!(part1(&input, 7.0..27.0, 7.0..27.0), PART1_RESULT);
}

#[test]
fn test_part2() {
    let input = parse_input(&mut TESTDATA.trim_matches('\n').as_bytes()).unwrap();
    assert_eq!(part2(&input), PART2_RESULT);
}

#[cfg(test)]
const TESTDATA: &str = r"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";
