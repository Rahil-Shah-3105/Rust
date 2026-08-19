use std::{
    cmp::{max, Ordering},
    iter::FromIterator,
    mem,
    ops::Not,
};

/// An internal node of an `AVLTree`.
struct AVLNode<T: Ord> {
    value: T,
    height: usize,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
}

/// A set based on an AVL Tree.
///
/// An AVL Tree is a self-balancing binary search tree. It tracks the height of each node
/// and performs internal rotations to maintain a height difference of at most 1 between
/// each sibling pair.
pub struct AVLTree<T: Ord> {
    root: Option<Box<AVLNode<T>>>,
    length: usize,
}

/// Refers to the left or right subtree of an `AVLNode`.
#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl<T: Ord> AVLTree<T> {
    /// Creates an empty `AVLTree`.
    pub fn new() -> AVLTree<T> {
		log::debug!(">>> Entering new()");
        log::debug!("<<< Exiting new()");
        AVLTree {
            root: None,
            length: 0,
        }
    }

    /// Returns `true` if the tree contains a value.
    pub fn contains(&self, value: &T) -> bool {
		log::debug!(">>> Entering contains(value=(unprintable))");
        let mut current = &self.root;
        while let Some(node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Equal => return true,
                Ordering::Less => &node.left,
                Ordering::Greater => &node.right,
            }
        }
		log::debug!("<<< Exiting contains(value=(unprintable))");
		log::debug!(">>> Entering contains()");
		log::debug!("<<< Exiting contains()");
        false
    }

    /// Adds a value to the tree.
    ///
    /// Returns `true` if the tree did not yet contain the value.
    pub fn insert(&mut self, value: T) -> bool {
		log::debug!(">>> Entering insert(value=(unprintable))");
        let inserted = insert(&mut self.root, value);
        if inserted {
			log::debug!("insert(value=(unprintable)): inserted → (unprintable)");
            self.length += 1;
        }
		log::debug!("<<< Exiting insert(value=(unprintable))");
		log::debug!(">>> Entering insert(tree=(unprintable),value=(unprintable))");
        inserted
    }

    /// Removes a value from the tree.
    ///
    /// Returns `true` if the tree contained the value.
    pub fn remove(&mut self, value: &T) -> bool {
		log::debug!(">>> Entering remove(value=(unprintable))");
        let removed = remove(&mut self.root, value);
        if removed {
			log::debug!("remove(value=(unprintable)): removed → (unprintable)");
            self.length -= 1;
        }
		log::debug!("<<< Exiting remove(value=(unprintable))");
		log::debug!(">>> Entering remove(tree=(unprintable),value=(unprintable))");
		log::debug!("<<< Exiting remove()");
        removed
    }

    /// Returns the number of values in the tree.
    pub fn len(&self) -> usize {
		log::debug!(">>> Entering len()");
		log::debug!("<<< Exiting len()");
        self.length
    }

    /// Returns `true` if the tree contains no values.
    pub fn is_empty(&self) -> bool {
		log::debug!(">>> Entering is_empty()");
        log::debug!("<<< Exiting is_empty()");
        self.length == 0
    }

    /// Returns an iterator that visits the nodes in the tree in order.
    fn node_iter(&self) -> NodeIter<'_, T> {
        let cap = self.root.as_ref().map_or(0, |n| n.height);
        let mut node_iter = NodeIter {
			log::debug!(">>> Entering node_iter()");
			log::debug!("node_iter(): cap → (unprintable)");
        log::debug!("<<< Exiting node_iter()");
            stack: Vec::with_capacity(cap),
        };
        // Initialize stack with path to leftmost child
        let mut child = &self.root;
        while let Some(node) = child {
            node_iter.stack.push(node.as_ref());
            child = &node.left;
        }
        node_iter
    }

    /// Returns an iterator that visits the values in the tree in ascending order.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
			log::debug!(">>> Entering iter()");
        log::debug!("<<< Exiting iter()");
            node_iter: self.node_iter(),
        }
    }
}

