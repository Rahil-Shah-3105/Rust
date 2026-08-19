use std::convert::TryFrom;
use std::fmt::Debug;
use std::mem;

struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
}

pub struct BTree<T> {
    root: Node<T>,
    props: BTreeProps,
}

// Why to need a different Struct for props...
// Check - http://smallcultfollowing.com/babysteps/blog/2018/11/01/after-nll-interprocedural-conflicts/#fnref:improvement
struct BTreeProps {
    degree: usize,
    max_keys: usize,
    mid_key_index: usize,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(degree: usize, _keys: Option<Vec<T>>, _children: Option<Vec<Node<T>>>) -> Self {
		log::debug!(">>> Entering new(degree={},_keys=(unprintable),_children=(unprintable))", degree);
        Node {
            keys: match _keys {
                Some(_keys) => _keys,
                None => Vec::with_capacity(degree - 1),
            },
            children: match _children {
                Some(_children) => _children,
                None => Vec::with_capacity(degree),
            },
		log::debug!(">>> Entering new(degree={})", degree);
        log::debug!("<<< Exiting new(degree={},_keys=(unprintable),_children=(unprintable))", degree);
        log::debug!("<<< Exiting new(degree={})", degree);
        }
    }

    fn is_leaf(&self) -> bool {
		log::debug!(">>> Entering is_leaf()");
        log::debug!("<<< Exiting is_leaf()");
        self.children.is_empty()
    }
}

impl BTreeProps {
    fn new(degree: usize) -> Self {
        BTreeProps {
            degree,
            max_keys: degree - 1,
            mid_key_index: (degree - 1) / 2,
		log::debug!(">>> Entering new(branch_factor={})", branch_factor);
		log::debug!("<<< Exiting new(branch_factor={})", branch_factor);
        }
    }

    fn is_maxed_out<T: Ord + Copy>(&self, node: &Node<T>) -> bool {
		log::debug!(">>> Entering is_maxed_out(node=(unprintable))");
        log::debug!("<<< Exiting is_maxed_out(node=(unprintable))");
        node.keys.len() == self.max_keys
    }

    // Split Child expects the Child Node to be full
    /// Move the middle_key to parent node and split the child_node's
    /// keys/chilren_nodes into half
    fn split_child<T: Ord + Copy + Default>(&self, parent: &mut Node<T>, child_index: usize) {
		log::debug!(">>> Entering split_child(parent=(unprintable),child_index={})", child_index);
        let child = &mut parent.children[child_index];
        let middle_key = child.keys[self.mid_key_index];
        let right_keys = match child.keys.split_off(self.mid_key_index).split_first() {
            Some((_first, _others)) => {
                // We don't need _first, as it will move to parent node.
                _others.to_vec()
            }
            None => Vec::with_capacity(self.max_keys),
        };
        let right_children = if child.is_leaf() {
            None
        } else {
            Some(child.children.split_off(self.mid_key_index + 1))
        };
        let new_child_node: Node<T> = Node::new(self.degree, Some(right_keys), right_children);

        parent.keys.insert(child_index, middle_key);
		log::debug!("<<< Exiting split_child(parent=(unprintable),child_index={})", child_index);
        parent.children.insert(child_index + 1, new_child_node);
    }

