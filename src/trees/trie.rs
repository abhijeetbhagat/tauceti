use std::str::Chars;

#[derive(Clone, Debug)]
pub struct Node {
    alphabets: Option<Vec<Option<Node>>>,
    doc_ids: Option<Vec<u32>>,
    is_term: bool,
}

impl Node {
    fn new() -> Node {
        Node {
            alphabets: None,
            doc_ids: None,
            is_term: false,
        }
    }
}

/// Represents a dictionary of words
///
/// Supported operations - `insert`, 'is_present', `get`.
pub struct Trie {
    root: Node,
}

impl Trie {
    /// Creates a new `Trie`
    pub fn new() -> Trie {
        Trie { root: Node::new() }
    }

    /// Inserts a new word in the dict
    ///
    /// Converts the word to lowercase before inserting.
    pub fn insert(&mut self, word: &str) {
        let word = word.to_lowercase();
        let mut word_itr = word.chars();
        Self::insert_internal(word_itr.next(), &mut word_itr, &mut self.root);
    }

    fn insert_internal(c: Option<char>, word_itr: &mut Chars, node: &mut Node) {
        if let Some(c) = c {
            let i = c as usize % 97;
            if node.alphabets.is_none() {
                node.alphabets = Some(vec![None; 26]);
            }

            if node.alphabets.as_ref().unwrap()[i].is_none() {
                node.alphabets.as_mut().unwrap()[i] = Some(Node::new());
            }

            Self::insert_internal(
                word_itr.next(),
                word_itr,
                node.alphabets.as_mut().unwrap()[i].as_mut().unwrap(),
            );
        } else {
            node.is_term = true;
        }
    }

    /// Gets info related to a word from the dictionary if present
    pub fn is_present(&self, word: &str) -> bool {
        let word = word.to_lowercase();
        let mut word_itr = word.chars();
        Self::is_present_internal(word_itr.next(), &mut word_itr, &self.root)
    }

    fn is_present_internal(c: Option<char>, word_itr: &mut Chars, node: &Node) -> bool {
        if let Some(c) = c {
            let c = c as usize % 97;
            if node.alphabets.is_some() && node.alphabets.as_ref().unwrap()[c].is_some() {
                Self::is_present_internal(
                    word_itr.next(),
                    word_itr,
                    node.alphabets.as_ref().unwrap()[c].as_ref().unwrap(),
                )
            } else {
                false
            }
        } else {
            true
        }
    }

    /// Gets a list of word that contain the prefix
    pub fn get(&self, prefix: &str) -> Option<Vec<String>> {
        if self.root.alphabets.is_some() {
            let mut words = vec![];
            let mut word_itr = prefix.chars();
            if let Some(node) = Self::get_next(&self.root, word_itr.next(), &mut word_itr) {
                Self::collect_words(node, &prefix, &mut words);
            }
            Some(words)
        } else {
            None
        }
    }

