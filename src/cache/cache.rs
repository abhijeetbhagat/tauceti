use async_trait::async_trait;

use crate::utils::error_structs::TaucetiError;

#[async_trait]
/// Represents a cache where data is stored temporarily
pub trait Cache: Clone {
    async fn get(&self, key: &str) -> Result<String, TaucetiError>;
    async fn put(&mut self, key: &str, value: &str) -> Result<(), TaucetiError>;
}
