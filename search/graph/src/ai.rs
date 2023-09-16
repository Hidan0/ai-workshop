use crate::Graph;

pub trait Solution<T> {
    fn build_path(&self, from: T) -> String;
    fn build_mermaid_graph(&self) -> String;
}

#[derive(Hash, PartialEq, Eq)]
pub enum State {
    Start,
    Neutral,
    Goal,
}

impl Default for State {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Clone)]
pub struct Edge(i32);

pub type AIGraphVId = &'static str;
pub type AIGraph = Graph<AIGraphVId, Edge, State>;

impl AIGraph {
    pub fn get_start_node(&self) -> Option<AIGraphVId> {
        if let Some((vid, _)) = self.vertices.iter().find(|(_, v)| **v == State::Start) {
            return Some(*vid);
        }

        None
    }

    pub fn is_goal(&self, vid: AIGraphVId) -> bool {
        self.vertices.get(vid) == Some(&State::Goal)
    }

    pub fn running_example() -> AIGraph {
        let mut g = Graph::new();

        g.push_vertex("A", State::Start);
        g.push_vertex("E", State::Goal);

        g.push_undirected_edge("A", "B", Edge(5));
        g.push_undirected_edge("A", "F", Edge(6));

        g.push_undirected_edge("B", "D", Edge(3));
        g.push_undirected_edge("B", "C", Edge(7));

        g.push_undirected_edge("D", "F", Edge(3));
        g.push_undirected_edge("D", "G", Edge(4));

        g.push_undirected_edge("F", "G", Edge(5));

        g.push_undirected_edge("G", "E", Edge(3));

        g
    }

    pub fn get_cost_of(&self, from_vid: AIGraphVId, to_vid: AIGraphVId) -> Option<i32> {
        if let Some(adj) = self.adjacency.get(from_vid) {
            if let Some((_, value)) = adj.iter().find(|(vid, _)| *vid == to_vid) {
                return Some(value.0);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_graph() {
        let g = AIGraph::running_example();

        assert_eq!(g.expand("A"), ["B", "F"]);
        assert_eq!(g.expand("B"), ["A", "D", "C"]);
        assert_eq!(g.expand("C"), ["B"]);
        assert_eq!(g.expand("D"), ["B", "F", "G"]);
        assert_eq!(g.expand("E"), ["G"]);
        assert_eq!(g.expand("F"), ["A", "D", "G"]);
    }

    #[test]
    fn check_vertices() {
        let g = AIGraph::running_example();

        assert!(matches!(g.get_vertex("A").unwrap(), State::Start));
        assert!(matches!(g.get_vertex("E").unwrap(), State::Goal));
        assert!(matches!(g.get_vertex("D").unwrap(), State::Neutral));
    }

    #[test]
    fn get_start_node() {
        let g = AIGraph::running_example();
        assert_eq!(g.get_start_node(), Some("A"));
    }

    #[test]
    fn get_cost() {
        let g = AIGraph::running_example();
        assert_eq!(g.get_cost_of("A", "B"), Some(5));
    }

    #[test]
    fn dont_get_cost() {
        let g = AIGraph::running_example();
        assert_eq!(g.get_cost_of("A", "Z"), None);
    }
}
