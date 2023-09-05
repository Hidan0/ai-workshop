use anyhow::bail;
use anyhow::Result;
use graph::running_example::AIGraph;
use graph::running_example::State;
use log::debug;
use log::info;
use log::warn;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    env_logger::init();

    let g = AIGraph::running_example();
    match depth_first_search(&g) {
        Ok(path) => println!("Path found: {}", path.join(", ")),
        Err(e) => println!("Error: {}", e),
    }
}

#[allow(dead_code)]
fn depth_first_search(graph: &AIGraph) -> Result<Vec<&'static str>> {
    let mut seen = HashSet::<&str>::new();
    let mut search_stack = VecDeque::new();

    let mut path = Vec::new();

    info!("Starting depth first search");

    let start_node = match graph.get_start_node() {
        Some(node) => {
            info!("{} is the staring node", node);
            node
        }
        None => {
            warn!("No start node found");
            bail!("No start node found");
        }
    };

    info!("Adding {} to search stack", start_node);
    search_stack.push_back(start_node);

    let mut goal_found = false;
    while !search_stack.is_empty() && !goal_found {
        let expanded_node = search_stack.pop_front().unwrap();

        info!("Adding {} to seen set", expanded_node);
        seen.insert(expanded_node);

        path.push(expanded_node);

        info!("Checking if {} is the goal", expanded_node);
        if *(graph.get_vertex(expanded_node).unwrap()) == State::Goal {
            info!("{} is the goal!", expanded_node);
            goal_found = true;
            break;
        }

        info!("{} is not the goal", expanded_node);

        info!("Expanding {}", expanded_node);
        let mut nodes = graph.expand(expanded_node);

        debug!(
            "Removing from {:?} nodes that have already been seen...",
            nodes
        );
        nodes.retain(|&n| !seen.contains(n));
        debug!("Nodes that have not been seen are {:?}", nodes);

        if nodes.is_empty() {
            info!("Empty node list, backtracking...");
            path.pop();
            continue;
        }

        nodes.sort_by(|a, b| b.partial_cmp(a).unwrap()); // tie braking, sorting in reverse order
        info!("Adding {} to search stack", nodes.join(", "));
        nodes.iter().for_each(|n| search_stack.push_front(n)); // add nodes to the front of the queue
        debug!("Search stack is now {:?}", search_stack);
    }

    if goal_found {
        Ok(path)
    } else {
        bail!("No path found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_dfs() {
        let g = AIGraph::running_example();
        assert_eq!(
            depth_first_search(&g).unwrap().join(", "),
            "A, B, D, F, G, E"
        )
    }
}
