use crate::parsing::{parser::Parser, reader::DocReader};
use crate::storage::storage_interface::Storage;
use crate::trees::index_tree::IndexTree;
use std::sync::{Arc, Mutex};

struct UploadTask {}

impl UploadTask {
    fn new() -> UploadTask {
        UploadTask {}
    }
}

pub async fn build<S: Storage + ?Sized, R: DocReader + ?Sized>(
    storage: &mut S,
    reader: &mut R,
    index_tree: Arc<Mutex<IndexTree<String, u32>>>,
    doc_id: u32,
) -> Result<(), std::io::Error> {
    let local_path = storage.get().await?;
    let content = reader.parse(local_path).await?;
    let mut parser = Parser::new(&content, index_tree, doc_id);
    parser.parse();
    Ok(())
}

#[async_std::test]
async fn test_buiding() -> std::io::Result<()> {
    Ok(())
}
