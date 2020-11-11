use crate::cache::cache::Cache;
use tide::{Request, Response};

use super::SearchEngine;

/// Searches for all the matching document(s) that satisfy
///
/// the given query.
pub(crate) async fn search<C>(req: Request<SearchEngine<C>>) -> tide::Result<impl Into<Response>>
where
    C: Cache,
{
    let query = req.param("query")?;
    let results: Vec<u32> = req.state().search(query).await.unwrap();
    let results: Vec<String> = results.iter().map(|n| n.to_string()).collect();
    Ok(results.join("\n"))
}

/// Inserts a skill name or a list of skill names in the
///
/// internal data store.
/*pub(crate) async fn insert<>(req: Request<SearchEngine>) -> tide::Result<impl Into<Response>> {
    Ok("success")
}*/

/// Accepts a prefix from the query param and queries the internal
///
/// data store to get a list of words matching the prefix.
pub(crate) async fn prefix_search<C>(
    req: Request<SearchEngine<C>>,
) -> tide::Result<impl Into<Response>>
where
    C: Cache,
{
    let prefix = req.param("prefix")?;
    let results = req.state().prefix_search(prefix).await.unwrap();
    Ok(results.join("\n"))
}
