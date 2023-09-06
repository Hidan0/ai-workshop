use anyhow::bail;
use anyhow::Result;
use graph::running_example::AIGraph;
use graph::running_example::AIGraphVId;
use log::debug;
use log::info;
use log::warn;
use std::collections::HashSet;
use std::collections::VecDeque;

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

/// Performs a Depth-First Search (BFS) with backtracking and pruning on the graph.
///
/// # Errors
///
/// Returns an error if either no initial node is found or no path is found.
fn dfs(graph: &AIGraph) -> Result<AIGraphVId> {
    let mut eql = HashSet::<AIGraphVId>::new(); // EQL
    let mut frontier = VecDeque::<AIGraphVId>::new();

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
    frontier.push_back(start_node);

    eql.insert(start_node);

    while let Some(node) = frontier.pop_front() {
        info!("Checking if {} is the goal.", node);
        if graph.is_goal(node) {
            info!("{} is the goal!", node);

            debug!("Final EQl is now {:?}", eql);
            debug!("Final Frontier is now {:?}", frontier);

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
            frontier.push_front(e_node);
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
    let mut frontier = VecDeque::<AIGraphVId>::new();

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
    frontier.push_back(start_node);

    eql.insert(start_node);

    while let Some(node) = frontier.pop_front() {
        info!("Checking if {} is the goal.", node);
        if graph.is_goal(node) {
            info!("{} is the goal!", node);

            debug!("Final EQl is now {:?}", eql);
            debug!("Final Frontier is now {:?}", frontier);

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
            frontier.push_back(e_node);
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
        g.push_vertex("A", graph::running_example::State::Start);
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
