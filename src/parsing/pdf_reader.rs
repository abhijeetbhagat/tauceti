use super::reader::DocReader;
use async_std::path::PathBuf;
use async_trait::async_trait;

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
        todo!()
    }
}
