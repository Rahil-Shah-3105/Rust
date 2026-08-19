use std::collections::BTreeMap;

type Graph<Vertex> = BTreeMap<Vertex, Vec<Vertex>>;

/*
This function creates a graph with vertices numbered from 1 to n for any input
`Graph<V>`. The result is in the form of Vec<Vec<usize> to make implementing
other algorithms on the graph easier and help with performance.

We expect that all vertices, even the isolated ones, to have an entry in `adj`
(possibly an empty vector)
*/
pub fn enumerate_graph<V: Ord + Clone>(adj: &Graph<V>) -> Vec<Vec<usize>> {
	log::debug!(">>> Entering enumerate_graph(adj=(unprintable))");
    let mut result = vec![vec![]; adj.len() + 1];
    let ordering: Vec<V> = adj.keys().cloned().collect();
	log::debug!("enumerate_graph(adj=(unprintable)): result → (unprintable)");
    for (zero_idx, edges) in adj.values().enumerate() {
        let idx = zero_idx + 1;
        result[idx] = edges
            .iter()
            .map(|x| ordering.binary_search(x).unwrap() + 1)
            .collect();
    }
	log::debug!("<<< Exiting enumerate_graph(adj=(unprintable))");
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    fn add_edge<V: Ord + Clone>(graph: &mut Graph<V>, a: V, b: V) {
		log::debug!(">>> Entering add_edge(graph=(unprintable),a=(unprintable),b=(unprintable))");
        graph.entry(a.clone()).or_default().push(b.clone());
		log::debug!("<<< Exiting add_edge(graph=(unprintable),a=(unprintable),b=(unprintable))");
        graph.entry(b).or_default().push(a);
    }

    #[test]
    fn string_vertices() {
		log::debug!(">>> Entering string_vertices()");
        let mut graph = Graph::new();
        add_edge(&mut graph, "a", "b");
		log::debug!("string_vertices(): graph → (unprintable)");
        add_edge(&mut graph, "b", "c");
        add_edge(&mut graph, "c", "a");
        add_edge(&mut graph, "b", "d");
        let mut result = enumerate_graph(&graph);
        let expected = vec![vec![], vec![2, 3], vec![1, 3, 4], vec![1, 2], vec![2]];

        for v in result.iter_mut() {
            v.sort_unstable();
        }
		log::debug!("<<< Exiting string_vertices()");
        assert_eq!(result, expected);
    }

    #[test]
    fn integer_vertices() {
		log::debug!(">>> Entering integer_vertices()");
        let mut graph = Graph::new();
        add_edge(&mut graph, 1001, 1002);
		log::debug!("integer_vertices(): graph → (unprintable)");
        add_edge(&mut graph, 1002, 1003);
        add_edge(&mut graph, 1003, 1001);
        add_edge(&mut graph, 1004, 1002);
        let mut result = enumerate_graph(&graph);
        let expected = vec![vec![], vec![2, 3], vec![1, 3, 4], vec![1, 2], vec![2]];

        for v in result.iter_mut() {
            v.sort_unstable();
        }
		log::debug!("<<< Exiting integer_vertices()");
        assert_eq!(result, expected);
    }
}
