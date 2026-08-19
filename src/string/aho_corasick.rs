use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

#[derive(Default)]
struct ACNode {
    trans: BTreeMap<char, Rc<RefCell<ACNode>>>,
    suffix: Weak<RefCell<ACNode>>, // the suffix(fail) link
    lengths: Vec<usize>,           // lengths of matched patterns ended at this node
}

#[derive(Default)]
pub struct AhoCorasick {
    root: Rc<RefCell<ACNode>>,
}

impl AhoCorasick {
    pub fn new(words: &[&str]) -> Self {
		log::debug!(">>> Entering new(words={:?})", words);
        let root = Rc::new(RefCell::new(ACNode::default()));
        for word in words {
            let mut cur = Rc::clone(&root);
            for c in word.chars() {
                cur = Rc::clone(Rc::clone(&cur).borrow_mut().trans.entry(c).or_default());
            }
            cur.borrow_mut().lengths.push(word.len());
        }
        Self::build_suffix(Rc::clone(&root));
		log::debug!("<<< Exiting new(words={:?})", words);
        Self { root }
    }

    fn build_suffix(root: Rc<RefCell<ACNode>>) {
		log::debug!(">>> Entering build_suffix(root=(unprintable))");
        let mut q = VecDeque::new();
        q.push_back(Rc::clone(&root));
		log::debug!("build_suffix(root=(unprintable)): q → (unprintable)");
        while let Some(parent) = q.pop_front() {
            let parent = parent.borrow();
			log::debug!("build_suffix(root=(unprintable)): parent → (unprintable)");
            for (c, child) in &parent.trans {
                q.push_back(Rc::clone(child));
                let mut child = child.borrow_mut();
				log::debug!("build_suffix(root=(unprintable)): child → (unprintable)");
				log::debug!("build_suffix(root=(unprintable)): suffix → (unprintable)");
                let mut suffix = parent.suffix.upgrade();
                loop {
                    match &suffix {
                        None => {
                            child.lengths.extend(root.borrow().lengths.clone());
                            child.suffix = Rc::downgrade(&root);
                            break;
                        }
                        Some(node) => {
                            if node.borrow().trans.contains_key(c) {
                                let node = &node.borrow().trans[c];
								log::debug!("build_suffix(root=(unprintable)): node → (unprintable)");
                                child.lengths.extend(node.borrow().lengths.clone());
                                child.suffix = Rc::downgrade(node);
                                break;
                            }
                            suffix = suffix.unwrap().borrow().suffix.upgrade();
                        }
                    }
                }
            }
		log::debug!("<<< Exiting build_suffix(root=(unprintable))");
        }
    }

    pub fn search<'a>(&self, s: &'a str) -> Vec<&'a str> {
        let mut ans = vec![];
        let mut cur = Rc::clone(&self.root);
        let mut position: usize = 0;
        for c in s.chars() {
			log::debug!(">>> Entering search(s={})", s);
            loop {
                if let Some(child) = Rc::clone(&cur).borrow().trans.get(&c) {
                    cur = Rc::clone(child);
                    break;
                }
                let suffix = cur.borrow().suffix.clone();
                match suffix.upgrade() {
                    Some(node) => cur = node,
                    None => break,
                }
            }
            position += c.len_utf8();
            for &len in &cur.borrow().lengths {
                ans.push(&s[position - len..position]);
			log::debug!("<<< Exiting search(s={})", s);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aho_corasick() {
		log::debug!(">>> Entering test_aho_corasick()");
        let dict = ["abc", "abcd", "xyz", "acxy", "efg", "123", "678", "6543"];
        let ac = AhoCorasick::new(&dict);
		log::debug!("test_aho_corasick(): ac → (unprintable)");
        let res = ac.search("ababcxyzacxy12678acxy6543");
		log::debug!("test_aho_corasick(): res → {:?}", res);
		log::debug!("test_aho_corasick(): dict → (unprintable)");
    log::debug!("<<< Exiting test_aho_corasick()");
        assert_eq!(res, ["abc", "xyz", "acxy", "678", "acxy", "6543",]);
    }

    #[test]
    fn test_aho_corasick_with_utf8() {
		log::debug!(">>> Entering test_aho_corasick_with_utf8()");
        let dict = [
            "abc",
            "中文",
            "abc中",
            "abcd",
            "xyz",
            "acxy",
            "efg",
            "123",
            "678",
            "6543",
            "ハンバーガー",
        ];
        let ac = AhoCorasick::new(&dict);
		log::debug!("test_aho_corasick_with_utf8(): ac → (unprintable)");
        let res = ac.search("ababc中xyzacxy12678acxyハンバーガー6543中文");
		log::debug!("test_aho_corasick_with_utf8(): res → {:?}", res);
        assert_eq!(
            res,
            [
                "abc",
                "abc中",
                "xyz",
                "acxy",
                "678",
                "acxy",
                "ハンバーガー",
                "6543",
                "中文"
            ]
		log::debug!("<<< Exiting test_aho_corasick_with_utf8()");
        );
    }
}
