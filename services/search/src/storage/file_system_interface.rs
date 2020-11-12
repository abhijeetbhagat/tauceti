use super::storage_interface::Storage;
use async_std::path::PathBuf;
use async_trait::async_trait;

/// A file system interface.
pub struct FileSystemInterface {
    path: PathBuf,
}

impl FileSystemInterface {
    /// Creates an file system
    pub fn new<P: Into<PathBuf>>(path: P) -> FileSystemInterface {
        FileSystemInterface { path: path.into() }
    }
}

#[async_trait]
impl Storage for FileSystemInterface {
    async fn get(&mut self) -> Result<PathBuf, std::io::Error> {
        Ok(self.path.clone())
    }
}
