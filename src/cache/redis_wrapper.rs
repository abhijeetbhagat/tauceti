use super::cache::Cache;
use async_trait::async_trait;

pub struct RedisWrapper {}

impl RedisWrapper {
    pub fn new() -> RedisWrapper {
        RedisWrapper {}
    }
}

#[async_trait::async_trait]
impl Cache for RedisWrapper {
    async fn get(&self) -> Result<String, std::io::Error> {
        todo!()
    }
    async fn put(&mut self) -> Result<(), std::io::Error> {
        todo!()
    }
}
