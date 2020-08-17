extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// Type of storage - file system, object storage, etc.
pub enum StorageType {
    FileSystem,
    ObjectStorageSystem,
    Fake,
}

/// Type of document
#[derive(Serialize, Deserialize, Debug)]
pub enum DocType {
    Text,
    PDF,
    Word,
    Raw,
}

#[derive(Serialize, Deserialize, Debug)]
/// All the events in the system
pub enum TaucetiEvent {
    /// When an upload is done and it needs to be processed
    UploadEvent(StorageType, DocType, String, u32),
    /// When a search term is entered
    SearchEvent(String),
}
