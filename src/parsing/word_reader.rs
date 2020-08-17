use super::reader::DocReader;
use async_std::path::PathBuf;
use async_trait::async_trait;

/// A PDF file reader
pub struct WordReader {
    path: PathBuf,
}

impl WordReader {
    pub fn new<P: Into<PathBuf>>(path: P) -> WordReader {
        WordReader { path: path.into() }
    }
}

#[async_trait]
impl DocReader for WordReader {
    async fn parse(&mut self, _: async_std::path::PathBuf) -> Result<String, std::io::Error> {
        todo!()
    }
}
