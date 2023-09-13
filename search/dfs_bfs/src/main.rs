use anyhow::{bail, Result};
use fnv::FnvHashMap;
use graph::ai::{AIGraph, AIGraphVId, Frontier};
use log::{debug, info, warn};
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

fn main() {
    env_logger::init();

    let g = AIGraph::running_example();

    match dfs(&g) {
        Ok(goal_node) => println!(
            "[DFS] A path to the goal has been found! The goal node is {}.",
            goal_node
        ),
        Err(e) => println!("Error: {}", e),
    }

    match bfs(&g) {
        Ok(goal_node) => println!(
            "[BFS] A path to the goal has been found! The goal node is {}.",
            goal_node
        ),
        Err(e) => println!("Error: {}", e),
    }
}

fn build_path(tree: &FnvHashMap<AIGraphVId, AIGraphVId>, from: AIGraphVId) -> String {
    let mut path: Vec<AIGraphVId> = vec![];

    path.push(from);
    let mut current = from;

    while let Some(next) = tree.get(&current) {
        path.push(*next);
        current = *next
    }

    path.join(" -> ")
}

fn build_mermaid_graph(tree: &FnvHashMap<AIGraphVId, AIGraphVId>) -> String {
    let mut id = 0;
    let mut mermaid_id = FnvHashMap::<AIGraphVId, usize>::default();

    let mut graph = String::new();

    for (node, parent) in tree {
        if let Some(parent_id) = mermaid_id.get(parent) {
            graph.push_str(format!("{} --> ", *parent_id).as_str());
        } else {
            id += 1;
            mermaid_id.insert(*parent, id);
            graph.push_str(format!("{}(({})) --> ", id, parent).as_str());
        }

        if let Some(node_id) = mermaid_id.get(node) {
            graph.push_str(format!("{};", *node_id).as_str());
        } else {
            id += 1;
            mermaid_id.insert(*node, id);
            graph.push_str(format!("{}(({}));", id, node).as_str());
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

/// Performs a Depth-First Search (DFS) with backtracking and pruning on the graph.
///
/// # Errors
///
/// Returns an error if either no initial node is found or no path is found.
fn dfs(graph: &AIGraph) -> Result<AIGraphVId> {
    let mut eql = HashSet::<AIGraphVId>::new(); // EQL
    let mut frontier = DfsFrontier::default();
    let mut child_of = FnvHashMap::<AIGraphVId, AIGraphVId>::default();

    info!("Starting DFS...");

    let start_node = match graph.get_start_node() {
        Some(node) => {
            info!("{} is the staring node.", node);
            node
        }
        None => {
            warn!("No start node found.");
            bail!("No start node found");
        }
    };

    info!("Initializing the frontier with {}.", start_node);
    frontier.push(start_node);

    eql.insert(start_node);

    while let Some(node) = frontier.pop() {
        info!("Checking if {} is the goal.", node);
        if graph.is_goal(node) {
            info!("{} is the goal!", node);

            debug!("Final EQl is now {:?}", eql);
            debug!("Final Frontier is now {:?}", frontier);

            println!("{}", build_path(&child_of, node));
            println!("{}", build_mermaid_graph(&child_of));

            return Ok(node);
        }

        info!("Expanding {}.", node);
        let mut expanded_nodes = graph.expand(node);

        debug!("Expanded nodes are {:?}.", expanded_nodes);
        info!("Checking if any of the expanded nodes are in the EQL.");
        expanded_nodes.retain(|node| !eql.contains(node));
        debug!("Expanded nodes after pruning are {:?}.", expanded_nodes);

        if expanded_nodes.is_empty() {
            info!("No new nodes found from {}. Backtracking.", node);
            continue;
        }

        expanded_nodes.sort();
        while let Some(e_node) = expanded_nodes.pop() {
            info!("Adding {} to EQL and frontier.", e_node);
            eql.insert(e_node);

            child_of.insert(e_node, node);

            frontier.push(e_node);
        }

        debug!("EQl is now {:?}", eql);
        debug!("Frontier is now {:?}", frontier);
    }
    bail!("No path found")
}

/// Performs a Breadth-First Search (BFS) with backtracking and pruning on the graph.
///
/// # Errors
///
/// Returns an error if either no initial node is found or no path is found.
fn bfs(graph: &AIGraph) -> Result<AIGraphVId> {
    let mut eql = HashSet::<AIGraphVId>::new(); // EQL
    let mut frontier = BfsFrontier::default();
    let mut child_of = FnvHashMap::<AIGraphVId, AIGraphVId>::default();

    info!("Starting BFS...");

    let start_node = match graph.get_start_node() {
        Some(node) => {
            info!("{} is the staring node.", node);
            node
        }
        None => {
            warn!("No start node found.");
            bail!("No start node found");
        }
    };

    info!("Initializing the frontier with {}.", start_node);
    frontier.push(start_node);

    eql.insert(start_node);

    while let Some(node) = frontier.pop() {
        info!("Checking if {} is the goal.", node);
        if graph.is_goal(node) {
            info!("{} is the goal!", node);

            debug!("Final EQl is now {:?}", eql);
            debug!("Final Frontier is now {:?}", frontier);

            println!("{}", build_path(&child_of, node));
            println!("{}", build_mermaid_graph(&child_of));

            return Ok(node);
        }

        info!("Expanding {}.", node);
        let mut expanded_nodes = graph.expand(node);

        debug!("Expanded nodes are {:?}.", expanded_nodes);
        info!("Checking if any of the expanded nodes are in the EQL.");
        expanded_nodes.retain(|node| !eql.contains(node));
        debug!("Expanded nodes after pruning are {:?}.", expanded_nodes);

        if expanded_nodes.is_empty() {
            info!("No new nodes found from {}. Backtracking.", node);
            continue;
        }

        expanded_nodes.sort();
        for e_node in expanded_nodes {
            info!("Adding {} to EQL and frontier.", e_node);
            eql.insert(e_node);

            child_of.insert(e_node, node);

            frontier.push(e_node);
        }

        debug!("EQl is now {:?}", eql);
        debug!("Frontier is now {:?}", frontier);
    }
    bail!("No path found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_dfs_no_start() {
        let g = AIGraph::new();
        assert_eq!(dfs(&g).unwrap_err().to_string(), "No start node found")
    }

    #[test]
    fn check_dfs_only_start() {
        let mut g = AIGraph::new();
        g.push_vertex("A", graph::ai::State::Start);
        assert_eq!(dfs(&g).unwrap_err().to_string(), "No path found")
    }

    #[test]
    fn check_dfs() {
        let g = AIGraph::running_example();
        assert_eq!(dfs(&g).unwrap(), "E")
    }

    #[test]
    fn check_bfs() {
        let g = AIGraph::running_example();
        assert_eq!(bfs(&g).unwrap(), "E")
    }
}
