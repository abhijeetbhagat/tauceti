extern crate dotext;
use super::reader::DocReader;
use crate::parsing::word_reader::dotext::MsDoc;
use async_std::path::PathBuf;
use async_trait::async_trait;
use dotext::Docx;
use std::io::Read;

/// A Word file reader
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
        let mut file = Docx::open(&self.path).unwrap();
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);
        Ok(content)
    }
}

#[async_std::test]
async fn test_doc_reading() -> std::io::Result<()> {
    let mut word_reader = WordReader::new("test-resume.docx");
    let content = word_reader.parse("".into()).await.unwrap();
    assert_eq!(content, "\n\ncpp python irrelevant java");
    Ok(())
}
