use crate::parsing::{parser::Parser, reader_utils::create_doc_reader};
use crate::storage::{storage_interface::Storage, storage_utils::create_storage_interface};
use crate::{
    trees::index_tree::IndexTree,
    utils::{reader_enums::DocType, storage_enums::StorageType},
};
use async_std::sync::{Arc, RwLock};

struct UploadTask {}

impl UploadTask {
    fn new() -> UploadTask {
        UploadTask {}
    }
}

/// Fetches a document from the storage, creates a document reader
///
/// that parses the document and builds the index-tree.
pub async fn build(
    storage_type: StorageType,
    uri: String,
    index_tree: Arc<RwLock<IndexTree<String, u32>>>,
    doc_id: u32,
    doc_type: DocType,
) -> Result<(), std::io::Error> {
    let mut storage = create_storage_interface(&storage_type, uri);
    let local_path = storage.get().await?;
    let mut reader = create_doc_reader(&doc_type, local_path);
    let content = reader.parse().await?;
    let mut parser = Parser::new(&content, index_tree, doc_id);
    parser.parse().await;
    Ok(())
}
