use anyhow::{bail, Result};
use fnv::{FnvHashMap, FnvHashSet};
use graph::ai::{AIGraph, AIGraphVId};

#[derive(Default)]
struct Frontier {
    f: Vec<(i32, AIGraphVId)>,
}

impl Frontier {
    fn is_empty(&self) -> bool {
        self.f.is_empty()
    }

    fn pop(&mut self) -> Option<(i32, AIGraphVId)> {
        if self.is_empty() {
            return None;
        }

        self.f.pop()
    }

    fn push(&mut self, cost: i32, node: AIGraphVId) {
        self.f.push((cost, node));
        self.f.sort_by(|(c1, _), (c2, _)| c2.cmp(c1));
    }
}

pub type SearchTree = FnvHashMap<(i32, AIGraphVId), (i32, AIGraphVId)>;

pub trait ST {
    fn build_path(&self, from: (i32, AIGraphVId)) -> String;
    fn build_mermaid_graph(&self) -> String;
}

impl ST for SearchTree {
    fn build_path(&self, from: (i32, AIGraphVId)) -> String {
        let mut path: Vec<AIGraphVId> = vec![];

        path.push(from.1);
        let mut current = from;

        while let Some(next) = self.get(&current) {
            path.push(next.1);
            current = *next
        }

        path.reverse();
        path.join(" -> ")
    }

    fn build_mermaid_graph(&self) -> String {
        let mut id = 0;
        let mut mermaid_id = FnvHashMap::<(i32, AIGraphVId), usize>::default();

        let mut graph = String::new();

        for (node, parent) in self {
            if let Some(parent_id) = mermaid_id.get(parent) {
                graph.push_str(format!("    {} --> ", *parent_id).as_str());
            } else {
                id += 1;
                mermaid_id.insert(*parent, id);
                graph
                    .push_str(format!("    {}((\"{} {}\")) --> ", id, parent.0, parent.1).as_str());
            }

            if let Some(node_id) = mermaid_id.get(node) {
                graph.push_str(format!("{};\n", *node_id).as_str());
            } else {
                id += 1;
                mermaid_id.insert(*node, id);
                graph.push_str(format!("{}((\"{} {}\"));\n", id, node.0, node.1).as_str());
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
}

pub struct UcsSolver<'a> {
    graph: &'a AIGraph,
    frontier: Frontier,
    exl: FnvHashSet<AIGraphVId>,
    search_tree: SearchTree,
}

impl<'a> UcsSolver<'a> {
    pub fn new(graph: &'a AIGraph) -> Self {
        UcsSolver {
            graph,
            frontier: Frontier::default(),
            exl: FnvHashSet::default(),
            search_tree: SearchTree::default(),
        }
    }

    pub fn solve(&mut self) -> Result<String> {
        let start_node = match self.graph.get_start_node() {
            Some(node) => node,
            None => {
                bail!("No start node found");
            }
        };

        self.frontier.push(0, start_node);

        while let Some((cost, node)) = self.frontier.pop() {
            if self.exl.contains(node) {
                continue;
            }

            if self.graph.is_goal(node) {
                println!("{}", self.search_tree.build_mermaid_graph());
                return Ok(self.search_tree.build_path((cost, node)));
            }

            self.exl.insert(node);

            let mut expanded_nodes = self.graph.expand(node);
            expanded_nodes.retain(|node| !self.exl.contains(node));

            for e_node in expanded_nodes {
                let child_cost = self.graph.get_cost_of(node, e_node).unwrap();

                let g = cost + child_cost;

                self.search_tree.insert((g, e_node), (cost, node));
                self.frontier.push(g, e_node);
            }
        }
        bail!("No goal found")
    }
}

fn main() {
    let g = AIGraph::running_example();

    let mut solver = UcsSolver::new(&g);

    match solver.solve() {
        Ok(solution) => println!("[UCS] A path to the goal has been found! {}.", solution),
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
        let mut solver = UcsSolver::new(&g);
        assert_eq!(
            solver.solve().unwrap_err().to_string(),
            "No start node found"
        )
    }

    #[test]
    fn check_only_start() {
        let mut g = AIGraph::new();
        g.push_vertex("A", State::Start);

        let mut solver = UcsSolver::new(&g);
        assert_eq!(solver.solve().unwrap_err().to_string(), "No goal found")
    }

    #[test]
    fn check_ucs() {
        let g = AIGraph::running_example();
        let mut solver = UcsSolver::new(&g);

        assert_eq!(solver.solve().unwrap(), "A -> F -> G -> E");
    }
}