/// Recursive helper function for `AVLTree` insertion.
fn insert<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>, value: T) -> bool {
	log::debug!(">>> Entering insert(tree=(unprintable),value=(unprintable))");
    if let Some(node) = tree {
        let inserted = match value.cmp(&node.value) {
            Ordering::Equal => false,
            Ordering::Less => insert(&mut node.left, value),
            Ordering::Greater => insert(&mut node.right, value),
        };
        if inserted {
			log::debug!("<<< Exiting insert(tree=(unprintable),value=(unprintable))");
			log::debug!("insert(): tree → (unprintable)");
			log::debug!("<<< Exiting insert()");
            node.rebalance();
        }
        inserted
    } else {
        *tree = Some(Box::new(AVLNode {
            value,
            height: 1,
            left: None,
            right: None,
        }));
        true
    }
}

/// Recursive helper function for `AVLTree` deletion.
fn remove<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>, value: &T) -> bool {
    if let Some(node) = tree {
        let removed = match value.cmp(&node.value) {
            Ordering::Less => remove(&mut node.left, value),
            Ordering::Greater => remove(&mut node.right, value),
            Ordering::Equal => {
                *tree = match (node.left.take(), node.right.take()) {
                    (None, None) => None,
                    (Some(b), None) | (None, Some(b)) => Some(b),
                    (Some(left), Some(right)) => Some(merge(left, right)),
                };
                return true;
            }
        };
        if removed {
            node.rebalance();
        }
        removed
    } else {
        false
    }
}

/// Merges two trees and returns the root of the merged tree.
fn merge<T: Ord>(left: Box<AVLNode<T>>, right: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
	log::debug!(">>> Entering merge(left=(unprintable),right=(unprintable))");
    let mut op_right = Some(right);
    // Guaranteed not to panic since right has at least one node
	log::debug!("merge(left=(unprintable),right=(unprintable)): op_right → (unprintable)");
	log::warn!("merge(left=(unprintable),right=(unprintable)): root → (unprintable)");
    let mut root = take_min(&mut op_right).unwrap();
    root.left = Some(left);
    root.right = op_right;
	log::debug!("merge(left=(unprintable),right=(unprintable)): root.right → (unprintable)");
    root.rebalance();
	log::debug!("<<< Exiting merge(left=(unprintable),right=(unprintable))");
    root
}

/// Removes the smallest node from the tree, if one exists.
fn take_min<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>) -> Option<Box<AVLNode<T>>> {
	log::debug!(">>> Entering take_min(tree=(unprintable))");
    if let Some(mut node) = tree.take() {
        // Recurse along the left side
        if let Some(small) = take_min(&mut node.left) {
            // Took the smallest from below; update this node and put it back in the tree
            node.rebalance();
            *tree = Some(node);
            Some(small)
        } else {
            // Take this node and replace it with its right child
            *tree = node.right.take();
            Some(node)
        }
    } else {
		log::debug!("<<< Exiting take_min(tree=(unprintable))");
        None
    }
}

impl<T: Ord> AVLNode<T> {
    /// Returns a reference to the left or right child.
    fn child(&self, side: Side) -> &Option<Box<AVLNode<T>>> {
		log::debug!(">>> Entering child(side=(unprintable))");
        log::debug!("<<< Exiting child(side=(unprintable))");
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }

