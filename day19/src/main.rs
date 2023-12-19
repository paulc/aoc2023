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

type In = (Vec<Part>, HashMap<String, Vec<Opcode>>);
type Out = i32;
const PART1_RESULT: Out = 19114;
const PART2_RESULT: Out = 0;

#[derive(Debug, PartialEq, Eq)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl TryFrom<&str> for Part {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts = s
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .collect::<Vec<_>>();
        if parts.len() != 4 {
            Err("Invalid part")
        } else {
            Ok(Part {
                x: parts[0][2..].parse::<i32>().map_err(|e| "ParseIntError")?,
                m: parts[1][2..].parse::<i32>().map_err(|e| "ParseIntError")?,
                a: parts[2][2..].parse::<i32>().map_err(|e| "ParseIntError")?,
                s: parts[3][2..].parse::<i32>().map_err(|e| "ParseIntError")?,
            })
        }
    }
}

#[derive(Debug, Clone)]
enum Dest {
    Accept,
    Reject,
    Rule(String),
}

impl From<&str> for Dest {
    fn from(s: &str) -> Self {
        match s {
            "A" => Dest::Accept,
            "R" => Dest::Reject,
            _ => Dest::Rule(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Test {
    Less(i32),
    Greater(i32),
}

impl Test {
    fn run(&self, v: i32) -> bool {
        match self {
            Test::Less(n) => &v < n,
            Test::Greater(n) => &v > n,
        }
    }
}

impl TryFrom<&str> for Test {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match &s[..1] {
            "<" => Ok(Test::Less(
                s[1..].parse::<i32>().map_err(|e| "ParseIntError")?,
            )),
            ">" => Ok(Test::Greater(
                s[1..].parse::<i32>().map_err(|e| "ParseIntError")?,
            )),
            _ => Err("Cant parse test"),
        }
    }
}

#[derive(Debug)]
enum Opcode {
    JX(Test, Dest),
    JM(Test, Dest),
    JA(Test, Dest),
    JS(Test, Dest),
    J(Dest),
}

impl TryFrom<&str> for Opcode {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Some((a, b)) = s.split_once(':') {
            match &a[..1] {
                "x" => Ok(Opcode::JX(Test::try_from(&a[1..])?, Dest::from(b))),
                "m" => Ok(Opcode::JM(Test::try_from(&a[1..])?, Dest::from(b))),
                "a" => Ok(Opcode::JA(Test::try_from(&a[1..])?, Dest::from(b))),
                "s" => Ok(Opcode::JS(Test::try_from(&a[1..])?, Dest::from(b))),
                _ => Err("Unknown"),
            }
        } else {
            Ok(Opcode::J(Dest::from(s)))
        }
    }
}

fn parse_input(input: &mut impl Read) -> std::io::Result<In> {
    let mut workflow: HashMap<String, Vec<Opcode>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();
    let mut parse_rule = true;
    BufReader::new(input).lines().for_each(|l| {
        let l = l.unwrap();
        if parse_rule {
            if l.is_empty() {
                parse_rule = false;
            } else {
                let s = l.split(['{', '}', ',']).collect::<Vec<_>>();
                workflow.insert(
                    s[0].to_string(),
                    s.iter()
                        .skip(1)
                        .filter_map(|&r| {
                            if r.is_empty() {
                                None
                            } else {
                                Some(Opcode::try_from(r).unwrap())
                            }
                        })
                        .collect::<Vec<_>>(),
                );
            }
        } else {
            parts.push(Part::try_from(l.as_str()).unwrap());
        }
    });
    Ok((parts, workflow))
}

fn run(p: &Part, w: &HashMap<String, Vec<Opcode>>) -> bool {
    let mut rule = w.get("in").unwrap();
    loop {
        for op in rule {
            match op {
                Opcode::JX(test, dest) => {
                    if test.run(p.x) {
                        match dest {
                            Dest::Accept => return true,
                            Dest::Reject => return false,
                            Dest::Rule(r) => {
                                rule = w.get(r).unwrap();
                                break;
                            }
                        }
                    }
                }
                Opcode::JM(test, dest) => {
                    if test.run(p.m) {
                        match dest {
                            Dest::Accept => return true,
                            Dest::Reject => return false,
                            Dest::Rule(r) => {
                                rule = w.get(r).unwrap();
                                break;
                            }
                        }
                    }
                }
                Opcode::JA(test, dest) => {
                    if test.run(p.a) {
                        match dest {
                            Dest::Accept => return true,
                            Dest::Reject => return false,
                            Dest::Rule(r) => {
                                rule = w.get(r).unwrap();
                                break;
                            }
                        }
                    }
                }
                Opcode::JS(test, dest) => {
                    if test.run(p.s) {
                        match dest {
                            Dest::Accept => return true,
                            Dest::Reject => return false,
                            Dest::Rule(r) => {
                                rule = w.get(r).unwrap();
                                break;
                            }
                        }
                    }
                }
                Opcode::J(dest) => match dest {
                    Dest::Accept => return true,
                    Dest::Reject => return false,
                    Dest::Rule(r) => {
                        rule = w.get(r).unwrap();
                        break;
                    }
                },
            }
        }
    }
    false
}

fn part1((parts, workflow): &In) -> Out {
    parts
        .iter()
        .filter_map(|p| {
            if run(p, workflow) {
                Some(p.x + p.m + p.a + p.s)
            } else {
                None
            }
        })
        .sum()
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
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
