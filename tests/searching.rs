extern crate tauceti;

use async_std::sync::{Arc, RwLock};
use tauceti::{tasks::search_task::search, trees::index_tree::IndexTree};

#[async_std::test]
async fn test_searching() -> std::io::Result<()> {
    let tree = Arc::new(RwLock::new(IndexTree::new()));
    {
        let mut guard = tree.write().await;
        guard.insert("cpp".into(), 1);
        guard.insert("cpp".into(), 2);
        guard.insert("python".into(), 1);
        guard.insert("java".into(), 3);
    }

    assert_eq!(
        search(tree.clone(), "cpp and python".into())
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(search(tree, "cpp or java".into()).await.unwrap().len(), 3);
    Ok(())
}
