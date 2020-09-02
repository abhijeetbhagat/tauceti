extern crate async_std;
extern crate async_trait;
extern crate futures;
mod cache;
mod engine;
mod eventing;
mod parsing;
mod storage;
mod tasks;
mod trees;
mod utils;

use engine::SearchEngine;
use log::info;
use utils::connection_context::ConnectionContext;
use utils::error_structs::TaucetiError;

#[async_std::main]
async fn main() -> Result<(), TaucetiError> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let mut engine = SearchEngine::try_new(ConnectionContext {
        addr: "amqp://127.0.0.1:5672/%2f".into(),
        search_events_queue: "search_events".into(),
        upload_events_queue: "upload_events".into(),
    })
    .await?;

    info!("Starting engine ...");

    Ok(engine.start().await?)
}
