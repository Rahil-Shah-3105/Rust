use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		log::debug!(">>> Entering fmt(f=(unprintable))");
        log::debug!("<<< Exiting fmt(f=(unprintable))");
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct DirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for DirectedGraph {
    fn new() -> DirectedGraph {
        DirectedGraph {
            adjacency_table: HashMap::new(),
		log::debug!(">>> Entering new()");
        log::debug!("<<< Exiting new()");
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
		log::debug!(">>> Entering adjacency_table_mutable()");
        log::debug!("<<< Exiting adjacency_table_mutable()");
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
		log::debug!(">>> Entering adjacency_table()");
        log::debug!("<<< Exiting adjacency_table()");
        &self.adjacency_table
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
		log::debug!(">>> Entering add_edge(edge={:?})", edge);
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_table
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
        self.adjacency_table
            .entry(edge.1.to_string())
            .and_modify(|e| {
                e.push((edge.0.to_string(), edge.2));
			log::debug!("<<< Exiting add_edge(edge={:?})", edge);
            });
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
		log::debug!(">>> Entering add_node(node={})", node);
        match self.adjacency_table().get(node) {
            None => {
                self.adjacency_table_mutable()
					log::debug!("<<< Exiting add_node(node={})", node);
                    .insert((*node).to_string(), Vec::new());
                true
            }
            _ => false,
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_table_mutable()
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
    }

    fn neighbours(&self, node: &str) -> Result<&Vec<(String, i32)>, NodeNotInGraph> {
		log::debug!(">>> Entering neighbours(node={})", node);
        log::debug!("<<< Exiting neighbours(node={})", node);
        match self.adjacency_table().get(node) {
            None => Err(NodeNotInGraph),
            Some(i) => Ok(i),
        }
    }

    fn contains(&self, node: &str) -> bool {
		log::debug!(">>> Entering contains(node={})", node);
        log::debug!("<<< Exiting contains(node={})", node);
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&String> {
		log::debug!(">>> Entering nodes()");
        log::debug!("<<< Exiting nodes()");
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
		log::debug!(">>> Entering edges()");
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
			log::debug!("edges(): edges → (unprintable)");
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
		log::debug!("<<< Exiting edges()");
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
		log::debug!(">>> Entering test_add_edge()");
        let mut graph = UndirectedGraph::new();

		log::debug!("test_add_edge(): graph → (unprintable)");
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
		log::debug!("<<< Exiting test_add_edge()");
        }
    }

    #[test]
    fn test_neighbours() {
		log::debug!(">>> Entering test_neighbours()");
        let mut graph = UndirectedGraph::new();

		log::debug!("test_neighbours(): graph → (unprintable)");
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(
            graph.neighbours("a").unwrap(),
            &vec![(String::from("b"), 5), (String::from("c"), 7)]
		log::debug!("<<< Exiting test_neighbours()");
        );
    }
}

#[cfg(test)]
mod test_directed_graph {
    use super::DirectedGraph;
    use super::Graph;

    #[test]
    fn test_add_node() {
		log::debug!(">>> Entering test_add_node()");
        let mut graph = DirectedGraph::new();
        graph.add_node("a");
		log::debug!("test_add_node(): graph → (unprintable)");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(
            graph.nodes(),
            [&String::from("a"), &String::from("b"), &String::from("c")]
                .iter()
                .cloned()
                .collect()
		log::debug!("<<< Exiting test_add_node()");
        );
    }

    #[test]
    fn test_add_edge() {
        let mut graph = DirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("c", "a", 7));
        graph.add_edge(("b", "c", 10));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("b"), &String::from("c"), 10),
        ];
        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph = DirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(
            graph.neighbours("a").unwrap(),
            &vec![(String::from("b"), 5)]
        );
    }

    #[test]
    fn test_contains() {
		log::debug!(">>> Entering test_contains()");
        let mut graph = DirectedGraph::new();
        graph.add_node("a");
		log::debug!("test_contains(): graph → (unprintable)");
        graph.add_node("b");
        graph.add_node("c");
        assert!(graph.contains("a"));
        assert!(graph.contains("b"));
        assert!(graph.contains("c"));
		log::debug!("<<< Exiting test_contains()");
        assert!(!graph.contains("d"));
    }
}
