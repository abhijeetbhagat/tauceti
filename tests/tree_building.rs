#[cfg(test)]
mod tests {
    extern crate tauceti;
    extern crate relative_path;
    use tauceti::{
        parsing::word_reader::WordReader,
        parsing::{pdf_reader::PdfReader, reader::DocReader, text_file_reader::TextFileReader},
        storage::{file_system_interface::FileSystemInterface, storage_interface::Storage},
        trees::index_tree::IndexTree,
        tasks::build_task::build
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

    #[async_std::test]
    async fn test_buiding_with_pdf() -> std::io::Result<()> {
        let path = Path::new("test-resume.pdf");
        assert!(path.exists().await);
        let mut fs = FileSystemInterface::new(path);
        let mut reader = PdfReader::new(path);
        build_helper(&mut fs, &mut reader, 3).await?;
        Ok(())
    }

    async fn build_helper<S: Storage + ?Sized, R: DocReader + ?Sized>(
        s: &mut S,
        r: &mut R,
        expected: usize,
    ) -> Result<(), std::io::Error> {
        let tree = Arc::new(RwLock::new(IndexTree::new()));
        build(s, r, tree.clone(), 1).await?;
        assert_eq!(tree.read().await.keys(), expected);
        Ok(())
    }
}