    fn get_tail<'a>(node: &'a Node, c: Option<char>, word_itr: &mut Chars) -> Option<&'a Node> {
        if let Some(c) = c {
            let c = c as usize % 97;
            if node.alphabets.is_some() && node.alphabets.as_ref().unwrap()[c].is_some() {
                Self::get_tail(
                    node.alphabets.as_ref().unwrap()[c].as_ref().unwrap(),
                    word_itr.next(),
                    word_itr,
                )
            } else {
                None
            }
        } else if node.is_term {
            Some(node)
        } else {
            None
        }
    }

    fn get_next<'a>(node: &'a Node, c: Option<char>, word_itr: &mut Chars) -> Option<&'a Node> {
        if let Some(c) = c {
            let c = c as usize % 97;
            if node.alphabets.is_some() && node.alphabets.as_ref().unwrap()[c].is_some() {
                Self::get_next(
                    node.alphabets.as_ref().unwrap()[c].as_ref().unwrap(),
                    word_itr.next(),
                    word_itr,
                )
            } else {
                None
            }
        } else {
            Some(node)
        }
    }

    fn collect_words(node: &Node, prefix: &str, collection: &mut Vec<String>) {
        //let mut collection = vec![];
        if node.alphabets.is_some() {
            for c in node
                .alphabets
                .as_ref()
                .unwrap()
                .iter()
                .zip(0..)
                .filter(|n| n.0.is_some())
            {
                let ch = ((c.1 as u8 % 97) + 97) as char;

                Self::collect_words(
                    c.0.as_ref().unwrap(),
                    &format!("{}{}", prefix, ch),
                    collection,
                );
                if c.0.as_ref().unwrap().is_term {
                    collection.push(format!("{}{}", prefix, ch));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_trie_store() {
        let mut trie = Trie::new();
        trie.insert("cpp");
        assert!(trie.is_present("cpp"));
    }

    #[test]
    fn test_trie_construct() {
        let mut trie = Trie::new();
        trie.insert("c");
        assert!(trie.root.alphabets.is_some());
        assert!(trie.is_present("c"));
        assert!(!trie.is_present("gandole"));
        trie.insert("cpp");
        trie.insert("python");
        trie.insert("java");
        assert!(trie.is_present("cpp"));
        assert!(trie.is_present("python"));
    }

    #[test]
    fn test_trie_tail_fetching() {
        let mut trie = Trie::new();
        trie.insert("cpp");
        trie.insert("clojure");
        let mut word = "cpp".chars();
        let node = Trie::get_tail(&trie.root, word.next(), &mut word);
        assert!(node.is_some());
        assert!(node.unwrap().is_term);
        let mut word = "cp".chars();
        let node = Trie::get_tail(&trie.root, word.next(), &mut word);
        assert!(node.is_none());
        let mut word = "clojure".chars();
        let node = Trie::get_tail(&trie.root, word.next(), &mut word);
        assert!(node.is_some());
        assert!(node.as_ref().unwrap().alphabets.is_none());
    }

    #[test]
    fn test_trie_next() {
        let mut trie = Trie::new();
        trie.insert("cpp");
        trie.insert("clojure");
        let mut word = "cpp".chars();
        let node = Trie::get_next(&trie.root, word.next(), &mut word);
        assert!(node.is_some());
        assert!(node.unwrap().is_term);
        let mut word = "cp".chars();
        let node = Trie::get_next(&trie.root, word.next(), &mut word);
        assert!(node.is_some());
        let mut word = "cloj".chars();
        let node = Trie::get_next(&trie.root, word.next(), &mut word);
        assert!(node.is_some());
    }

    #[test]
    fn test_trie_words_collection() {
        let mut trie = Trie::new();
        trie.insert("cpp");
        trie.insert("clisp");
        trie.insert("clojure");
        assert!(trie.root.alphabets.is_some());
        let word = String::from("c");
        let mut word_itr = word.chars();
        let mut collection = vec![];
        let node = Trie::get_next(&trie.root, word_itr.next(), &mut word_itr);

        assert_eq!(
            node.as_ref()
                .unwrap()
                .alphabets
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.is_some())
                .count(),
            2
        );

        let _ = Trie::collect_words(node.as_ref().unwrap(), &word, &mut collection);
        assert_eq!(collection.len(), 3);
        assert_eq!(collection[0], String::from("clisp"));
        assert_eq!(collection[1], String::from("clojure"));
        assert_eq!(collection[2], String::from("cpp"));
    }

    #[test]
    fn test_trie_get_words() {
        let mut trie = Trie::new();
        trie.insert("cpp");
        trie.insert("clisp");
        trie.insert("clojure");

        let results = trie.get("c");
        assert!(results.is_some());
        let results = results.unwrap();
        assert_eq!(results[0], "clisp");
        assert_eq!(results[1], "clojure");
        assert_eq!(results[2], "cpp");

        let results = trie.get("cl");
        assert!(results.is_some());
        let results = results.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], "clisp");
        assert_eq!(results[1], "clojure");
    }
}
