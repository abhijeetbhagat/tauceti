use super::task::TaucetiTask;
use crate::parsing::query_parsing::query_parser::QueryParser;
use crate::{trees::index_tree::IndexTree, utils::error_structs::TaucetiError};
use async_std::sync::{Arc, RwLock};

struct SearchTask {}

impl SearchTask {
    fn new() -> SearchTask {
        SearchTask {}
    }
}

impl TaucetiTask for SearchTask {
    fn execute() -> Result<(), std::io::Error> {
        todo!()
    }
}

pub fn filter(query: String) -> Vec<String> {
    todo!()
}

pub async fn search(
    tree: Arc<RwLock<IndexTree<String, u32>>>,
    query: String,
) -> Result<Vec<u32>, TaucetiError> {
    let mut parser = QueryParser::new(&query);
    let expr = parser.parse()?;
    let set = super::search_helper::walk(&expr, tree).await;
    Ok(set.into_iter().collect::<Vec<u32>>())
}
