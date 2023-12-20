use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;

use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct Edge<V: Eq>(V, u32);

impl<V: Eq> Edge<V> {
    pub fn key(&self) -> &V {
        &self.0
    }
}

// Sort edges in reverse cost order
impl<V: Eq> Ord for Edge<V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1) // Compare the u32 field in reverse order
    }
}

impl<V: Eq> PartialOrd for Edge<V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq)]
pub struct Graph<V>(HashMap<V, Vec<Edge<V>>>)
where
    V: Clone + Eq + Hash;

impl<V> Graph<V>
where
    V: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn edges(&self, v: &V) -> Option<&Vec<Edge<V>>> {
        self.0.get(v)
    }
    pub fn add_vertex(&mut self, vertex: V, edges: Vec<Edge<V>>) {
        self.0.entry(vertex).or_insert_with(|| edges);
    }
    pub fn add_edge(&mut self, from: &V, to: &V, cost: u32) {
        // Create 'to' if necessary
        self.0.entry(to.clone()).or_insert_with(|| vec![]);
        // Add edge
        self.0
            .entry(from.clone())
            .or_insert_with(|| vec![])
            .push(Edge(to.clone(), cost));
    }
    pub fn vertices(&self) -> impl Iterator<Item = &V> {
        self.0.keys()
    }
    pub fn iter(&self) -> impl Iterator<Item = (&V, &Vec<Edge<V>>)> {
        self.0.iter()
    }
    pub fn new_from_edges(edges: Vec<(V, V, u32)>) -> Self {
        let mut out = Self::new();
        for (from, to, cost) in edges {
            out.add_edge(&from, &to, cost);
        }
        out
    }
    pub fn new_from_bidirectional_edges(edges: Vec<(V, V, u32)>) -> Self {
        let mut out = Self::new();
        for (from, to, cost) in edges {
            out.add_edge(&from, &to, cost);
            out.add_edge(&to, &from, cost);
        }
        out
    }
    pub fn astar<F>(&self, start: &V, target: &V, h: F) -> Option<(u32, Vec<V>)>
    where
        F: Fn(&V) -> u32,
    {
        let mut open: BinaryHeap<Edge<V>> = BinaryHeap::new();
        let mut from: HashMap<V, V> = HashMap::new();
        let mut score: HashMap<V, u32> = HashMap::new();
        open.push(Edge(start.clone(), h(&start)));
        score.insert(start.clone(), 0);
        while let Some(current) = open.pop() {
            if current.0 == *target {
                if let Some(cost) = score.get(&target) {
                    let mut current = &current.0;
                    let mut path = vec![current.clone()];
                    while let Some(prev) = from.get(current) {
                        path.push(prev.clone());
                        current = prev;
                    }
                    path.reverse();
                    return Some((cost.clone(), path));
                } else {
                    return None;
                }
            }
            if let Some(v) = self.0.get(&current.0) {
                for Edge(next, cost) in v.iter() {
                    let tentative = score[&current.0] + cost;
                    if tentative < *score.get(&next).unwrap_or(&u32::MAX) {
                        from.insert(next.clone(), current.0.clone());
                        score.insert(next.clone(), tentative);
                        open.push(Edge(next.clone(), tentative + h(&next)));
                    }
                }
            }
        }
        None
    }
    pub fn floyd(&self) -> HashMap<(&V, &V), Option<u32>> {
        let mut cost: HashMap<(&V, &V), Option<u32>> = HashMap::new();
        for u in self.0.iter() {
            for v in self.0.iter() {
                if u.0 == v.0 {
                    cost.insert((&u.0, &v.0), Some(0));
                } else {
                    cost.insert(
                        (&u.0, &v.0),
                        match u.1.iter().find(|&e| &e.0 == v.0) {
                            Some(e) => Some(e.1),
                            None => None,
                        },
                    );
                }
            }
        }
        for r in self.0.iter() {
            for u in self.0.iter() {
                for v in self.0.iter() {
                    let (c1, c2, c3) = (cost[&(u.0, v.0)], cost[&(u.0, r.0)], cost[&(r.0, v.0)]);
                    if let (None, Some(c2), Some(c3)) = (c1, c2, c3) {
                        // No existing route from u-v but we do have u->r->v
                        cost.insert((&u.0, &v.0), Some(c2 + c3));
                    } else if let (Some(c1), Some(c2), Some(c3)) = (c1, c2, c3) {
                        if c2 + c3 < c1 {
                            cost.insert((&u.0, &v.0), Some(c2 + c3));
                        }
                    }
                }
            }
        }
        cost
    }
}

impl<V> Graph<V>
where
    V: Display + Clone + Eq + Hash,
{
    pub fn to_dot(&self) {
        println!("digraph g {{");
        for (vertex, edge) in &self.0 {
            for e in edge {
                println!("{} -> {};", vertex, e.key());
            }
        }
        println!("}}");
    }
}

