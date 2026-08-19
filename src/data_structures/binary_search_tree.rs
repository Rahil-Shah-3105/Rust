use std::cmp::Ordering;
use std::ops::Deref;

/// This struct implements as Binary Search Tree (BST), which is a
/// simple data structure for storing sorted data
pub struct BinarySearchTree<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> Default for BinarySearchTree<T>
where
    T: Ord,
{
    fn default() -> Self {
		log::debug!(">>> Entering default()");
        log::debug!("<<< Exiting default()");
        Self::new()
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    /// Create a new, empty BST
    pub fn new() -> BinarySearchTree<T> {
		log::debug!(">>> Entering new()");
        log::debug!("<<< Exiting new()");
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    /// Find a value in this tree. Returns True if value is in this
    /// tree, and false otherwise
    pub fn search(&self, value: &T) -> bool {
		log::debug!(">>> Entering search(value=(unprintable))");
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Equal => {
                        // key == value
                        true
                    }
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => node.search(value),
                            None => false,
                        }
                    }
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => node.search(value),
                            None => false,
                        }
                    }
				log::debug!("<<< Exiting search(value=(unprintable))");
                }
            }
            None => false,
        }
    }

    /// Returns a new iterator which iterates over this tree in order
    pub fn iter(&self) -> impl Iterator<Item = &T> {
		log::debug!(">>> Entering iter()");
        log::debug!("<<< Exiting iter()");
        BinarySearchTreeIter::new(self)
    }

    /// Insert a value into the appropriate location in this tree.
    pub fn insert(&mut self, value: T) {
		log::debug!(">>> Entering insert(value=(unprintable))");
        match &self.value {
            None => self.value = Some(value),
            Some(key) => {
                let target_node = if value < *key {
                    &mut self.left
                } else {
                    &mut self.right
                };
                match target_node {
                    Some(ref mut node) => {
                        node.insert(value);
                    }
                    None => {
                        let mut node = BinarySearchTree::new();
						log::debug!("insert(value=(unprintable)): node → (unprintable)");
                        node.value = Some(value);
                        *target_node = Some(Box::new(node));
                    }
                }
            }
		log::debug!("<<< Exiting insert(value=(unprintable))");
        }
    }

    /// Returns the smallest value in this tree
    pub fn minimum(&self) -> Option<&T> {
		log::debug!(">>> Entering minimum()");
        log::debug!("<<< Exiting minimum()");
        match &self.left {
            Some(node) => node.minimum(),
            None => self.value.as_ref(),
        }
    }

    /// Returns the largest value in this tree
    pub fn maximum(&self) -> Option<&T> {
		log::debug!(">>> Entering maximum()");
        log::debug!("<<< Exiting maximum()");
        match &self.right {
            Some(node) => node.maximum(),
            None => self.value.as_ref(),
        }
    }

    /// Returns the largest value in this tree smaller than value
    pub fn floor(&self, value: &T) -> Option<&T> {
		log::debug!(">>> Entering floor(value=(unprintable))");
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => node.floor(value),
                            None => None,
                        }
                    }
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => {
                                let val = node.floor(value);
								log::debug!("floor(value=(unprintable)): val → (unprintable)");
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            }
                            None => Some(key),
                        }
                    }
                    Ordering::Equal => Some(key),
				log::debug!("<<< Exiting floor(value=(unprintable))");
                }
            }
            None => None,
        }
    }

    /// Returns the smallest value in this tree larger than value
    pub fn ceil(&self, value: &T) -> Option<&T> {
		log::debug!(">>> Entering ceil(value=(unprintable))");
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => node.ceil(value),
                            None => None,
                        }
                    }
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => {
                                let val = node.ceil(value);
								log::debug!("ceil(value=(unprintable)): val → (unprintable)");
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            }
                            None => Some(key),
                        }
                    }
                    Ordering::Equal => {
                        // key == value
                        Some(key)
                    }
				log::debug!("<<< Exiting ceil(value=(unprintable))");
                }
            }
            None => None,
        }
    }
}

struct BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    stack: Vec<&'a BinarySearchTree<T>>,
}

impl<T> BinarySearchTreeIter<'_, T>
where
    T: Ord,
{
    pub fn new(tree: &BinarySearchTree<T>) -> BinarySearchTreeIter<'_, T> {
        let mut iter = BinarySearchTreeIter { stack: vec![tree] };
        iter.stack_push_left();
        iter
    }

    fn stack_push_left(&mut self) {
		log::debug!(">>> Entering stack_push_left()");
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
		log::debug!("<<< Exiting stack_push_left()");
        }
    }
}

