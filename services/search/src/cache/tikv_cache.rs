use crate::utils::error_structs::TaucetiError;

use super::cache::Cache;
use tikv_client::{Config, RawClient};

#[derive(Clone)]
pub struct TiKVCache {
    client: RawClient,
}

impl TiKVCache {
    pub async fn new(addr: &str) -> Result<TiKVCache, TaucetiError> {
        let config = Config::new(vec![addr]);
        let client = RawClient::new(config)
            .await
            .map_err(|_| TaucetiError::CacheConnectionError)?;

        Ok(TiKVCache { client })
    }
}

#[async_trait::async_trait]
impl Cache for TiKVCache {
    async fn get(&self, key: &str) -> Result<String, TaucetiError> {
        let result = self
            .client
            .get(key.as_bytes().to_owned())
            .await
            .map_err(|_| TaucetiError::CacheError)?;
        let result = result.ok_or_else(|| TaucetiError::CacheDataNotFoundError)?;
        Ok(std::str::from_utf8(result.as_slice()).unwrap().to_owned())
    }

    async fn put(&mut self, key: &str, value: &str) -> Result<(), TaucetiError> {
        self.client
            .put(key.as_bytes().to_owned(), value.as_bytes().to_owned())
            .await
            .map_err(|_| TaucetiError::CacheWriteError)
    }
}