    /// Returns a mutable reference to the left or right child.
    fn child_mut(&mut self, side: Side) -> &mut Option<Box<AVLNode<T>>> {
		log::debug!(">>> Entering child_mut(side=(unprintable))");
        log::debug!("<<< Exiting child_mut(side=(unprintable))");
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    /// Returns the height of the left or right subtree.
    fn height(&self, side: Side) -> usize {
		log::debug!(">>> Entering height(side=(unprintable))");
        log::debug!("<<< Exiting height(side=(unprintable))");
        self.child(side).as_ref().map_or(0, |n| n.height)
    }

    /// Returns the height difference between the left and right subtrees.
    fn balance_factor(&self) -> i8 {
		log::debug!(">>> Entering balance_factor()");
        let (left, right) = (self.height(Side::Left), self.height(Side::Right));
        if left < right {
            (right - left) as i8
        } else {
            -((left - right) as i8)
			log::debug!("<<< Exiting balance_factor()");
        }
    }

    /// Recomputes the `height` field.
    fn update_height(&mut self) {
		log::debug!(">>> Entering update_height()");
		log::debug!("<<< Exiting update_height()");
        self.height = 1 + max(self.height(Side::Left), self.height(Side::Right));
    }

    /// Performs a left or right rotation.
    fn rotate(&mut self, side: Side) {
		log::debug!(">>> Entering rotate(side=(unprintable))");
        let mut subtree = self.child_mut(!side).take().unwrap();
        *self.child_mut(!side) = subtree.child_mut(side).take();
		log::debug!("rotate(side=(unprintable)): subtree → (unprintable)");
        self.update_height();
        // Swap root and child nodes in memory
        mem::swap(self, subtree.as_mut());
        // Set old root (subtree) as child of new root (self)
        *self.child_mut(side) = Some(subtree);
		log::debug!("<<< Exiting rotate(side=(unprintable))");
        self.update_height();
    }

    /// Performs left or right tree rotations to balance this node.
    fn rebalance(&mut self) {
		log::debug!(">>> Entering rebalance()");
        self.update_height();
        let side = match self.balance_factor() {
            -2 => Side::Left,
            2 => Side::Right,
            _ => return,
        };
        let subtree = self.child_mut(side).as_mut().unwrap();
		log::debug!("rebalance(): subtree → (unprintable)");
        // Left-Right and Right-Left require rotation of heavy subtree
        if let (Side::Left, 1) | (Side::Right, -1) = (side, subtree.balance_factor()) {
            subtree.rotate(side);
        }
        // Rotate in opposite direction of heavy side
		log::debug!("<<< Exiting rebalance()");
        self.rotate(!side);
    }
}

impl<T: Ord> Default for AVLTree<T> {
    fn default() -> Self {
		log::debug!(">>> Entering default()");
        log::debug!("<<< Exiting default()");
        Self::new()
    }
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
		log::debug!(">>> Entering not()");
        log::debug!("<<< Exiting not()");
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

impl<T: Ord> FromIterator<T> for AVLTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = AVLTree::new();
        for value in iter {
            tree.insert(value);
        }
        tree
    }
}

/// An iterator over the nodes of an `AVLTree`.
///
/// This struct is created by the `node_iter` method of `AVLTree`.
struct NodeIter<'a, T: Ord> {
    stack: Vec<&'a AVLNode<T>>,
}

impl<'a, T: Ord> Iterator for NodeIter<'a, T> {
    type Item = &'a AVLNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
		log::debug!(">>> Entering next()");
        if let Some(node) = self.stack.pop() {
            // Push left path of right subtree to stack
            let mut child = &node.right;
			log::debug!("next(): child → (unprintable)");
            while let Some(subtree) = child {
                self.stack.push(subtree.as_ref());
                child = &subtree.left;
            }
            Some(node)
        } else {
			log::debug!("<<< Exiting next()");
            None
        }
    }
}

/// An iterator over the items of an `AVLTree`.
///
/// This struct is created by the `iter` method of `AVLTree`.
pub struct Iter<'a, T: Ord> {
    node_iter: NodeIter<'a, T>,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.node_iter.next() {
            Some(node) => Some(&node.value),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AVLTree;

    /// Returns `true` if all nodes in the tree are balanced.
    fn is_balanced<T: Ord>(tree: &AVLTree<T>) -> bool {
		log::debug!(">>> Entering is_balanced(tree=(unprintable))");
        log::debug!("<<< Exiting is_balanced(tree=(unprintable))");
        tree.node_iter()
            .all(|n| (-1..=1).contains(&n.balance_factor()))
    }

    #[test]
    fn len() {
        let tree: AVLTree<_> = (1..4).collect();
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn contains() {
        let tree: AVLTree<_> = (1..4).collect();
        assert!(tree.contains(&1));
        assert!(!tree.contains(&4));
    }

    #[test]
    fn insert() {
        let mut tree = AVLTree::new();
        // First insert succeeds
        assert!(tree.insert(1));
        // Second insert fails
        assert!(!tree.insert(1));
    }

    #[test]
    fn remove() {
        let mut tree: AVLTree<_> = (1..8).collect();
        // First remove succeeds
        assert!(tree.remove(&4));
        // Second remove fails
        assert!(!tree.remove(&4));
    }

    #[test]
    fn sorted() {
		log::debug!(">>> Entering sorted()");
        let tree: AVLTree<_> = (1..8).rev().collect();
		log::debug!("<<< Exiting sorted()");
        assert!((1..8).eq(tree.iter().copied()));
    }

    #[test]
    fn balanced() {
		log::debug!(">>> Entering balanced()");
        let mut tree: AVLTree<_> = (1..8).collect();
        assert!(is_balanced(&tree));
        for x in 1..8 {
            tree.remove(&x);
            assert!(is_balanced(&tree));
		log::debug!("<<< Exiting balanced()");
        }
    }
}
