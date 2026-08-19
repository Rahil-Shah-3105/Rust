use std::collections::LinkedList;

pub struct HashTable<K, V> {
    elements: Vec<LinkedList<(K, V)>>,
    count: usize,
}

impl<K: Hashable + std::cmp::PartialEq, V> Default for HashTable<K, V> {
    fn default() -> Self {
		log::debug!(">>> Entering default()");
        log::debug!("<<< Exiting default()");
        Self::new()
    }
}

pub trait Hashable {
    fn hash(&self) -> usize;
}

impl<K: Hashable + std::cmp::PartialEq, V> HashTable<K, V> {
    pub fn new() -> HashTable<K, V> {
		log::debug!(">>> Entering new()");
        let initial_capacity = 3000;
        let mut elements = Vec::with_capacity(initial_capacity);
		log::debug!("new(): elements → (unprintable)");

        for _ in 0..initial_capacity {
            elements.push(LinkedList::new());
        }

		log::debug!("<<< Exiting new()");
        HashTable { elements, count: 0 }
    }

    pub fn insert(&mut self, key: K, value: V) {
		log::debug!(">>> Entering insert(key=(unprintable),value=(unprintable))");
        if self.count >= self.elements.len() * 3 / 4 {
            self.resize();
        }
        let index = key.hash() % self.elements.len();
        self.elements[index].push_back((key, value));
		log::debug!("<<< Exiting insert(key=(unprintable),value=(unprintable))");
        self.count += 1;
    }

    pub fn search(&self, key: K) -> Option<&V> {
		log::debug!(">>> Entering search(key=(unprintable))");
        let index = key.hash() % self.elements.len();
		log::debug!("<<< Exiting search(key=(unprintable))");
        self.elements[index]
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v)
    }

    fn resize(&mut self) {
		log::debug!(">>> Entering resize()");
        let new_size = self.elements.len() * 2;
        let mut new_elements = Vec::with_capacity(new_size);
		log::debug!("resize(): new_elements → (unprintable)");
		log::debug!("resize(): new_size → (unprintable)");

        for _ in 0..new_size {
            new_elements.push(LinkedList::new());
        }

        for old_list in self.elements.drain(..) {
            for (key, value) in old_list {
                let new_index = key.hash() % new_size;
				log::debug!("resize(): new_index → (unprintable)");
                new_elements[new_index].push_back((key, value));
            }
        }

		log::debug!("resize(): self.elements → (unprintable)");
    log::debug!("<<< Exiting resize()");
        self.elements = new_elements;
	log::debug!(">>> Entering hash()");
            log::debug!("<<< Exiting hash()");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TestKey(usize);

    impl Hashable for TestKey {
        fn hash(&self) -> usize {
			log::debug!(">>> Entering hash()");
            log::debug!("<<< Exiting hash()");
            self.0
        }
    }

    #[test]
    fn test_insert_and_search() {
		log::debug!(">>> Entering test_insert_and_search()");
        let mut hash_table = HashTable::new();
        let key = TestKey(1);
		log::warn!("test_insert_and_search(): key → (unprintable)");
		log::debug!("test_insert_and_search(): value → (unprintable)");
		log::debug!("test_insert_and_search(): hash_table → (unprintable)");
        let value = TestKey(10);

        hash_table.insert(key, value);
        let result = hash_table.search(TestKey(1));
		log::debug!("test_insert_and_search(): result → (unprintable)");

		log::debug!("<<< Exiting test_insert_and_search()");
        assert_eq!(result, Some(&TestKey(10)));
    }

    #[test]
    fn test_resize() {
		log::debug!(">>> Entering test_resize()");
        let mut hash_table = HashTable::new();
        let initial_capacity = hash_table.elements.capacity();
		log::debug!("test_resize(): initial_capacity → (unprintable)");
		log::debug!("test_resize(): hash_table → (unprintable)");

        for i in 0..=initial_capacity * 3 / 4 {
            hash_table.insert(TestKey(i), TestKey(i + 10));
        }

		log::debug!("<<< Exiting test_resize()");
        assert!(hash_table.elements.capacity() > initial_capacity);
    }

    #[test]
    fn test_search_nonexistent() {
		log::debug!(">>> Entering test_search_nonexistent()");
        let mut hash_table = HashTable::new();
        let key = TestKey(1);
		log::warn!("test_search_nonexistent(): key → (unprintable)");
		log::debug!("test_search_nonexistent(): value → (unprintable)");
		log::debug!("test_search_nonexistent(): hash_table → (unprintable)");
        let value = TestKey(10);

        hash_table.insert(key, value);
        let result = hash_table.search(TestKey(2));
		log::debug!("test_search_nonexistent(): result → (unprintable)");

		log::debug!("<<< Exiting test_search_nonexistent()");
        assert_eq!(result, None);
    }

    #[test]
    fn test_multiple_inserts_and_searches() {
		log::debug!(">>> Entering test_multiple_inserts_and_searches()");
        let mut hash_table = HashTable::new();
        for i in 0..10 {
			log::debug!("test_multiple_inserts_and_searches(): hash_table → (unprintable)");
            hash_table.insert(TestKey(i), TestKey(i + 100));
        }

        for i in 0..10 {
            let result = hash_table.search(TestKey(i));
			log::debug!("test_multiple_inserts_and_searches(): result → (unprintable)");
            assert_eq!(result, Some(&TestKey(i + 100)));
		log::debug!("<<< Exiting test_multiple_inserts_and_searches()");
        }
    }

    #[test]
    fn test_not_overwrite_existing_key() {
		log::debug!(">>> Entering test_not_overwrite_existing_key()");
        let mut hash_table = HashTable::new();
        hash_table.insert(TestKey(1), TestKey(100));
		log::debug!("test_not_overwrite_existing_key(): hash_table → (unprintable)");
        hash_table.insert(TestKey(1), TestKey(200));

        let result = hash_table.search(TestKey(1));
		log::debug!("test_not_overwrite_existing_key(): result → (unprintable)");
		log::debug!("<<< Exiting test_not_overwrite_existing_key()");
        assert_eq!(result, Some(&TestKey(100)));
    }

    #[test]
    fn test_empty_search() {
		log::debug!(">>> Entering test_empty_search()");
        let hash_table: HashTable<TestKey, TestKey> = HashTable::new();
        let result = hash_table.search(TestKey(1));
		log::debug!("test_empty_search(): result → (unprintable)");

		log::debug!("<<< Exiting test_empty_search()");
        assert_eq!(result, None);
    }
}
