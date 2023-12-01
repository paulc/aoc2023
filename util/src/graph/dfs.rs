use crate::graph::Graph;
use crate::graph::Vertex;

use std::collections::HashSet;
use std::hash::Hash;

pub struct DfsIter<'a, I, D>
where
    I: Clone + Eq + Hash,
{
    graph: &'a Graph<I, D>,
    discovered: HashSet<I>,
    stack: Vec<I>,
}

impl<'a, I, D> Iterator for DfsIter<'a, I, D>
where
    I: Clone + Eq + Hash,
{
    type Item = &'a Vertex<I, D>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(i) = self.stack.pop() {
            if !self.discovered.contains(&i) {
                self.discovered.insert(i.clone());
                if let Some(v) = self.graph.get(&i) {
                    for (e, _) in &v.edges {
                        self.stack.push(e.clone());
                    }
                };
                return self.graph.get(&i);
            }
        }
        None
    }
}

impl<I, D> Graph<I, D>
where
    I: Clone + Eq + Hash,
{
    pub fn dfs_iter(&self, root: &I) -> DfsIter<I, D> {
        DfsIter {
            graph: &self,
            discovered: HashSet::new(),
            stack: vec![root.clone()],
        }
    }

    pub fn dfs<F>(&self, root: &I, f: &mut F)
    where
        F: FnMut(&Vertex<I, D>),
    {
        let mut discovered: HashSet<I> = HashSet::new();
        Self::dfs_r(&self, &mut discovered, root, f);
    }

    fn dfs_r<F>(graph: &Graph<I, D>, discovered: &mut HashSet<I>, i: &I, f: &mut F)
    where
        F: FnMut(&Vertex<I, D>),
    {
        discovered.insert(i.clone());
        if let Some(v) = graph.get(&i) {
            f(v);
            for (e, _) in &v.edges {
                if !discovered.contains(e) {
                    Self::dfs_r(graph, discovered, e, f)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::*;

    fn make_graph<'a>() -> Graph<&'a str, &'a str> {
        Graph::new_from_bidirectional_edges(vec![
            ("A", "B", 1.0),
            ("A", "C", 1.0),
            ("A", "E", 1.0),
            ("B", "D", 1.0),
            ("B", "F", 1.0),
            ("C", "G", 1.0),
            ("E", "F", 1.0),
        ])
    }

    #[test]
    fn test_dfs() {
        let g = make_graph();
        let mut out: Vec<String> = vec![];
        let mut f = |v: &Vertex<&str, &str>| out.push(v.key.to_string());
        g.dfs(&"A", &mut f);
        assert_eq!(
            out,
            vec!["A", "B", "D", "F", "E", "C", "G"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_dfs_iter() {
        let g = make_graph();
        assert_eq!(
            g.dfs_iter(&"A").map(|v| v.key).collect::<Vec<_>>(),
            vec!["A", "E", "F", "B", "D", "C", "G"]
        );
    }
}
