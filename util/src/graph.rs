pub mod astar;
pub mod bfs;
pub mod dfs;
pub mod floyd;

use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub struct Vertex<I, D>
where
    I: Clone + Eq + Hash,
{
    pub key: I,
    pub data: D,
    pub edges: Vec<(I, f32)>,
}

impl<I, D> Vertex<I, D>
where
    I: Clone + Eq + Hash,
{
    pub fn new(key: I, data: D, edges: Vec<(I, f32)>) -> Self {
        Self { key, data, edges }
    }
    pub fn add_edge(&mut self, to: I, cost: f32) {
        self.edges.push((to, cost))
    }
}

impl<I, D> Display for Vertex<I, D>
where
    I: Display + Clone + Eq + Hash,
    D: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}> -> ", self.key, self.data)?;
        let mut n = self.edges.len();
        for (d, c) in &self.edges {
            write!(f, "[{}]({})", d, c)?;
            n -= 1;
            if n > 0 {
                write!(f, ",")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Graph<I, D>(HashMap<I, Vertex<I, D>>)
where
    I: Clone + Eq + Hash;

impl<I, D> Graph<I, D>
where
    I: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn add_vertex(&mut self, v: Vertex<I, D>) {
        self.0.entry(v.key.clone()).or_insert_with(|| v);
    }
    pub fn vertices(&self) -> impl Iterator<Item = &Vertex<I, D>> {
        self.0.values()
    }
    pub fn get(&self, key: &I) -> Option<&Vertex<I, D>> {
        self.0.get(key)
    }
    pub fn get_mut(&mut self, key: &I) -> Option<&mut Vertex<I, D>> {
        self.0.get_mut(key)
    }
}

impl<I, D> Graph<I, D>
where
    I: Clone + Eq + Hash,
    D: Default,
{
    pub fn new_from_edges(edges: Vec<(I, I, f32)>) -> Self {
        let mut out = Self::new();
        for (v1, v2, cost) in edges {
            out.0
                .entry(v2.clone())
                .or_insert_with(|| Vertex::new(v2.clone(), Default::default(), vec![]));
            out.0
                .entry(v1.clone())
                .or_insert_with(|| Vertex::new(v1.clone(), Default::default(), vec![]))
                .add_edge(v2.clone(), cost);
        }
        out
    }
    pub fn new_from_bidirectional_edges(edges: Vec<(I, I, f32)>) -> Self {
        let mut out = Self::new();
        for (v1, v2, cost) in edges {
            out.0
                .entry(v2.clone())
                .or_insert_with(|| Vertex::new(v2.clone(), Default::default(), vec![]))
                .add_edge(v1.clone(), cost);
            out.0
                .entry(v1.clone())
                .or_insert_with(|| Vertex::new(v1.clone(), Default::default(), vec![]))
                .add_edge(v2.clone(), cost);
        }
        out
    }
}

impl<I, D> Display for Graph<I, D>
where
    I: Display + Clone + Eq + Hash,
    D: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in self.vertices() {
            writeln!(f, "{}", v)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct D(i32, i32);
    impl Display for D {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({},{})", self.0, self.1)
        }
    }

    fn make_graph() -> Graph<&'static str, i32> {
        Graph::new_from_edges(vec![
            ("DD", "EE", 1.0),
            ("BB", "DD", 5.0),
            ("AA", "BB", 1.0),
            ("AA", "CC", 2.0),
            ("CC", "DD", 1.0),
        ])
    }

    #[test]
    fn test_graph_new() {
        let mut g: Graph<&'static str, i32> = Graph::new();
        g.add_vertex(Vertex::new("AA", 0, vec![("BB", 1.0), ("CC", 2.0)]));
        g.add_vertex(Vertex::new("BB", 0, vec![("DD", 5.0)]));
        g.add_vertex(Vertex::new("CC", 0, vec![("DD", 1.0)]));
        g.add_vertex(Vertex::new("DD", 0, vec![("EE", 1.0)]));
        g.add_vertex(Vertex::new("EE", 0, vec![]));
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
                "AA <0> -> [BB](1),[CC](2)",
                "BB <0> -> [DD](5)",
                "CC <0> -> [DD](1)",
                "DD <0> -> [EE](1)",
                "EE <0> -> "
            ]
        );
    }

    #[test]
    fn test_graph_from_edges_symmetric() {
        let mut g: Graph<String, i32> = Graph::new_from_bidirectional_edges(vec![
            (String::from("AA"), String::from("BB"), 1.0),
            (String::from("AA"), String::from("CC"), 2.0),
            (String::from("BB"), String::from("DD"), 3.0),
            (String::from("CC"), String::from("DD"), 4.0),
        ]);
        if let Some(v) = g.get_mut(&String::from("DD")) {
            v.data = 99;
        };
        let s = g.to_string();
        assert_eq!(
            {
                let mut l = s.lines().collect::<Vec<_>>();
                l.sort();
                l
            },
            vec![
                "AA <0> -> [BB](1),[CC](2)",
                "BB <0> -> [AA](1),[DD](3)",
                "CC <0> -> [AA](2),[DD](4)",
                "DD <99> -> [BB](3),[CC](4)",
            ]
        );
    }

    #[test]
    fn test_graph_get() {
        let g = make_graph();
        assert_eq!(
            g.get(&"AA"),
            Some(&Vertex::new("AA", 0, vec![("BB", 1.0), ("CC", 2.0)]))
        );
        assert_eq!(g.get(&"ZZ"), None);
    }

    #[test]
    fn test_graph_vertices() {
        let g = make_graph();
        assert_eq!(
            {
                let mut v = g.vertices().map(|v| v.key).collect::<Vec<_>>();
                v.sort();
                v
            },
            vec!["AA", "BB", "CC", "DD", "EE"]
        );
    }

    #[test]
    fn test_graph_add_vertex() {
        let mut g = make_graph();
        assert_eq!(g.get(&"ZZ"), None);
        g.add_vertex(Vertex::new("ZZ", 0, vec![("AA", 99.0)]));
        assert_eq!(
            {
                let mut v = g.vertices().map(|v| v.key).collect::<Vec<_>>();
                v.sort();
                v
            },
            vec!["AA", "BB", "CC", "DD", "EE", "ZZ"]
        );
        assert_eq!(
            g.get(&"ZZ"),
            Some(&Vertex::new("ZZ", 0, vec![("AA", 99.0)]))
        );
    }

    #[test]
    fn test_graph_get_mut() {
        let mut g = make_graph();
        if let Some(v) = g.get_mut(&"AA") {
            v.add_edge("EE", 10.0);
            v.data = 99;
        };
        assert_eq!(
            g.get(&"AA"),
            Some(&Vertex::new(
                "AA",
                99,
                vec![("BB", 1.0), ("CC", 2.0), ("EE", 10.0)]
            ))
        );
    }

    #[test]
    fn test_graph_get_mut2() {
        let mut g: Graph<&'static str, i32> = Graph::new();
        g.add_vertex(Vertex::new("AA", 0, vec![]));
        if let Some(v) = g.get_mut(&"AA") {
            v.data = 99;
        };
        assert_eq!(g.get(&"AA").map(|v| v.data), Some(99));
    }

    #[test]
    fn test_vertex_add_edge() {
        let mut g = make_graph();
        if let Some(v) = g.get_mut(&"AA") {
            v.add_edge("ZZ", 99.0);
        };
        assert_eq!(
            g.get(&"AA").map(|v| v.edges.iter().collect::<Vec<_>>()),
            Some(vec![&("BB", 1.0), &("CC", 2.0), &("ZZ", 99.0)])
        );
    }

    #[test]
    fn test_vertex_key() {
        let g = make_graph();
        assert_eq!(g.get(&"AA").map(|v| v.key), Some("AA"));
    }

    #[test]
    fn test_vertex_data() {
        let mut g: Graph<&'static str, D> = Graph::new();
        g.add_vertex(Vertex::new("AA", D(0, 1), vec![]));
        assert_eq!(g.get(&"AA").map(|v| &v.data), Some(&D(0, 1)));
    }

    #[test]
    fn test_vertex_data_mut() {
        let mut g: Graph<&'static str, D> = Graph::new();
        g.add_vertex(Vertex::new("AA", D(0, 1), vec![]));
        if let Some(v) = g.get_mut(&"AA") {
            v.data.0 = 99;
        };
        assert_eq!(g.get(&"AA").map(|v| &v.data), Some(&D(99, 1)));
    }

    #[test]
    fn test_vertex_edges() {
        let g = make_graph();
        assert_eq!(
            g.get(&"AA").map(|v| v.edges.iter().collect::<Vec<_>>()),
            Some(vec![&("BB", 1.0), &("CC", 2.0)])
        );
    }

    #[test]
    fn test_vertex_edges_mut() {
        let mut g = make_graph();
        if let Some(v) = g.get_mut(&"AA") {
            v.edges.push(("ZZ", 99.0));
        };
        assert_eq!(
            g.get(&"AA").map(|v| v.edges.iter().collect::<Vec<_>>()),
            Some(vec![&("BB", 1.0), &("CC", 2.0), &("ZZ", 99.0)])
        );
    }

    #[test]
    fn test_graph_refcell() {
        use std::cell::RefCell;
        let mut g: Graph<&str, RefCell<i32>> = Graph::new_from_edges(vec![("AA", "BB", 1.0)]);
        assert_eq!(*g.get(&"AA").unwrap().data.borrow(), 0);
        g.get_mut(&"AA").unwrap().data.replace(99);
        assert_eq!(*g.get(&"AA").unwrap().data.borrow(), 99);
    }
}
