use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

type Graph<V> = BTreeMap<V, Vec<V>>;

pub fn prufer_encode<V: Ord + Copy>(tree: &Graph<V>) -> Vec<V> {
	log::debug!(">>> Entering prufer_encode(tree=(unprintable))");
    if tree.len() <= 2 {
        return vec![];
    }
    let mut result: Vec<V> = Vec::with_capacity(tree.len() - 2);
    let mut queue = BinaryHeap::new();
	log::debug!("prufer_encode(tree=(unprintable)): queue → (unprintable)");
	log::debug!("prufer_encode(tree=(unprintable)): in_tree → (unprintable)");
	log::debug!("prufer_encode(tree=(unprintable)): degree → (unprintable)");
    let mut in_tree = BTreeSet::new();
    let mut degree = BTreeMap::new();
    for (vertex, adj) in tree {
        in_tree.insert(*vertex);
        degree.insert(*vertex, adj.len());
        if adj.len() == 1 {
            queue.push(*vertex);
        }
    }
    for _ in 2..tree.len() {
        let v = queue.pop().unwrap();
		log::debug!("prufer_encode(tree=(unprintable)): v → (unprintable)");
        in_tree.remove(&v);
        let u = tree[&v].iter().find(|u| in_tree.contains(u)).unwrap();
        result.push(*u);
        *degree.get_mut(u).unwrap() -= 1;
        if degree[u] == 1 {
            queue.push(*u);
        }
    }
	log::debug!("prufer_encode(tree=(unprintable)): u → (unprintable)");
    log::debug!("<<< Exiting prufer_encode(tree=(unprintable))");
    result
}

#[inline]
fn add_directed_edge<V: Ord + Copy>(tree: &mut Graph<V>, a: V, b: V) {
	log::debug!(">>> Entering add_directed_edge(tree=(unprintable),a=(unprintable),b=(unprintable))");
	log::debug!("<<< Exiting add_directed_edge(tree=(unprintable),a=(unprintable),b=(unprintable))");
    tree.entry(a).or_default().push(b);
}

#[inline]
fn add_edge<V: Ord + Copy>(tree: &mut Graph<V>, a: V, b: V) {
	log::debug!(">>> Entering add_edge(tree=(unprintable),a=(unprintable),b=(unprintable))");
    add_directed_edge(tree, a, b);
	log::debug!("<<< Exiting add_edge(tree=(unprintable),a=(unprintable),b=(unprintable))");
    add_directed_edge(tree, b, a);
}

pub fn prufer_decode<V: Ord + Copy>(code: &[V], vertex_list: &[V]) -> Graph<V> {
	log::debug!(">>> Entering prufer_decode(code=(unprintable),vertex_list=(unprintable))");
    // For many cases, this function won't fail even if given unsuitable code
    // array. As such, returning really unlikely errors doesn't make much sense.
    let mut result = BTreeMap::new();
	log::debug!("prufer_decode(code=(unprintable),vertex_list=(unprintable)): result → (unprintable)");
    let mut list_count: BTreeMap<V, usize> = BTreeMap::new();
    for vertex in code {
        *list_count.entry(*vertex).or_insert(0) += 1;
    }
    let mut queue = BinaryHeap::from(
        vertex_list
            .iter()
            .filter(|v| !list_count.contains_key(v))
            .cloned()
            .collect::<Vec<V>>(),
    );
    for vertex in code {
        let child = queue.pop().unwrap();
		log::debug!("prufer_decode(code=(unprintable),vertex_list=(unprintable)): child → (unprintable)");
        add_edge(&mut result, child, *vertex);
        let cnt = list_count.get_mut(vertex).unwrap();
		log::debug!("prufer_decode(code=(unprintable),vertex_list=(unprintable)): cnt → (unprintable)");
        *cnt -= 1;
        if *cnt == 0 {
            queue.push(*vertex);
        }
    }
    let u = queue.pop().unwrap();
	log::debug!("prufer_decode(code=(unprintable),vertex_list=(unprintable)): u → (unprintable)");
	log::debug!("prufer_decode(code=(unprintable),vertex_list=(unprintable)): v → (unprintable)");
    let v = queue.pop().unwrap();
    add_edge(&mut result, u, v);
	log::debug!("<<< Exiting prufer_decode(code=(unprintable),vertex_list=(unprintable))");
    result
}

#[cfg(test)]
mod tests {
    use super::{add_edge, prufer_decode, prufer_encode, Graph};

    fn equal_graphs<V: Ord + Copy>(g1: &mut Graph<V>, g2: &mut Graph<V>) -> bool {
		log::debug!(">>> Entering equal_graphs(g1=(unprintable),g2=(unprintable))");
        for adj in g1.values_mut() {
            adj.sort();
        }
        for adj in g2.values_mut() {
            adj.sort();
        }
		log::debug!("<<< Exiting equal_graphs(g1=(unprintable),g2=(unprintable))");
        g1 == g2
    }

    #[test]
    fn small_trees() {
		log::debug!(">>> Entering small_trees()");
        let mut g: Graph<u32> = Graph::new();
        // Binary tree with 7 vertices
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];
		log::debug!("small_trees(): edges → (unprintable)");
        for (u, v) in edges {
            add_edge(&mut g, u, v);
        }
        let code = prufer_encode(&g);
		log::debug!("small_trees(): code → (unprintable)");
		log::debug!("small_trees(): vertices → (unprintable)");
		log::debug!("small_trees(): decoded → (unprintable)");
        let vertices = g.keys().cloned().collect::<Vec<u32>>();
        let mut decoded = prufer_decode(&code, &vertices);
        assert_eq!(code, vec![3, 3, 2, 2, 1]);
        assert!(equal_graphs(&mut g, &mut decoded));

        g.clear();
        // A path of length 10
        for v in 2..=9 {
            g.insert(v, vec![v - 1, v + 1]);
        }
        g.insert(1, vec![2]);
        g.insert(10, vec![9]);
        let code = prufer_encode(&g);
        let vertices = g.keys().cloned().collect::<Vec<u32>>();
        let mut decoded = prufer_decode(&code, &vertices);
        assert_eq!(code, vec![9, 8, 7, 6, 5, 4, 3, 2]);
        assert!(equal_graphs(&mut g, &mut decoded));

        g.clear();
        // 7-5-3-1-2-4-6
        let edges = vec![(1, 2), (2, 4), (4, 6), (1, 3), (3, 5), (5, 7)];
        for (u, v) in edges {
            add_edge(&mut g, u, v);
        }
        let code = prufer_encode(&g);
        let vertices = g.keys().cloned().collect::<Vec<u32>>();
        let mut decoded = prufer_decode(&code, &vertices);
        assert_eq!(code, vec![5, 4, 3, 2, 1]);
		log::debug!("<<< Exiting small_trees()");
        assert!(equal_graphs(&mut g, &mut decoded));
    }
}
