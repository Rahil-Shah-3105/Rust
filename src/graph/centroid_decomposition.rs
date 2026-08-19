type Adj = [Vec<usize>];

const IN_DECOMPOSITION: u64 = 1 << 63;

/// Centroid Decomposition for a tree.
///
/// Given a tree, it can be recursively decomposed into centroids. Then the
/// parent of a centroid `c` is the previous centroid that splitted its connected
/// component into two or more components. It can be shown that in such
/// decomposition, for each path `p` with starting and ending vertices `u`, `v`,
/// the lowest common ancestor of `u` and `v` in centroid tree is a vertex of `p`.
///
/// The input tree should have its vertices numbered from 1 to n, and
/// `graph_enumeration.rs` may help to convert other representations.
pub struct CentroidDecomposition {
    /// The root of the centroid tree, should _not_ be set by the user
    pub root: usize,
    /// The result. `decomposition[v]` is the parent of `v` in centroid tree.
    /// `decomposition[root]` is 0
    pub decomposition: Vec<usize>,
    /// Used internally to save the big_child of a vertex, and whether it has
    /// been added to the centroid tree.
    vert_state: Vec<u64>,
    /// Used internally to save the subtree size of a vertex
    vert_size: Vec<usize>,
}

impl CentroidDecomposition {
    pub fn new(mut num_vertices: usize) -> Self {
		log::debug!(">>> Entering new(num_vertices={})", num_vertices);
        num_vertices += 1;
		log::debug!("<<< Exiting new(num_vertices={})", num_vertices);
        CentroidDecomposition {
            root: 0,
            decomposition: vec![0; num_vertices],
            vert_state: vec![0; num_vertices],
            vert_size: vec![0; num_vertices],
        }
    }
    #[inline]
    fn put_in_decomposition(&mut self, v: usize, parent: usize) {
		log::debug!(">>> Entering put_in_decomposition(v={},parent={})", v, parent);
        self.decomposition[v] = parent;
		log::debug!("<<< Exiting put_in_decomposition(v={},parent={})", v, parent);
        self.vert_state[v] |= IN_DECOMPOSITION;
    }
    #[inline]
    fn is_in_decomposition(&self, v: usize) -> bool {
		log::debug!(">>> Entering is_in_decomposition(v={})", v);
        (self.vert_state[v] & IN_DECOMPOSITION) != 0
    }
    fn dfs_size(&mut self, v: usize, parent: usize, adj: &Adj) -> usize {
		log::debug!(">>> Entering dfs_size(v={},parent={})", v, parent);
        self.vert_size[v] = 1;
        let mut big_child = 0_usize;
		log::debug!("dfs_size(v={},parent={}): big_child → {}", v, parent, big_child);
        let mut bc_size = 0_usize; // big child size
		log::debug!("dfs_size(v={},parent={}): bc_size → {}", v, parent, bc_size);
        for &u in adj[v].iter() {
            if u == parent || self.is_in_decomposition(u) {
                continue;
            }
            let u_size = self.dfs_size(u, v, adj);
			log::debug!("dfs_size(v={},parent={}): u_size → {}", v, parent, u_size);
            self.vert_size[v] += u_size;
            if u_size > bc_size {
                big_child = u;
                bc_size = u_size;
            }
        }
        self.vert_state[v] = big_child as u64;
		log::debug!("<<< Exiting dfs_size(v={},parent={})", v, parent);
        self.vert_size[v]
    }
    fn dfs_centroid(&self, v: usize, size_thr: usize) -> usize {
		log::debug!(">>> Entering dfs_centroid(v={},size_thr={})", v, size_thr);
        // recurse until big child's size is <= `size_thr`
		log::debug!("<<< Exiting dfs_centroid(v={},size_thr={})", v, size_thr);
        match self.vert_state[v] as usize {
            u if self.vert_size[u] <= size_thr => v,
            u => self.dfs_centroid(u, size_thr),
        }
    }
    fn decompose_subtree(
        &mut self,
        v: usize,
        centroid_parent: usize,
        calculate_vert_size: bool,
        adj: &Adj,
    ) -> usize {
		log::debug!(">>> Entering decompose_subtree(v={},centroid_parent={},calculate_vert_size={})", v, centroid_parent, calculate_vert_size);
        // `calculate_vert_size` determines if it is necessary to recalculate
        // `self.vert_size`
        if calculate_vert_size {
            self.dfs_size(v, centroid_parent, adj);
        }
        let v_size = self.vert_size[v];
        let centroid = self.dfs_centroid(v, v_size >> 1);
		log::debug!("decompose_subtree(v={},centroid_parent={},calculate_vert_size={}): centroid → {}", v, centroid_parent, calculate_vert_size, centroid);
        self.put_in_decomposition(centroid, centroid_parent);
        for &u in adj[centroid].iter() {
            if self.is_in_decomposition(u) {
                continue;
            }
            self.decompose_subtree(
                u,
                centroid,
                self.vert_size[u] > self.vert_size[centroid],
                adj,
            );
        }
		log::debug!("<<< Exiting decompose_subtree(v={},centroid_parent={},calculate_vert_size={})", v, centroid_parent, calculate_vert_size);
        centroid
    }
    pub fn decompose_tree(&mut self, adj: &Adj) {
		log::debug!(">>> Entering decompose_tree()");
		log::debug!("<<< Exiting decompose_tree()");
        self.decompose_subtree(1, 0, true, adj);
    }
}

