use anyhow::{bail, Result};
use fnv::FnvHashMap;
use graph::ai::{AIGraph, AIGraphVId, Frontier};
use std::collections::{HashSet, VecDeque};

#[derive(Default, Debug)]
struct DfsFrontier {
    f: VecDeque<AIGraphVId>,
}

impl Frontier for DfsFrontier {
    fn is_empty(&self) -> bool {
        self.f.is_empty()
    }

    fn pop(&mut self) -> Option<AIGraphVId> {
        self.f.pop_back()
    }

    fn push(&mut self, node: AIGraphVId) {
        self.f.push_back(node);
    }
}

#[derive(Default, Debug)]
struct BfsFrontier {
    f: VecDeque<AIGraphVId>,
}

impl Frontier for BfsFrontier {
    fn is_empty(&self) -> bool {
        self.f.is_empty()
    }

    fn pop(&mut self) -> Option<AIGraphVId> {
        self.f.pop_front()
    }

    fn push(&mut self, node: AIGraphVId) {
        self.f.push_back(node);
    }
}

pub enum Algorithm {
    Dfs,
    Bfs,
}

pub struct Solver<'a> {
    graph: &'a AIGraph,
    eql: HashSet<AIGraphVId>,
    frontier: Box<dyn Frontier>,
    search_tree: FnvHashMap<AIGraphVId, AIGraphVId>,
}

impl<'a> Solver<'a> {
    pub fn new(graph: &'a AIGraph, algorithm: Algorithm) -> Self {
        let frontier: Box<dyn Frontier> = match algorithm {
            Algorithm::Dfs => Box::<DfsFrontier>::default(),
            Algorithm::Bfs => Box::<BfsFrontier>::default(),
        };

        Self {
            graph,
            eql: HashSet::new(),
            frontier,
            search_tree: FnvHashMap::default(),
        }
    }

    fn build_path(&self, from: AIGraphVId) -> String {
        let mut path: Vec<AIGraphVId> = vec![];

        path.push(from);
        let mut current = from;

        while let Some(next) = self.search_tree.get(&current) {
            path.push(*next);
            current = *next
        }

        path.reverse();
        path.join(" -> ")
    }

    pub fn build_mermaid_graph(&self) -> String {
        let mut id = 0;
        let mut mermaid_id = FnvHashMap::<AIGraphVId, usize>::default();

        let mut graph = String::new();

        for (node, parent) in &self.search_tree {
            if let Some(parent_id) = mermaid_id.get(parent) {
                graph.push_str(format!("    {} --> ", *parent_id).as_str());
            } else {
                id += 1;
                mermaid_id.insert(parent, id);
                graph.push_str(format!("    {}(({})) --> ", id, parent).as_str());
            }

            if let Some(node_id) = mermaid_id.get(node) {
                graph.push_str(format!("{};\n", *node_id).as_str());
            } else {
                id += 1;
                mermaid_id.insert(node, id);
                graph.push_str(format!("{}(({}));\n", id, node).as_str());
            }
        }

        format!(
            r#"
graph TD;
{}
"#,
            graph
        )
    }

    pub fn solve(&mut self) -> Result<String> {
        let start_node = match self.graph.get_start_node() {
            Some(node) => node,
            None => {
                bail!("No start node found");
            }
        };

        self.frontier.push(start_node);
        self.eql.insert(start_node);

        while let Some(node) = self.frontier.pop() {
            if self.graph.is_goal(node) {
                return Ok(self.build_path(node));
            }

            let mut expanded_nodes = self.graph.expand(node);
            expanded_nodes.retain(|node| !self.eql.contains(node));

            if expanded_nodes.is_empty() {
                continue;
            }

            expanded_nodes.sort();
            while let Some(e_node) = expanded_nodes.pop() {
                self.eql.insert(e_node);

                self.search_tree.insert(e_node, node);

                self.frontier.push(e_node);
            }
        }
        bail!("No goal found")
    }
}

fn main() {
    let g = AIGraph::running_example();

    let mut dfs_solver = Solver::new(&g, Algorithm::Dfs);
    let mut bfs_solver = Solver::new(&g, Algorithm::Bfs);

    match dfs_solver.solve() {
        Ok(solution) => println!("[DFS] A path to the goal has been found! {}.", solution),
        Err(e) => println!("Error: {}", e),
    }
    match bfs_solver.solve() {
        Ok(solution) => println!("[BFS] A path to the goal has been found! {}.", solution),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph::ai::State;

    #[test]
    fn check_no_start() {
        let g = AIGraph::new();
        let mut solver = Solver::new(&g, Algorithm::Dfs);
        assert_eq!(
            solver.solve().unwrap_err().to_string(),
            "No start node found"
        )
    }

    #[test]
    fn check_only_start() {
        let mut g = AIGraph::new();
        g.push_vertex("A", State::Start);

        let mut solver = Solver::new(&g, Algorithm::Dfs);
        assert_eq!(solver.solve().unwrap_err().to_string(), "No goal found")
    }

    #[test]
    fn check_dfs() {
        let g = AIGraph::running_example();
        let mut solver = Solver::new(&g, Algorithm::Dfs);

        assert_eq!(solver.solve().unwrap(), "A -> B -> D -> G -> E");
    }

    #[test]
    fn check_bfs() {
        let g = AIGraph::running_example();
        let mut solver = Solver::new(&g, Algorithm::Bfs);

        assert_eq!(solver.solve().unwrap(), "A -> F -> G -> E");
    }
}
