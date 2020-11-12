use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// Type of storage - file system, object storage, etc.
pub enum StorageType {
    FileSystem,
    ObjectStorageSystem,
    Fake,
}
