use crate::utils::error_structs::TaucetiError;

use super::cache::Cache;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MockCache {
    store: HashMap<String, String>,
}

impl MockCache {
    pub fn new() -> Self {
        MockCache {
            store: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl Cache for MockCache {
    async fn get(&self, key: &str) -> Result<String, TaucetiError> {
        self.store
            .get(key)
            .ok_or_else(|| TaucetiError::CacheError)
            .map(|val| val.clone())
    }

    async fn put(&mut self, key: &str, value: &str) -> Result<(), TaucetiError> {
        self.store.insert(key.into(), value.into());
        Ok(())
    }
}
