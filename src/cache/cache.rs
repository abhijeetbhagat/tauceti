use async_trait::async_trait;

#[async_trait]
/// Represents a cache where data is stored temporarily
pub trait Cache {
    async fn get(&self) -> Result<String, std::io::Error>;
    async fn put(&mut self) -> Result<(), std::io::Error>;
}
