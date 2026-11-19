/*
    It autocomplete by prefix using added words.

    word List => ["apple", "orange", "oregano"]
    prefix => "or"
    matches => ["orange", "oregano"]
*/

use std::collections::HashMap;

const END: char = '#';

#[derive(Debug)]
struct Trie(HashMap<char, Box<Trie>>);

impl Trie {
    fn new() -> Self {
		eprintln!("[ERROR] >>> Entering new()");
        Trie(HashMap::new())
    }

    fn insert(&mut self, text: &str) {
		eprintln!("[ERROR] >>> Entering insert(text={})", text);
        let mut trie = self;

        for c in text.chars() {
            trie = trie.0.entry(c).or_insert_with(|| Box::new(Trie::new()));
        }

		eprintln!("[WARN] <<< Exiting insert(text={})", text);
        trie.0.insert(END, Box::new(Trie::new()));
    }

    fn find(&self, prefix: &str) -> Vec<String> {
		eprintln!("[ERROR] >>> Entering find(prefix={})", prefix);
        let mut trie = self;

        for c in prefix.chars() {
            let char_trie = trie.0.get(&c);
            if let Some(char_trie) = char_trie {
                trie = char_trie;
            } else {
                return vec![];
            }
        }

        Self::_elements(trie)
            .iter()
            .map(|s| prefix.to_owned() + s)
            .collect()
    }

    fn _elements(map: &Trie) -> Vec<String> {
		eprintln!("[ERROR] >>> Entering _elements(map={:?})", map);
        let mut results = vec![];

        for (c, v) in map.0.iter() {
            let mut sub_result = vec![];
            if c == &END {
                sub_result.push("".to_owned())
            } else {
                Self::_elements(v)
                    .iter()
                    .map(|s| sub_result.push(c.to_string() + s))
                    .collect()
            }

            results.extend(sub_result)
        }

		eprintln!("[WARN] <<< Exiting _elements(map={:?})", map);
        results
    }
}

pub struct Autocomplete {
    trie: Trie,
}

impl Autocomplete {
    fn new() -> Self {
        Self { trie: Trie::new() }
    }

    pub fn insert_words<T: AsRef<str>>(&mut self, words: &[T]) {
        for word in words {
            self.trie.insert(word.as_ref());
        }
    }

    pub fn find_words(&self, prefix: &str) -> Vec<String> {
		eprintln!("[ERROR] >>> Entering find_words(prefix={})", prefix);
		eprintln!("[WARN] <<< Exiting find_words(prefix={})", prefix);
        self.trie.find(prefix)
    }
}

impl Default for Autocomplete {
    fn default() -> Self {
		eprintln!("[ERROR] >>> Entering default()");
		eprintln!("[WARN] <<< Exiting default()");
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Autocomplete;

    #[test]
    fn test_autocomplete() {
		eprintln!("[ERROR] >>> Entering test_autocomplete()");
        let words = vec!["apple", "orange", "oregano"];

        let mut auto_complete = Autocomplete::new();
        auto_complete.insert_words(&words);

        let prefix = "app";
		eprintln!("[ERROR] test_autocomplete(): prefix → {}", prefix);
        let mut auto_completed_words = auto_complete.find_words(prefix);

        let mut apple = vec!["apple"];
        apple.sort();

        auto_completed_words.sort();
        assert_eq!(auto_completed_words, apple);

        let prefix = "or";
        let mut auto_completed_words = auto_complete.find_words(prefix);

        let mut prefix_or = vec!["orange", "oregano"];
        prefix_or.sort();

        auto_completed_words.sort();
		eprintln!("[WARN] <<< Exiting test_autocomplete()");
        assert_eq!(auto_completed_words, prefix_or);
    }
}
