use super::reader::DocReader;
use crate::async_std::io::ReadExt;
use async_std::{fs::File, io::BufReader, path::PathBuf};
use async_trait::async_trait;

/// A text file reader.
pub struct TextFileReader {
    path: PathBuf,
}

impl TextFileReader {
    pub fn new<P: Into<PathBuf>>(path: P) -> TextFileReader {
        TextFileReader { path: path.into() }
    }
}

#[async_trait]
impl DocReader for TextFileReader {
    async fn parse(&mut self, path: PathBuf) -> Result<String, std::io::Error> {
        let mut text = String::new();
        let file = File::open(&self.path).await?;
        let mut reader = BufReader::new(file);
        let mut buf = [0u8; 4096];
        loop {
            let size = reader.read(&mut buf).await?;
            if size == 0 {
                break;
            }
            text.push_str(std::str::from_utf8(&buf[..size]).unwrap());
        }

        Ok(text)
    }
}
