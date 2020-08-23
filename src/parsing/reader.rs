use async_std::path::PathBuf;
use async_trait::async_trait;

#[async_trait]
pub trait DocReader: Send {
    /// Parse data from the temp path and return the contents as string
    async fn parse(&mut self) -> Result<String, std::io::Error>;
}