impl<V> Display for Graph<V>
where
    V: Display + Clone + Eq + Hash,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (vertex, edge) in &self.0 {
            writeln!(
                f,
                "{} -> {}",
                vertex,
                edge.iter()
                    .map(|Edge(to, cost)| format!("{}({})", to, cost))
                    .collect::<Vec<_>>()
                    .join(",")
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::{max, min};
    use std::fs;

    fn make_graph() -> Graph<&'static str> {
        Graph::new_from_edges(vec![
            ("DD", "EE", 1),
            ("BB", "DD", 5),
            ("AA", "BB", 1),
            ("AA", "CC", 2),
            ("CC", "DD", 1),
        ])
    }

    #[test]
    fn test_graph_new() {
        let mut g: Graph<&'static str> = Graph::new();
        g.add_vertex("AA", vec![Edge("BB", 1), Edge("CC", 2)]);
        g.add_vertex("BB", vec![Edge("DD", 5)]);
        g.add_vertex("CC", vec![Edge("DD", 1)]);
        g.add_vertex("DD", vec![Edge("EE", 1)]);
        g.add_vertex("EE", vec![]);
        assert_eq!(g, make_graph());
    }

    #[test]
    fn test_graph_from_edges() {
        let g = make_graph();
        let s = g.to_string();
        assert_eq!(
            {
                let mut l = s.lines().collect::<Vec<_>>();
                l.sort();
                l
            },
            vec![
                "AA -> BB(1),CC(2)",
                "BB -> DD(5)",
                "CC -> DD(1)",
                "DD -> EE(1)",
                "EE -> "
            ]
        );
    }

    #[test]
    fn test_graph_from_bidirectional_edges() {
        let g = Graph::new_from_bidirectional_edges(vec![
            ("AA", "BB", 1),
            ("AA", "CC", 2),
            ("BB", "DD", 3),
            ("CC", "DD", 4),
        ]);
        let s = g.to_string();
        assert_eq!(
            {
                let mut l = s.lines().collect::<Vec<_>>();
                l.sort();
                l
            },
            vec![
                "AA -> BB(1),CC(2)",
                "BB -> AA(1),DD(3)",
                "CC -> AA(2),DD(4)",
                "DD -> BB(3),CC(4)",
            ]
        );
    }

    #[test]
    fn test_graph_vertices() {
        let g = make_graph();
        assert_eq!(
            {
                let mut v = g.vertices().cloned().collect::<Vec<_>>();
                v.sort();
                v
            },
            vec!["AA", "BB", "CC", "DD", "EE"]
        );
    }

    #[test]
    fn test_astar_simple() {
        let g = Graph::new_from_edges(vec![
            ("A", "B", 2),
            ("A", "C", 3),
            ("B", "D", 10),
            ("B", "E", 3),
            ("C", "D", 3),
            ("C", "E", 5),
            ("D", "F", 1),
            ("E", "F", 1),
            ("F", "A", 1),
        ]);
        assert_eq!(
            g.astar(&"A", &"F", |_| 1),
            Some((6, vec!["A", "B", "E", "F"]))
        );
        assert_eq!(
            g.astar(&"B", &"A", |_| 1),
            Some((5, vec!["B", "E", "F", "A"]))
        );
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct Point(usize, usize);

    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    // From aoc2021/day15
    fn make_graph_grid(path: &str) -> Graph<Point> {
        let a = fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(|l| {
                l.as_bytes()
                    .iter()
                    .map(|b| (*b - b'0') as u32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut g: Graph<Point> = Graph::new();
        for y in 0..a.len() {
            for x in 0..a[0].len() {
                g.add_vertex(
                    Point(x, y),
                    adj(&a, (x, y))
                        .iter()
                        .map(|&(x, y)| Edge(Point(x, y), a[y][x] as u32))
                        .collect::<Vec<_>>(),
                )
            }
        }
        g
    }

    fn adj(a: &Vec<Vec<u32>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let (x_max, y_max) = (a[0].len() as i32, a.len() as i32);
        let (x, y) = (x as i32, y as i32);
        let mut out = vec![];
        for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if x + dx >= 0 && x + dx < x_max && y + dy >= 0 && y + dy < y_max {
                out.push(((x + dx) as usize, (y + dy) as usize));
            }
        }
        out
    }

    fn md((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
        (max(x1, x2) - min(x1, x2)) + (max(y1, y2) - min(y1, y2))
    }

    #[test]
    fn test_astar_grid() {
        let g = make_graph_grid("testdata/grid.txt");
        let (cost, path) = g
            .astar(&Point(0, 0), &Point(9, 9), |&Point(x, y)| {
                md((x, y), (9, 9)) as u32
            })
            .unwrap();
        assert_eq!((cost, path.len()), (40, 19));
    }

    #[test]
    fn test_astar_grid_large() {
        let g = make_graph_grid("testdata/grid_large.txt");
        let (cost, _) = g
            .astar(&Point(0, 0), &Point(99, 99), |&Point(x, y)| {
                md((x, y), (99, 99)) as u32
            })
            .unwrap();
        assert_eq!(cost, 602);
    }

    #[test]
    fn test_floyd() {
        let g = Graph::new_from_edges(vec![
            ("A", "B", 2),
            ("A", "C", 3),
            ("B", "D", 10),
            ("B", "E", 3),
            ("C", "D", 3),
            ("C", "E", 5),
            ("D", "F", 1),
            ("E", "F", 1),
            ("F", "A", 1),
        ]);
        let costs = g.floyd();
        for u in g.0.keys() {
            for v in g.0.keys() {
                assert_eq!(
                    costs[&(u, v)],
                    match g.astar(u, v, |_| 1) {
                        Some((c, _)) => Some(c),
                        None => None,
                    }
                );
            }
        }
    }
}