impl<'a, T> Iterator for BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.stack.is_empty() {
			log::debug!(">>> Entering next()");
			log::debug!("<<< Exiting next()");
            None
        } else {
            let node = self.stack.pop().unwrap();
			log::debug!("next(): node → (unprintable)");
            if let Some(right_node) = &node.right {
                self.stack.push(right_node.deref());
                self.stack_push_left();
            }
            node.value.as_ref()
        }
    }
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    fn prequel_memes_tree() -> BinarySearchTree<&'static str> {
        let mut tree = BinarySearchTree::new();
		log::debug!(">>> Entering prequel_memes_tree()");
        log::debug!("prequel_memes_tree(): tree → (unprintable)");
        log::debug!("<<< Exiting prequel_memes_tree()");
        tree.insert("hello there");
        tree.insert("general kenobi");
        tree.insert("you are a bold one");
        tree.insert("kill him");
        tree.insert("back away...I will deal with this jedi slime myself");
        tree.insert("your move");
        tree.insert("you fool");
        tree
    }

    #[test]
    fn test_search() {
		log::debug!(">>> Entering test_search()");
        let tree = prequel_memes_tree();
        assert!(tree.search(&"hello there"));
		log::debug!("test_search(): tree → (unprintable)");
        assert!(tree.search(&"you are a bold one"));
        assert!(tree.search(&"general kenobi"));
        assert!(tree.search(&"you fool"));
        assert!(tree.search(&"kill him"));
        assert!(
            !tree.search(&"but i was going to tosche station to pick up some power converters",)
        );
        assert!(!tree.search(&"only a sith deals in absolutes"));
		log::debug!("<<< Exiting test_search()");
        assert!(!tree.search(&"you underestimate my power"));
    }

    #[test]
    fn test_maximum_and_minimum() {
		log::debug!(">>> Entering test_maximum_and_minimum()");
        let tree = prequel_memes_tree();
        assert_eq!(*tree.maximum().unwrap(), "your move");
		log::debug!("test_maximum_and_minimum(): tree → (unprintable)");
        assert_eq!(
            *tree.minimum().unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        let mut tree2: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree2.maximum().is_none());
        assert!(tree2.minimum().is_none());
        tree2.insert(0);
        assert_eq!(*tree2.minimum().unwrap(), 0);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(-5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
		log::debug!("<<< Exiting test_maximum_and_minimum()");
        assert_eq!(*tree2.maximum().unwrap(), 5);
    }

    #[test]
    fn test_floor_and_ceil() {
		log::debug!(">>> Entering test_floor_and_ceil()");
        let tree = prequel_memes_tree();
        assert_eq!(*tree.floor(&"hello there").unwrap(), "hello there");
		log::debug!("test_floor_and_ceil(): tree → (unprintable)");
        assert_eq!(
            *tree
                .floor(&"these are not the droids you're looking for")
                .unwrap(),
            "kill him"
        );
        assert!(tree.floor(&"another death star").is_none());
        assert_eq!(*tree.floor(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.floor(&"but i was going to tasche station").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(
            *tree.floor(&"you underestimate my power").unwrap(),
            "you fool"
        );
        assert_eq!(*tree.floor(&"your new empire").unwrap(), "your move");
        assert_eq!(*tree.ceil(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .ceil(&"these are not the droids you're looking for")
                .unwrap(),
            "you are a bold one"
        );
        assert_eq!(
            *tree.ceil(&"another death star").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(*tree.ceil(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.ceil(&"but i was going to tasche station").unwrap(),
            "general kenobi"
        );
        assert_eq!(
            *tree.ceil(&"you underestimate my power").unwrap(),
            "your move"
        );
		log::debug!("<<< Exiting test_floor_and_ceil()");
        assert!(tree.ceil(&"your new empire").is_none());
    }

    #[test]
    fn test_iterator() {
		log::debug!(">>> Entering test_iterator()");
        let tree = prequel_memes_tree();
        let mut iter = tree.iter();
		log::debug!("test_iterator(): iter → (unprintable)");
        log::debug!("test_iterator(): tree → (unprintable)");
        assert_eq!(
            iter.next().unwrap(),
            &"back away...I will deal with this jedi slime myself"
        );
        assert_eq!(iter.next().unwrap(), &"general kenobi");
        assert_eq!(iter.next().unwrap(), &"hello there");
        assert_eq!(iter.next().unwrap(), &"kill him");
        assert_eq!(iter.next().unwrap(), &"you are a bold one");
        assert_eq!(iter.next().unwrap(), &"you fool");
        assert_eq!(iter.next().unwrap(), &"your move");
        assert_eq!(iter.next(), None);
		log::debug!("<<< Exiting test_iterator()");
        assert_eq!(iter.next(), None);
    }
}
