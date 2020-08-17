use super::task::TaucetiTask;
use crate::trees::index_tree::IndexTree;
use async_std::sync::RwLock;
use std::sync::Arc;

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

pub async fn search(
    index_tree: Arc<RwLock<IndexTree<String>>>,
    doc_id: u32,
) -> Result<Vec<u32>, std::io::Error> {
    let guard = index_tree.read().await;
    //let result: Vec<u32> = guard.query();
    //Ok(result)
    todo!()
}

#[async_std::test]
async fn test_searching() -> std::io::Result<()> {
    Ok(())
}
