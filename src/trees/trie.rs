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
/// Supported operations - `insert`, `get`.
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
                //[i] = Some(Node::new());
            }
            node.alphabets.as_mut().unwrap()[i] = Some(Node::new());
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
}

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
    assert!(trie.is_present("java"));
}
