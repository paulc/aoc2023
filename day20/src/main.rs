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
use util::simplegraph::Graph;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    LOW,
    HIGH,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    ON,
    OFF,
}

#[derive(Debug, Clone)]
enum Node {
    FlipFlop(State),
    Conjunction(HashMap<String, Pulse>),
}

type In = (Graph<String>, HashMap<String, Node>);
type Out = usize;
const PART1_RESULT1: Out = 32000000;
const PART1_RESULT2: Out = 11687500;
const PART2_RESULT: Out = 0;

fn parse_input(input: &mut impl Read) -> std::io::Result<In> {
    let mut graph: Graph<String> = Graph::new();
    let mut rev: Graph<String> = Graph::new();
    let mut state: HashMap<String, Node> = HashMap::new();
    BufReader::new(input).lines().for_each(|l| {
        let l = l.unwrap();
        let s = l.split(" -> ").collect::<Vec<_>>();
        assert!(s.len() == 2);
        let v = if let Some(v) = s[0].strip_prefix(['%', '&']) {
            state.insert(
                v.to_string(),
                match s[0].chars().take(1).next().unwrap() {
                    '%' => Node::FlipFlop(State::OFF),
                    '&' => Node::Conjunction(HashMap::new()),
                    _ => panic!("Invalid Type"),
                },
            );
            v.to_string()
        } else {
            s[0].to_string()
        };
        s[1].split(", ").for_each(|e| {
            graph.add_edge(&v, &e.to_string(), 0);
            rev.add_edge(&e.to_string(), &v, 0);
        });
    });
    // Initialise conjunction nodes from reverse graph
    state.iter_mut().for_each(|(k, v)| match v {
        Node::Conjunction(m) => {
            if let Some(e) = rev.edges(k) {
                e.iter().for_each(|e| {
                    m.insert(e.key().clone(), Pulse::LOW);
                });
            }
        }
        _ => {}
    });
    Ok((graph, state))
}

fn push_button(
    graph: &Graph<String>,
    state: &HashMap<String, Node>,
) -> (usize, usize, HashMap<String, Node>) {
    let mut state = state.clone();
    let mut high: usize = 0;
    let mut low: usize = 1; // Initial LOW from button
    let mut q = VecDeque::from(vec![(
        "button".to_string(),      // From
        "broadcaster".to_string(), // To
        Pulse::LOW,                // Pulse
    )]);
    while let Some((from, to, pulse)) = q.pop_front() {
        let edge_iter = graph.edges(&to).unwrap().iter().map(|v| v.key());
        if to == "broadcaster" {
            edge_iter.for_each(|next| {
                q.push_back((to.clone(), next.clone(), pulse));
                match pulse {
                    Pulse::LOW => low += 1,
                    Pulse::HIGH => high += 1,
                }
            });
        } else {
            match state.get_mut(&to) {
                Some(Node::FlipFlop(s)) => {
                    if pulse == Pulse::LOW {
                        match s {
                            State::OFF => {
                                *s = State::ON;
                                edge_iter.for_each(|next| {
                                    q.push_back((to.clone(), next.clone(), Pulse::HIGH));
                                    high += 1;
                                });
                            }
                            State::ON => {
                                *s = State::OFF;
                                edge_iter.for_each(|next| {
                                    q.push_back((to.clone(), next.clone(), Pulse::LOW));
                                    low += 1;
                                });
                            }
                        }
                    }
                }
                Some(Node::Conjunction(v)) => {
                    v.insert(from.clone(), pulse);
                    if v.iter().all(|(_, v)| v == &Pulse::HIGH) {
                        edge_iter.for_each(|next| {
                            q.push_back((to.clone(), next.clone(), Pulse::LOW));
                            low += 1;
                        });
                    } else {
                        edge_iter.for_each(|next| {
                            q.push_back((to.clone(), next.clone(), Pulse::HIGH));
                            high += 1;
                        });
                    }
                }
                _ => {}
            }
        }
    }
    (high, low, state)
}

fn draw_graph(graph: &Graph<String>, state: &HashMap<String, Node>) {
    println!("digraph g {{");
    for (vertex, edges) in graph.iter() {
        let shape = match vertex.as_str() {
            "broadcast" => "doublecircle",
            "rx" => "star",
            _ => match state.get(vertex) {
                Some(Node::FlipFlop(_)) => "rectangle",
                Some(Node::Conjunction(_)) => "diamond",
                None => "circle",
            },
        };
        println!("{} [shape={}];", vertex, shape);
        for e in edges {
            println!("{} -> {};", vertex, e.key());
        }
    }
    println!("}}");
}

fn part1((graph, state): &In) -> Out {
    let mut high: usize = 0;
    let mut low: usize = 0;
    let mut h: usize = 0;
    let mut l: usize = 0;
    let mut state = state.clone();
    for _ in (0..1000) {
        (h, l, state) = push_button(graph, &state);
        high += h;
        low += l;
    }
    high * low
}

fn part2((graph, state): &In) -> Out {
    // draw_graph(graph, state);
    let mut state = state.clone();
    let mut count: usize = 0;
    loop {
        count += 1;
        (_, _, state) = push_button(graph, &state);
        /*
        println!("Cycle: {}", count);
        for (k, v) in &state {
            println!("{} : {:?}", k, v);
        }
        */
        if let Some(Node::Conjunction(v)) = state.get("lq") {
            //if count % 1000 == 0 {
            //    println!("{}: {:?}", count, v);
            //}
            if v.iter().all(|(_, p)| p == &Pulse::HIGH) {
                println!("{} --> {:?}", count, v);
            }
        }
    }
    0
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
    let input1 = parse_input(&mut TESTDATA1.trim_matches('\n').as_bytes()).unwrap();
    assert_eq!(part1(&input1), PART1_RESULT1);
    let input2 = parse_input(&mut TESTDATA2.trim_matches('\n').as_bytes()).unwrap();
    assert_eq!(part1(&input2), PART1_RESULT2);
}

#[cfg(test)]
const TESTDATA1: &str = r"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

#[cfg(test)]
const TESTDATA2: &str = r"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";
