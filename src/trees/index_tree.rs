use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub struct IndexTree<K, V> {
    map: HashMap<K, HashSet<V>>,
}

impl<K, V> IndexTree<K, V>
where
    K: Eq + Hash,
{
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
        V: Eq + Hash,
    {
        self.map.entry(key).or_insert_with(HashSet::new).insert(val);
    }

    /*
    /// Finds intersection of all the arrays
    pub fn intersection<U>(&self, ids: &[U]) -> HashSet<V>
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

        set
    }
    */

    /// Returns the length of the keys in the tree
    pub fn keys(&self) -> usize {
        self.map.keys().len()
    }

    /// Gets the list of indices (value) for a term (key)
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&HashSet<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        self.map.get(k)
    }

    /*
    /// Performs a boolean query (intersection) on the terms
    ///
    /// to find the common indices containing all the terms
    pub fn query<Q: ?Sized>(&self, terms: &[&Q]) -> Option<HashSet<V>>
    where
        V: Eq + Hash + Copy,
        Q: Eq + Hash,
        K: Borrow<Q>,
    {
        let mut id_collection = HashSet::new();
        for term in terms {
            let result = self.get(term);
            if result.is_none() {
                continue;
            }
            id_collection.extend(result.unwrap());
        }

        if id_collection.is_empty() {
            return None;
        }

        Some(self.intersection(id_collection))
    }
    */
}

impl<K, V> Default for IndexTree<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        IndexTree::new()
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
    assert_eq!(it.get(&"c++").unwrap().len(), 2);
}
