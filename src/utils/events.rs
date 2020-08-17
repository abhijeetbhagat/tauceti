extern crate serde;
use super::{reader_enums::DocType, storage_enums::StorageType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// All the events in the system
pub enum TaucetiEvent {
    /// When an upload is done and it needs to be processed
    UploadEvent(StorageType, DocType, String, u32),
    /// When a search term is entered
    SearchEvent(String),
}
