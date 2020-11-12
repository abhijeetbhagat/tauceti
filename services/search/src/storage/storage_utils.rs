use super::{
    dummy_storage_interface::DummyStorageInterface, file_system_interface::FileSystemInterface,
    storage_interface::Storage,
};
use crate::utils::storage_enums::StorageType;

/// Factory method to create a concrete storage interface depending on the doc type
pub fn create_storage_interface(storage_type: &StorageType, uri: String) -> Box<dyn Storage> {
    match storage_type {
        StorageType::FileSystem => Box::new(FileSystemInterface::new(uri)),
        _ => Box::new(DummyStorageInterface::new()),
    }
}
