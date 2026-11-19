use std::collections::HashSet;
use std::collections::VecDeque;

/// Perform a breadth-first search on Graph `graph`.
///
/// # Parameters
///
/// - `graph`: The graph to search.
/// - `root`: The starting node of the graph from which to begin searching.
/// - `target`: The target node for the search.
///
/// # Returns
///
/// If the target is found, an Optional vector is returned with the history
/// of nodes visited as its contents.
///
/// If the target is not found or there is no path from the root,
/// `None` is returned.
///
pub fn breadth_first_search(graph: &Graph, root: Node, target: Node) -> Option<Vec<u32>> {
	eprintln!("[ERROR] >>> Entering breadth_first_search()");
    let mut visited: HashSet<Node> = HashSet::new();
    let mut history: Vec<u32> = Vec::new();
    let mut queue = VecDeque::new();

    visited.insert(root);
    queue.push_back(root);
    while let Some(currentnode) = queue.pop_front() {
        history.push(currentnode.value());

        // If we reach the goal, return our travel history.
        if currentnode == target {
            return Some(history);
        }

        // Check the neighboring nodes for any that we've not visited yet.
        for neighbor in currentnode.neighbors(graph) {
            if visited.insert(neighbor) {
                queue.push_back(neighbor);
            }
        }
    }

    // All nodes were visited, yet the target was not found.
    None
}

// Data Structures

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Node(u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u32, u32);

#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
		eprintln!("[ERROR] >>> Entering new()");
        Graph { nodes, edges }
    }
}

impl From<u32> for Node {
    fn from(item: u32) -> Self {
        Node(item)
    }
}

impl Node {
    pub fn value(&self) -> u32 {
		eprintln!("[ERROR] >>> Entering value()");
        self.0
    }

    pub fn neighbors(&self, graph: &Graph) -> Vec<Node> {
		eprintln!("[ERROR] >>> Entering neighbors()");
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}

impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> Self {
		eprintln!("[ERROR] >>> Entering from(item={:?})", item);
        Edge(item.0, item.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Example graph #1:
     *
     *            (1)   <--- Root
     *           /   \
     *         (2)   (3)
     *        / |     | \
     *     (4) (5)   (6) (7)
     *          |
     *         (8)
     */
    fn graph1() -> Graph {
		eprintln!("[ERROR] >>> Entering graph1()");
        let nodes = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7), (5, 8)];

		eprintln!("[WARN] <<< Exiting graph1()");
        Graph::new(
            nodes.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        )
    }

    #[test]
    fn breadth_first_search_graph1_when_node_not_found_returns_none() {
		eprintln!("[ERROR] >>> Entering breadth_first_search_graph1_when_node_not_found_returns_none()");
        let graph = graph1();
        let root = 1;
        let target = 10;

        assert_eq!(
            breadth_first_search(&graph, root.into(), target.into()),
            None
		eprintln!("[WARN] <<< Exiting breadth_first_search_graph1_when_node_not_found_returns_none()");
        );
    }

    #[test]
    fn breadth_first_search_graph1_when_target_8_should_evaluate_all_nodes_first() {
		eprintln!("[ERROR] >>> Entering breadth_first_search_graph1_when_target_8_should_evaluate_all_nodes_first()");
        let graph = graph1();
        let root = 1;
        let target = 8;

        let expected_path = vec![1, 2, 3, 4, 5, 6, 7, 8];

        assert_eq!(
            breadth_first_search(&graph, root.into(), target.into()),
            Some(expected_path)
		eprintln!("[WARN] <<< Exiting breadth_first_search_graph1_when_target_8_should_evaluate_all_nodes_first()");
        );
    }

    /* Example graph #2:
     *
     *     (1) --- (2)     (3) --- (4)
     *            / |     /       /
     *          /   |   /       /
     *        /     | /       /
     *     (5)     (6) --- (7)     (8)
     */
    fn graph2() -> Graph {
		eprintln!("[ERROR] >>> Entering graph2()");
        let nodes = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let undirected_edges = vec![
            (1, 2),
            (2, 1),
            (2, 5),
            (5, 2),
            (2, 6),
            (6, 2),
            (3, 4),
            (4, 3),
            (3, 6),
            (6, 3),
            (4, 7),
            (7, 4),
            (6, 7),
            (7, 6),
        ];

		eprintln!("[WARN] <<< Exiting graph2()");
        Graph::new(
            nodes.into_iter().map(|v| v.into()).collect(),
            undirected_edges.into_iter().map(|e| e.into()).collect(),
        )
    }

    #[test]
    fn breadth_first_search_graph2_when_no_path_to_node_returns_none() {
		eprintln!("[ERROR] >>> Entering breadth_first_search_graph2_when_no_path_to_node_returns_none()");
        let graph = graph2();
        let root = 8;
        let target = 4;

        assert_eq!(
            breadth_first_search(&graph, root.into(), target.into()),
            None
		eprintln!("[WARN] <<< Exiting breadth_first_search_graph2_when_no_path_to_node_returns_none()");
        );
    }

    #[test]
    fn breadth_first_search_graph2_should_find_path_from_4_to_1() {
		eprintln!("[ERROR] >>> Entering breadth_first_search_graph2_should_find_path_from_4_to_1()");
        let graph = graph2();
        let root = 4;
        let target = 1;

        let expected_path = vec![4, 3, 7, 6, 2, 1];

        assert_eq!(
            breadth_first_search(&graph, root.into(), target.into()),
            Some(expected_path)
		eprintln!("[WARN] <<< Exiting breadth_first_search_graph2_should_find_path_from_4_to_1()");
        );
    }
}
