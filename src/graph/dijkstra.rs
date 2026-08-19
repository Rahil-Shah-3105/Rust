use std::collections::{BTreeMap, BTreeSet};
use std::ops::Add;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

// performs Dijsktra's algorithm on the given graph from the given start
// the graph is a positively-weighted directed graph
//
// returns a map that for each reachable vertex associates the distance and the predecessor
// since the start has no predecessor but is reachable, map[start] will be None
//
// Time: O(E * logV). For each vertex, we traverse each edge, resulting in O(E). For each edge, we
// insert a new shortest path for a vertex into the tree, resulting in O(E * logV).
// Space: O(V). The tree holds up to V vertices.
pub fn dijkstra<V: Ord + Copy, E: Ord + Copy + Add<Output = E>>(
    graph: &Graph<V, E>,
    start: V,
) -> BTreeMap<V, Option<(V, E)>> {
    let mut ans = BTreeMap::new();
    let mut prio = BTreeSet::new();

    // start is the special case that doesn't have a predecessor
    ans.insert(start, None);

    for (new, weight) in &graph[&start] {
        ans.insert(*new, Some((start, *weight)));
        prio.insert((*weight, *new));
    }

    while let Some((path_weight, vertex)) = prio.pop_first() {
        for (next, weight) in &graph[&vertex] {
            let new_weight = path_weight + *weight;
            match ans.get(next) {
                // if ans[next] is a lower dist than the alternative one, we do nothing
                Some(Some((_, dist_next))) if new_weight >= *dist_next => {}
                // if ans[next] is None then next is start and so the distance won't be changed, it won't be added again in prio
                Some(None) => {}
                // the new path is shorter, either new was not in ans or it was farther
                _ => {
                    if let Some(Some((_, prev_weight))) =
                        ans.insert(*next, Some((vertex, new_weight)))
                    {
                        prio.remove(&(prev_weight, *next));
                    }
                    prio.insert((new_weight, *next));
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::{dijkstra, Graph};
    use std::collections::BTreeMap;

    fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
		log::debug!(">>> Entering add_edge(graph=(unprintable),v1=(unprintable),v2=(unprintable),c=(unprintable))");
        graph.entry(v1).or_default().insert(v2, c);
		log::debug!("<<< Exiting add_edge(graph=(unprintable),v1=(unprintable),v2=(unprintable),c=(unprintable))");
        graph.entry(v2).or_default();
    }

    #[test]
    fn single_vertex() {
		log::debug!(">>> Entering single_vertex()");
        let mut graph: Graph<usize, usize> = BTreeMap::new();
        graph.insert(0, BTreeMap::new());

        let mut dists = BTreeMap::new();
		log::debug!("single_vertex(): dists → (unprintable)");
        dists.insert(0, None);

		log::debug!("<<< Exiting single_vertex()");
        assert_eq!(dijkstra(&graph, 0), dists);
    }

    #[test]
    fn single_edge() {
		log::debug!(">>> Entering single_edge()");
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 0, 1, 2);
		log::debug!("single_edge(): graph → (unprintable)");

        let mut dists_0 = BTreeMap::new();
		log::debug!("single_edge(): dists_0 → (unprintable)");
        dists_0.insert(0, None);
        dists_0.insert(1, Some((0, 2)));

        assert_eq!(dijkstra(&graph, 0), dists_0);

        let mut dists_1 = BTreeMap::new();
		log::debug!("single_edge(): dists_1 → (unprintable)");
        dists_1.insert(1, None);

		log::debug!("<<< Exiting single_edge()");
        assert_eq!(dijkstra(&graph, 1), dists_1);
    }

    #[test]
    fn tree_1() {
		log::debug!(">>> Entering tree_1()");
        let mut graph = BTreeMap::new();
        let mut dists = BTreeMap::new();
		log::debug!("tree_1(): dists → (unprintable)");
		log::debug!("tree_1(): graph → (unprintable)");
        dists.insert(1, None);
        for i in 1..100 {
            add_edge(&mut graph, i, i * 2, i * 2);
            add_edge(&mut graph, i, i * 2 + 1, i * 2 + 1);

            match dists[&i] {
                Some((_, d)) => {
                    dists.insert(i * 2, Some((i, d + i * 2)));
                    dists.insert(i * 2 + 1, Some((i, d + i * 2 + 1)));
                }
                None => {
                    dists.insert(i * 2, Some((i, i * 2)));
                    dists.insert(i * 2 + 1, Some((i, i * 2 + 1)));
                }
            }
        }

		log::debug!("<<< Exiting tree_1()");
        assert_eq!(dijkstra(&graph, 1), dists);
    }

    #[test]
    fn graph_1() {
		log::debug!(">>> Entering graph_1()");
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 'a', 'c', 12);
		log::debug!("graph_1(): graph → (unprintable)");
        add_edge(&mut graph, 'a', 'd', 60);
        add_edge(&mut graph, 'b', 'a', 10);
        add_edge(&mut graph, 'c', 'b', 20);
        add_edge(&mut graph, 'c', 'd', 32);
        add_edge(&mut graph, 'e', 'a', 7);

        let mut dists_a = BTreeMap::new();
		log::debug!("graph_1(): dists_a → (unprintable)");
        dists_a.insert('a', None);
        dists_a.insert('c', Some(('a', 12)));
        dists_a.insert('d', Some(('c', 44)));
        dists_a.insert('b', Some(('c', 32)));
        assert_eq!(dijkstra(&graph, 'a'), dists_a);

        let mut dists_b = BTreeMap::new();
		log::debug!("graph_1(): dists_b → (unprintable)");
        dists_b.insert('b', None);
        dists_b.insert('a', Some(('b', 10)));
        dists_b.insert('c', Some(('a', 22)));
        dists_b.insert('d', Some(('c', 54)));
        assert_eq!(dijkstra(&graph, 'b'), dists_b);

        let mut dists_c = BTreeMap::new();
		log::debug!("graph_1(): dists_c → (unprintable)");
        dists_c.insert('c', None);
        dists_c.insert('b', Some(('c', 20)));
        dists_c.insert('d', Some(('c', 32)));
        dists_c.insert('a', Some(('b', 30)));
        assert_eq!(dijkstra(&graph, 'c'), dists_c);

        let mut dists_d = BTreeMap::new();
		log::debug!("graph_1(): dists_d → (unprintable)");
        dists_d.insert('d', None);
        assert_eq!(dijkstra(&graph, 'd'), dists_d);

        let mut dists_e = BTreeMap::new();
		log::debug!("graph_1(): dists_e → (unprintable)");
        dists_e.insert('e', None);
        dists_e.insert('a', Some(('e', 7)));
        dists_e.insert('c', Some(('a', 19)));
        dists_e.insert('d', Some(('c', 51)));
        dists_e.insert('b', Some(('c', 39)));
		log::debug!("<<< Exiting graph_1()");
        assert_eq!(dijkstra(&graph, 'e'), dists_e);
    }
}
