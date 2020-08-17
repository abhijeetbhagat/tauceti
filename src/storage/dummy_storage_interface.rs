use super::storage_interface::Storage;
use async_std::path::{Path, PathBuf};
use async_trait::async_trait;

pub struct DummyStorageInterface {}

impl DummyStorageInterface {
    pub fn new() -> DummyStorageInterface {
        DummyStorageInterface {}
    }
}

#[async_trait]
impl Storage for DummyStorageInterface {
    async fn get(&mut self) -> Result<PathBuf, std::io::Error> {
        todo!()
    }
}
