use std::collections::HashMap;

use crate::graph::Graph;
use std::hash::Hash;

impl<I, D> Graph<I, D>
where
    I: Clone + Eq + Hash + std::fmt::Debug,
{
    pub fn floyd(&self) -> HashMap<(&I, &I), f32> {
        let mut cost: HashMap<(&I, &I), f32> = HashMap::new();
        for u in self.0.values() {
            for v in self.0.values() {
                if u.key == v.key {
                    cost.insert((&u.key, &v.key), 0.0);
                } else {
                    cost.insert(
                        (&u.key, &v.key),
                        match u.edges.iter().find(|&e| e.0 == v.key) {
                            Some(&(_, c)) => c as f32,
                            None => f32::INFINITY,
                        },
                    );
                }
            }
        }
        for r in self.0.values() {
            for u in self.0.values() {
                for v in self.0.values() {
                    let c1 = cost[&(&u.key, &v.key)];
                    let c2 = cost[&(&u.key, &r.key)];
                    let c3 = cost[&(&r.key, &v.key)];
                    if c2 + c3 < c1 {
                        cost.insert((&u.key, &v.key), c2 + c3);
                    }
                }
            }
        }
        cost
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::*;

    #[test]
    fn test_floyd() {
        let g: Graph<&str, &str> = Graph::new_from_edges(vec![
            ("A", "B", 2.0),
            ("A", "C", 3.0),
            ("B", "D", 10.0),
            ("B", "E", 3.0),
            ("C", "D", 3.0),
            ("C", "E", 5.0),
            ("D", "F", 1.0),
            ("E", "F", 1.0),
        ]);
        let costs = g.floyd();
        for u in g.0.keys() {
            for v in g.0.keys() {
                assert_eq!(
                    costs[&(u, v)],
                    match g.astar(u, v, |_| 1.0) {
                        Some((c, _)) => c as f32,
                        None => f32::INFINITY,
                    }
                );
            }
        }
    }
}
