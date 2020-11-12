use crate::utils::error_structs::TaucetiError;

use super::cache::Cache;
use async_trait::async_trait;

#[derive(Clone)]
pub struct RedisWrapper {}

impl RedisWrapper {
    pub fn new() -> RedisWrapper {
        RedisWrapper {}
    }
}

#[async_trait::async_trait]
impl Cache for RedisWrapper {
    async fn get(&self, key: &str) -> Result<String, TaucetiError> {
        todo!()
    }

    async fn put(&mut self, key: &str, value: &str) -> Result<(), TaucetiError> {
        todo!()
    }
}
