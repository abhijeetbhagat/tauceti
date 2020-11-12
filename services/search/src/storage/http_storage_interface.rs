use super::storage_interface::Storage;
use async_std::path::PathBuf;
use async_trait::async_trait;
use http::Uri;
/// Represents an interface over HTTP file download
struct HttpInterface {
    uri: Uri,
    temp_path: PathBuf,
}

impl HttpInterface {
    /// Creates a new HTTP interface
    ///
    /// Downloads the file over HTTP and stores it in a temp path
    pub fn new(uri: Uri, temp_path: PathBuf) -> HttpInterface {
        HttpInterface { uri, temp_path }
    }
}

#[async_trait]
impl Storage for HttpInterface {
    async fn get(&mut self) -> Result<async_std::path::PathBuf, std::io::Error> {
        todo!()
    }
}
