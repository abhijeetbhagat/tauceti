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
    index_tree: Arc<RwLock<IndexTree<String, u32>>>,
    terms: &[&str],
) -> Result<Vec<u32>, std::io::Error> {
    let guard = index_tree.read().await;
    let result: Vec<u32> = guard.query(terms);
    Ok(result)
}

#[async_std::test]
async fn test_searching() -> std::io::Result<()> {
    let tree = Arc::new(RwLock::new(IndexTree::new()));
    {
        let mut guard = tree.write().await;
        guard.insert("c++".into(), 1);
        guard.insert("c++".into(), 2);
        guard.insert("python".into(), 1);
        guard.insert("java".into(), 3);
    }

    assert_eq!(search(tree, &["c++"]).await.unwrap().len(), 2);
    Ok(())
}
