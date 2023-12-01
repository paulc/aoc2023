use crate::graph::Graph;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
struct OrdF32(f32);

impl PartialEq for OrdF32 {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < std::f32::EPSILON
    }
}

impl Eq for OrdF32 {}

impl PartialOrd for OrdF32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.0
                .partial_cmp(&other.0)
                .unwrap_or(std::cmp::Ordering::Equal),
        )
    }
}

impl Ord for OrdF32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct V<I>(I, OrdF32)
where
    I: Eq;

impl<I> PartialOrd for V<I>
where
    I: Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&other.1).map(|o| match o {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        })
    }
}
impl<I> Ord for V<I>
where
    I: Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.1.cmp(&other.1) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl<I, D> Graph<I, D>
where
    I: Clone + Eq + Hash,
{
    pub fn astar<F>(&self, start: &I, target: &I, h: F) -> Option<(f32, Vec<I>)>
    where
        F: Fn(&I) -> f32,
    {
        let mut open: BinaryHeap<V<I>> = BinaryHeap::new();
        let mut from: HashMap<I, I> = HashMap::new();
        let mut score: HashMap<I, f32> = HashMap::new();
        open.push(V(start.clone(), OrdF32(h(&start))));
        score.insert(start.clone(), 0.0);
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
            if let Some(v) = self.get(&current.0) {
                for (n, d) in &v.edges {
                    let tentative = score[&current.0] + d;
                    if tentative < *score.get(&n).unwrap_or(&f32::INFINITY) {
                        from.insert(n.clone(), current.0.clone());
                        score.insert(n.clone(), tentative);
                        open.push(V(n.clone(), OrdF32(tentative + h(&n))));
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::*;
    use std::cmp::{max, min};
    use std::fs;

    #[test]
    fn test_astar_simple() {
        let g: Graph<&str, ()> = Graph::new_from_edges(vec![
            ("A", "B", 2.0),
            ("A", "C", 3.0),
            ("B", "D", 10.0),
            ("B", "E", 3.0),
            ("C", "D", 3.0),
            ("C", "E", 5.0),
            ("D", "F", 1.0),
            ("E", "F", 1.0),
        ]);
        assert_eq!(
            g.astar(&"A", &"F", |_| 1.0),
            Some((6.0, vec!["A", "B", "E", "F"]))
        );
    }

    // From aoc2021/day15
    fn make_graph(path: &str) -> Graph<(usize, usize), ()> {
        let a = fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(|l| {
                l.as_bytes()
                    .iter()
                    .map(|b| (*b - b'0') as f32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut g: Graph<(usize, usize), ()> = Graph::new();
        for y in 0..a.len() {
            for x in 0..a[0].len() {
                g.add_vertex(Vertex::new(
                    (x, y),
                    (),
                    adj(&a, (x, y))
                        .iter()
                        .map(|&(x, y)| ((x, y), a[y][x] as f32))
                        .collect::<Vec<_>>(),
                ))
            }
        }
        g
    }

    fn adj(a: &Vec<Vec<f32>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
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
        let g = make_graph("testdata/grid.txt");
        let (cost, path) = g
            .astar(&(0, 0), &(9, 9), |&(x, y)| md((x, y), (9, 9)) as f32)
            .unwrap();
        assert_eq!((cost, path.len()), (40.0, 19));
    }

    #[test]
    fn test_astar_grid_large() {
        let g = make_graph("testdata/grid_large.txt");
        let (cost, _path) = g
            .astar(&(0, 0), &(99, 99), |&(x, y)| md((x, y), (99, 99)) as f32)
            .unwrap();
        assert_eq!(cost, 602.0);
    }
}
