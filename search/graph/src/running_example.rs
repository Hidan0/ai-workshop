use crate::Graph;

#[derive(Hash)]
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

pub fn running_example() -> Graph<&'static str, Edge, State> {
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

#[cfg(test)]
mod tests {
    use super::running_example;

    #[test]
    fn check_graph() {
        let g = running_example();

        assert_eq!(g.expand("A"), ["B", "F"]);
        assert_eq!(g.expand("B"), ["A", "D", "C"]);
        assert_eq!(g.expand("C"), ["B"]);
        assert_eq!(g.expand("D"), ["B", "F", "G"]);
        assert_eq!(g.expand("E"), ["G"]);
        assert_eq!(g.expand("F"), ["A", "D", "G"]);
    }
}