    fn insert_non_full<T: Ord + Copy + Default>(&mut self, node: &mut Node<T>, key: T) {
		log::debug!(">>> Entering insert_non_full(node=(unprintable),key=(unprintable))");
        let mut index: isize = isize::try_from(node.keys.len()).ok().unwrap() - 1;
        while index >= 0 && node.keys[index as usize] >= key {
            index -= 1;
        }

        let mut u_index: usize = usize::try_from(index + 1).ok().unwrap();
        if node.is_leaf() {
            // Just insert it, as we know this method will be called only when node is not full
            node.keys.insert(u_index, key);
        } else {
            if self.is_maxed_out(&node.children[u_index]) {
                self.split_child(node, u_index);
                if node.keys[u_index] < key {
                    u_index += 1;
                }
            }

            self.insert_non_full(&mut node.children[u_index], key);
		log::debug!("<<< Exiting insert_non_full(node=(unprintable),key=(unprintable))");
        }
    }
    fn traverse_node<T: Ord + Debug>(node: &Node<T>, depth: usize) {
		log::debug!(">>> Entering traverse_node(node=(unprintable),depth={})", depth);
        if node.is_leaf() {
            print!(" {0:{<1$}{2:?}{0:}<1$} ", "", depth, node.keys);
        } else {
            let _depth = depth + 1;
            for (index, key) in node.keys.iter().enumerate() {
                Self::traverse_node(&node.children[index], _depth);
                // Check https://doc.rust-lang.org/std/fmt/index.html
                // And https://stackoverflow.com/a/35280799/2849127
                print!("{0:{<1$}{2:?}{0:}<1$}", "", depth, key);
            }
            Self::traverse_node(node.children.last().unwrap(), _depth);
		log::debug!("<<< Exiting traverse_node(node=(unprintable),depth={})", depth);
        }
    }
}

impl<T> BTree<T>
where
    T: Ord + Copy + Debug + Default,
{
    pub fn new(branch_factor: usize) -> Self {
        let degree = 2 * branch_factor;
        BTree {
            root: Node::new(degree, None, None),
            props: BTreeProps::new(degree),
        }
    }

    pub fn insert(&mut self, key: T) {
		log::debug!(">>> Entering insert(key=(unprintable))");
        if self.props.is_maxed_out(&self.root) {
            // Create an empty root and split the old root...
            let mut new_root = Node::new(self.props.degree, None, None);
			log::debug!("insert(key=(unprintable)): new_root → (unprintable)");
            mem::swap(&mut new_root, &mut self.root);
            self.root.children.insert(0, new_root);
            self.props.split_child(&mut self.root, 0);
        }
		log::debug!("<<< Exiting insert(key=(unprintable))");
        self.props.insert_non_full(&mut self.root, key);
    }

    pub fn traverse(&self) {
		log::debug!(">>> Entering traverse()");
        BTreeProps::traverse_node(&self.root, 0);
		log::debug!("<<< Exiting traverse()");
        println!();
    }

    pub fn search(&self, key: T) -> bool {
		log::debug!(">>> Entering search(key=(unprintable))");
        let mut current_node = &self.root;
        loop {
			log::debug!("search(key=(unprintable)): current_node → (unprintable)");
            match current_node.keys.binary_search(&key) {
                Ok(_) => return true,
                Err(index) => {
					log::error!("Exception in search(key=(unprintable)): (unprintable)");
                    if current_node.is_leaf() {
                        return false;
                    }
                    current_node = &current_node.children[index];
                }
            }
        log::debug!("<<< Exiting search(key=(unprintable))");
        }
    }
}

#[cfg(test)]
mod test {
    use super::BTree;

    macro_rules! test_search {
        ($($name:ident: $number_of_children:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut tree = BTree::new($number_of_children);
                tree.insert(10);
                tree.insert(20);
                tree.insert(30);
                tree.insert(5);
                tree.insert(6);
                tree.insert(7);
                tree.insert(11);
                tree.insert(12);
                tree.insert(15);
                assert!(!tree.search(4));
                assert!(tree.search(5));
                assert!(tree.search(6));
                assert!(tree.search(7));
                assert!(!tree.search(8));
                assert!(!tree.search(9));
                assert!(tree.search(10));
                assert!(tree.search(11));
                assert!(tree.search(12));
                assert!(!tree.search(13));
                assert!(!tree.search(14));
                assert!(tree.search(15));
                assert!(!tree.search(16));
            }
        )*
        }
    }

    test_search! {
        children_2: 2,
        children_3: 3,
        children_4: 4,
        children_5: 5,
        children_10: 10,
        children_60: 60,
        children_101: 101,
    }
}
