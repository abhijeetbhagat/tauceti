use tide::{Request, Response};

use super::SearchEngine;

/// Searches for all the matching document(s) that satisfy
///
/// the given query.
pub(crate) async fn search(req: Request<SearchEngine>) -> tide::Result<impl Into<Response>> {
    Ok("success")
}

/// Inserts a skill name or a list of skill names in the
///
/// internal data store.
pub(crate) async fn insert(req: Request<SearchEngine>) -> tide::Result<impl Into<Response>> {
    Ok("success")
}

/// Accepts a prefix from the query param and queries the internal
///
/// data store to get a list of words matching the prefix.
pub(crate) async fn prefix_search(req: Request<SearchEngine>) -> tide::Result<impl Into<Response>> {
    let prefix: String = req.param("prefix")?;
    let results = req.state().prefix_search(prefix.as_str()).await.unwrap();
    Ok(results.join("\n"))
}
