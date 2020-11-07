extern crate async_std;
extern crate async_trait;
extern crate futures;
extern crate tide;
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
use engine::SearchEngine;
use log::info;
use routes::{prefix_search, search};
use utils::connection_context::ConnectionContext;
use utils::error_structs::TaucetiError;

async fn cache_connect() -> impl Cache {
    let mut cache = MockCache::new();
    cache
}

#[async_std::main]
async fn main() -> Result<(), TaucetiError> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let cache = cache_connect().await;

    let mut engine = SearchEngine::try_new(cache, "walkin".into()).await?;

    let mut app = tide::with_state(engine);

    info!("Starting engine ...");

    app.at("/search/:query").get(routes::search);
    //app.at("/insert").get(routes::insert);
    app.at("/get_words/:prefix").get(routes::prefix_search);
    app.listen("0.0.0.0:8000").await;

    Ok(())
}
