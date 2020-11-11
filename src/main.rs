//extern crate async_std;
//extern crate async_trait;
//extern crate futures;

mod cache;
mod engine;
mod eventing;
mod parsing;
mod routes;
mod storage;
mod tasks;
mod trees;
mod utils;

use cache::cache::Cache;
use cache::in_mem_cache::MockCache;
use cache::tikv_cache::TiKVCache;
use engine::SearchEngine;
use log::info;
use routes::{prefix_search, search};
use std::env;
use tide_rustls::TlsListener;
use utils::connection_context::ConnectionContext;
use utils::error_structs::TaucetiError;

async fn cache_connect() -> Result<impl Cache, TaucetiError> {
    TiKVCache::new("127.0.0.1:2379").await
}

#[async_std::main]
async fn main() -> Result<(), TaucetiError> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let cache = cache_connect().await?;

    let engine = SearchEngine::try_new(cache, "walkin".into()).await?;

    let mut app = tide::with_state(engine);

    info!("Starting engine ...");

    app.at("/search/:query").get(routes::search);
    //app.at("/insert").get(routes::insert);
    app.at("/get_words/:prefix").get(routes::prefix_search);

    app.listen(
        TlsListener::build()
            .addrs("localhost:4433")
            .cert(std::env::var("TIDE_CERT_PATH").unwrap())
            .key(std::env::var("TIDE_KEY_PATH").unwrap()),
    )
    .await
    .map_err(|_| TaucetiError::ServiceStartError)?;

    Ok(())
}
