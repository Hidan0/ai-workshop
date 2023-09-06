use anyhow::bail;
use anyhow::Result;
use graph::running_example::AIGraph;
use graph::running_example::AIGraphVId;
use log::debug;
use log::info;
use log::warn;
use std::collections::VecDeque;

fn main() {
    env_logger::init();

    let g = AIGraph::running_example();
    match dfs(&g) {
        Ok(path) => println!("Path found: {}", path.join(", ")),
        Err(e) => println!("Error: {}", e),
    }
}

fn dfs(graph: &AIGraph) -> Result<Vec<AIGraphVId>> {
    let mut eql = Vec::<AIGraphVId>::new(); // EQL
    let mut frontier = VecDeque::<AIGraphVId>::new();

    let mut path = Vec::<AIGraphVId>::new();

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

    eql.push(start_node);

    while let Some(node) = frontier.pop_front() {
        path.push(node);

        info!("Checking if {} is the goal.", node);
        if graph.is_goal(node) {
            info!("{} is the goal!", node);

            debug!("Final EQl is now {:?}", eql);
            debug!("Final Frontier is now {:?}", frontier);

            return Ok(path);
        }

        info!("Expanding {}.", node);
        let mut expanded_nodes = graph.expand(node);

        debug!("Expanded nodes are {:?}.", expanded_nodes);
        info!("Checking if any of the expanded nodes are in the EQL.");
        expanded_nodes.retain(|node| !eql.contains(node));
        debug!("Expanded nodes after pruning are {:?}.", expanded_nodes);

        if expanded_nodes.is_empty() {
            info!("No new nodes found from {}. Backtracking.", node);
            path.pop();
            continue;
        }

        expanded_nodes.sort_by(|a, b| b.cmp(a));
        for e_node in expanded_nodes {
            info!("Adding {} to EQL and frontier.", e_node);
            eql.push(e_node);
            frontier.push_front(e_node);
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
    fn check_dfs() {
        let g = AIGraph::running_example();
        assert_eq!(dfs(&g).unwrap().join(", "), "A, B, D, G, E")
    }
}
