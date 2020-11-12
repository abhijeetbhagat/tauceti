use super::fake_source::FakeSource;
use super::pdf_reader::PdfReader;
use super::reader::DocReader;
use super::{text_file_reader::TextFileReader, word_reader::WordReader};
use crate::utils::reader_enums::DocType;
use async_std::path::PathBuf;

/// Factory method to create a concrete doc reader depending on the doc type
pub fn create_doc_reader(doc_type: &DocType, path: PathBuf) -> Box<dyn DocReader> {
    match doc_type {
        DocType::Text => Box::new(TextFileReader::new(path)),
        DocType::PDF => Box::new(PdfReader::new(path)),
        DocType::Word => Box::new(WordReader::new(path)),
        DocType::Raw => Box::new(FakeSource::new()),
    }
}
