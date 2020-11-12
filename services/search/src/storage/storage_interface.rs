use async_std::path::PathBuf;
use async_trait::async_trait;

#[async_trait]
pub trait Storage: Send {
    async fn get(&mut self) -> Result<PathBuf, std::io::Error>;
}
