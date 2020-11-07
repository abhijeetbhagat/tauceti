#[cfg(test)]
mod tests {
    extern crate relative_path;
    extern crate tauceti;
    use async_std::sync::{Arc, RwLock};
    use tauceti::{
        tasks::build_task::build,
        trees::index_tree::IndexTree,
        utils::{reader_enums::DocType, storage_enums::StorageType},
    };

    #[async_std::test]
    #[cfg(target_os = "windows")]
    async fn test_buiding_with_text_file() -> std::io::Result<()> {
        build_helper(
            StorageType::FileSystem,
            "test-resume".into(),
            1,
            DocType::Word,
            3,
        )
        .await?;
        Ok(())
    }

    #[async_std::test]
    async fn test_buiding_with_word_docx() -> std::io::Result<()> {
        build_helper(
            StorageType::FileSystem,
            "test-resume.docx".into(),
            1,
            DocType::Word,
            3,
        )
        .await?;
        Ok(())
    }

    #[async_std::test]
    async fn test_buiding_with_pdf() -> std::io::Result<()> {
        build_helper(
            StorageType::FileSystem,
            "test-resume.pdf".into(),
            1,
            DocType::PDF,
            3,
        )
        .await?;
        Ok(())
    }

    async fn build_helper(
        storage_type: StorageType,
        uri: String,
        doc_id: u32,
        doc_type: DocType,
        expected: usize,
    ) -> Result<(), std::io::Error> {
        let tree = Arc::new(RwLock::new(IndexTree::new()));
        build(storage_type, uri, tree.clone(), doc_id, doc_type).await?;
        assert_eq!(tree.read().await.keys(), expected);
        Ok(())
    }
}
