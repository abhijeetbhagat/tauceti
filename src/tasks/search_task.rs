use super::task::TaucetiTask;
use crate::trees::index_tree::IndexTree;
use async_std::sync::{Arc, RwLock};

struct SearchTask {}

impl SearchTask {
    fn new() -> SearchTask {
        SearchTask {}
    }
}

impl TaucetiTask for SearchTask {
    fn execute() -> Result<(), std::io::Error> {
        todo!()
    }
}

pub fn filter(query: String) -> Vec<String> {
    todo!()
}

pub async fn search(
    index_tree: Arc<RwLock<IndexTree<String, u32>>>,
    terms: &[&str],
) -> Result<Option<Vec<u32>>, std::io::Error> {
    let guard = index_tree.read().await;
    Ok(guard.query(terms))
}