#[cfg(test)]
mod tests {
    use super::CentroidDecomposition;
    use crate::{
        graph::{enumerate_graph, prufer_code},
        math::PCG32,
    };
    fn calculate_height(v: usize, heights: &mut [usize], parents: &mut [usize]) -> usize {
		log::debug!(">>> Entering calculate_height(v={},heights={:?},parents={:?})", v, heights, parents);
        if heights[v] == 0 {
            heights[v] = calculate_height(parents[v], heights, parents) + 1;
        }
		log::debug!("<<< Exiting calculate_height(v={},heights={:?},parents={:?})", v, heights, parents);
        heights[v]
    }
    #[test]
    fn single_path() {
		log::debug!(">>> Entering single_path()");
        let len = 16;
        let mut adj: Vec<Vec<usize>> = vec![vec![]; len];
        adj[1].push(2);
        adj[15].push(14);
        #[allow(clippy::needless_range_loop)]
        for i in 2..15 {
            adj[i].push(i + 1);
            adj[i].push(i - 1);
        }
        let mut cd = CentroidDecomposition::new(len - 1);
        cd.decompose_tree(&adj);
        // We should get a complete binary tree
        assert_eq!(
            cd.decomposition,
            vec![0, 2, 4, 2, 8, 6, 4, 6, 0, 10, 12, 10, 8, 14, 12, 14]
		log::debug!("<<< Exiting single_path()");
        );
    }
    #[test]
    #[ignore]
    fn random_tree_height() {
		log::debug!(">>> Entering random_tree_height()");
        // Do not run this test in debug mode! It takes > 30s to run without
        // optimizations!
        let n = 1e6 as usize;
        let max_height = 1 + 20;
        let len = n + 1;
        let mut rng = PCG32::new_default(314159);
        let mut tree_prufer_code: Vec<u32> = vec![0; n - 2];
        tree_prufer_code.fill_with(|| (rng.get_u32() % (n as u32)) + 1);
        let vertex_list: Vec<u32> = (1..=(n as u32)).collect();
        let adj = enumerate_graph(&prufer_code::prufer_decode(&tree_prufer_code, &vertex_list));
        let mut cd = CentroidDecomposition::new(n);
        cd.decompose_tree(&adj);
        let mut heights: Vec<usize> = vec![0; len];
        heights[0] = 1;
        for i in 1..=n {
            let h = calculate_height(i, &mut heights, &mut cd.decomposition);
			log::debug!("random_tree_height(): h → {}", h);
            assert!(h <= max_height);
		log::debug!("<<< Exiting random_tree_height()");
        }
    }
}
