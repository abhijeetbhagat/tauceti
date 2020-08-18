use crate::parsing::{parser::Parser, reader::DocReader};
use crate::storage::storage_interface::Storage;
use crate::trees::index_tree::IndexTree;
use async_std::sync::{Arc, RwLock};

struct UploadTask {}

impl UploadTask {
    fn new() -> UploadTask {
        UploadTask {}
    }
}

pub async fn build<S: Storage + ?Sized, R: DocReader + ?Sized>(
    storage: &mut S,
    reader: &mut R,
    index_tree: Arc<RwLock<IndexTree<String, u32>>>,
    doc_id: u32,
) -> Result<(), std::io::Error> {
    let local_path = storage.get().await?;
    let content = reader.parse(local_path).await?;
    let mut parser = Parser::new(&content, index_tree, doc_id);
    parser.parse().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate relative_path;
    use crate::{
        parsing::word_reader::WordReader,
        parsing::{reader::DocReader, text_file_reader::TextFileReader},
        storage::{file_system_interface::FileSystemInterface, storage_interface::Storage},
        trees::index_tree::IndexTree,
    };
    use async_std::{
        path::Path,
        sync::{Arc, RwLock},
    };
    use relative_path::RelativePath;

    #[async_std::test]
    #[cfg(target_os = "windows")]
    async fn test_buiding_with_text_file() -> std::io::Result<()> {
        let path = Path::new("test-resume");
        assert!(path.exists().await);
        let mut fs = FileSystemInterface::new(path);
        let mut reader = TextFileReader::new(path);
        build_helper(&mut fs, &mut reader, 3).await?;
        Ok(())
    }

    #[async_std::test]
    async fn test_buiding_with_word_docx() -> std::io::Result<()> {
        let path = Path::new("test-resume.docx");
        assert!(path.exists().await);
        let mut fs = FileSystemInterface::new(path);
        let mut reader = WordReader::new(path);
        build_helper(&mut fs, &mut reader, 3).await?;
        Ok(())
    }

    async fn build_helper<S: Storage + ?Sized, R: DocReader + ?Sized>(
        s: &mut S,
        r: &mut R,
        expected: usize,
    ) -> Result<(), std::io::Error> {
        let tree = Arc::new(RwLock::new(IndexTree::new()));
        super::build(s, r, tree.clone(), 1).await?;
        assert_eq!(tree.read().await.keys(), expected);
        Ok(())
    }
}
