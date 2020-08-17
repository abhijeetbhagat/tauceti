use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub struct IndexTree<K, V> {
    map: HashMap<K, Vec<V>>,
}

impl<K, V> IndexTree<K, V> {
    /// Creates a new `IndexTree`
    pub fn new() -> Self {
        IndexTree {
            map: HashMap::new(),
        }
    }

    /// Inserts a new key-value pair in tree.
    ///
    /// Appends the value if the key is already present.
    pub fn insert(&mut self, key: K, val: V)
    where
        K: Eq + Hash + Clone,
    {
        let v = self.map.entry(key).or_insert(vec![]);
        v.push(val);
    }

    /// Finds intersection of all the arrays
    pub fn intersection<U>(&self, ids: &[U]) -> Vec<V>
    where
        V: Eq + Hash + Copy,
        U: AsRef<[V]>,
    {
        let mut set = HashSet::new();
        for array in ids {
            for id in array.as_ref() {
                set.insert(*id);
            }
        }

        set.into_iter().collect()
    }

    /// Returns the length of the keys in the tree
    fn keys(&self) -> usize {
        self.map.keys().len()
    }

    /// Gets the list of indices (value) for a term (key)
    fn get(&self, k: &K) -> Option<&Vec<V>>
    where
        K: Eq + Hash,
    {
        self.map.get(k)
    }

    /// Performs a boolean query (intersection) on the terms
    ///
    /// to find the common indices containing all the terms
    pub fn query(&self, terms: &[K]) -> Vec<V>
    where
        V: Eq + Hash + Copy,
        K: Eq + Hash,
    {
        let mut id_collection = vec![];
        for term in terms {
            id_collection.push(self.get(term).unwrap());
        }
        self.intersection(id_collection.as_slice())
    }
}

#[test]
fn test_it_ops() {
    let mut it = IndexTree::new();
    it.insert("c++", 1);
    it.insert("c++", 2);
    it.insert("python", 1);
    it.insert("java", 3);
    assert_eq!(it.keys(), 3);
    assert_eq!(it.get(&"c++"), Some(&vec![1, 2]));
    assert_eq!(
        it.intersection(&[&[1u32, 1, 2, 3] as &[_], &[1u32, 2, 3]])
            .len(),
        3
    );
    assert_eq!(it.query(&["c++"]).len(), 2);
    assert_eq!(it.query(&["c++", "python"]).len(), 2);
    assert_eq!(it.query(&["c++", "python", "java"]).len(), 3);
}
