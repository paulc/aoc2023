use crate::graph::Graph;
use crate::graph::Vertex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

impl<I, D> Graph<I, D>
where
    I: Clone + Eq + Hash,
{
    pub fn bfs<F>(&self, root: &I, f: &mut F) -> Option<Vec<I>>
    where
        F: FnMut(&Vertex<I, D>) -> bool,
    {
        let mut q: VecDeque<I> = VecDeque::new();
        let mut explored: HashSet<I> = HashSet::from([root.clone()]);
        let mut parent: HashMap<I, I> = HashMap::new();
        q.push_back(root.clone());
        while let Some(i) = q.pop_front() {
            if let Some(v) = self.get(&i) {
                if f(v) {
                    let mut path: Vec<I> = vec![i.clone()];
                    let mut current = &i;
                    while let Some(prev) = parent.get(current) {
                        path.push(prev.clone());
                        current = prev;
                    }
                    return Some(path);
                }
                for (e, _) in &v.edges {
                    if !explored.contains(&e) {
                        explored.insert(e.clone());
                        parent.insert(e.clone(), i.clone());
                        q.push_back(e.clone());
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
    fn test_bfs() {
        let g = make_graph();
        let mut visited: Vec<&str> = Vec::new();
        g.bfs(&"A", &mut |v: &Vertex<&str, &str>| {
            visited.push(v.key);
            false
        });
        assert_eq!(visited, vec!["A", "B", "C", "E", "D", "F", "G"]);
    }

    #[test]
    fn test_bfs_goal() {
        let g = make_graph();
        let goal = "F";
        let mut visited: Vec<&str> = Vec::new();
        let path = g.bfs(&"A", &mut |v: &Vertex<&str, &str>| {
            visited.push(v.key);
            v.key == goal
        });
        assert_eq!(visited, vec!["A", "B", "C", "E", "D", "F"]);
        assert_eq!(path, Some(vec!["F", "B", "A"]));
    }
}
