use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub struct Graph<V>(HashMap<V, Vec<(V, u32)>>)
where
    V: Display + Clone + Eq + Hash;

impl<V> Graph<V>
where
    V: Display + Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn add_vertex(&mut self, vertex: V, edges: Vec<(V, u32)>) {
        self.0.entry(vertex).or_insert_with(|| edges);
    }
    pub fn add_edge(&mut self, from: &V, to: &V, cost: u32) {
        // Create 'to' if necessary
        self.0.entry(to.clone()).or_insert_with(|| vec![]);
        // Add edge
        self.0
            .entry(from.clone())
            .or_insert_with(|| vec![])
            .push((to.clone(), cost));
    }
    pub fn vertices(&self) -> impl Iterator<Item = &V> {
        self.0.keys()
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
                    .map(|(to, cost)| format!("{}({})", to, cost))
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
        g.add_vertex("AA", vec![("BB", 1), ("CC", 2)]);
        g.add_vertex("BB", vec![("DD", 5)]);
        g.add_vertex("CC", vec![("DD", 1)]);
        g.add_vertex("DD", vec![("EE", 1)]);
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
}
