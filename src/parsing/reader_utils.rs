use super::fake_source::FakeSource;
use super::pdf_reader::PdfReader;
use super::reader::DocReader;
use super::{text_file_reader::TextFileReader, word_reader::WordReader};
use crate::utils::reader_enums::DocType;

/// Factory method to create a concrete doc reader depending on the doc type
pub fn create_doc_reader(doc_type: &DocType, uri: String) -> Box<dyn DocReader> {
    match doc_type {
        DocType::Text => Box::new(TextFileReader::new(uri)),
        DocType::PDF => Box::new(PdfReader::new(uri)),
        DocType::Word => Box::new(WordReader::new(uri)),
        DocType::Raw => Box::new(FakeSource::new()),
    }
}
