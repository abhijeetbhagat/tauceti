extern crate lopdf;
use super::reader::DocReader;
use async_std::path::PathBuf;
use async_trait::async_trait;
use lopdf::Document;

/// A PDF file reader
pub struct PdfReader {
    path: PathBuf,
}

impl PdfReader {
    pub fn new<P: Into<PathBuf>>(path: P) -> PdfReader {
        PdfReader { path: path.into() }
    }
}

#[async_trait]
impl DocReader for PdfReader {
    async fn parse(&mut self, _: async_std::path::PathBuf) -> Result<String, std::io::Error> {
        let doc = Document::load(&self.path).unwrap();
        let pages: Vec<u32> = doc.get_pages().keys().cloned().collect();
        let result = doc.extract_text(&pages).unwrap();
        Ok(result)
    }
}
