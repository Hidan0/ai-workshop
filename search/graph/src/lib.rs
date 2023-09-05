use std::collections::HashMap;
use std::hash::Hash;

pub mod running_example;

pub struct Graph<VId, E = (), V = ()> {
    vertices: HashMap<VId, V>,
    adjacency: HashMap<VId, Vec<(VId, E)>>,
}

impl<VId, E, V> Graph<VId, E, V>
where
    VId: Eq + Hash + Clone,
    V: Hash,
{
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn push_vertex(&mut self, vid: VId, vertex: V) {
        self.vertices.insert(vid, vertex);
    }

    pub fn push_edge(&mut self, from: VId, to: VId, edge: E) {
        let adjacency_to_from = self.adjacency.entry(from).or_default();
        adjacency_to_from.push((to, edge));
    }

    pub fn expand(&self, vid: VId) -> Vec<VId> {
        if let Some(vertex) = self.adjacency.get(&vid) {
            vertex.iter().map(|(v, _)| (*v).clone()).collect()
        } else {
            Vec::<VId>::default()
        }
    }
}

impl<VId, E, V> Default for Graph<VId, E, V>
where
    VId: Eq + Hash + Clone,
    V: Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<VId, E> Graph<VId, E, ()>
where
    VId: Eq + Hash,
{
    pub fn push_vid(&mut self, vid: VId) {
        self.vertices.insert(vid, ());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_empty() {
        let g: Graph<&str, ()> = Graph::new();
        assert_eq!(g.expand("A"), Vec::<&str>::default());
    }

    #[test]
    fn expand() {
        let mut g = Graph::new();
        g.push_vid("A");
        g.push_vid("B");
        g.push_vid("F");
        g.push_edge("A", "B", ());
        g.push_edge("A", "F", ());

        assert_eq!(g.expand("A"), ["B", "F"]);
        assert_eq!(g.expand("B"), Vec::<&str>::default());
    }

    #[test]
    fn create_v_when_edge_is_pushed() {
        let mut g = Graph::new();
        g.push_vid("A");
        g.push_edge("A", "B", ());
        g.push_edge("A", "F", ());

        assert_eq!(g.expand("A"), ["B", "F"]);
        assert_eq!(g.expand("B"), Vec::<&str>::default());
    }
}
