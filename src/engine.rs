use crate::cache::cache::Cache;
use crate::eventing::event_listener::EventListener;
use crate::parsing::query_parsing::query_parser::QueryParser;
use crate::tasks::search_helper;
use crate::trees::{index_tree::IndexTree, trie::Trie};
use crate::utils::connection_context::ConnectionContext;
use crate::utils::error_structs::TaucetiError;
use async_std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
/// Main engine that takes care of spawning event listening and other tasks.
pub(crate) struct SearchEngine<C>
where
    C: Cache,
{
    //listener: EventListener,
    trie: Trie,
    tree: Arc<RwLock<IndexTree<String, u32>>>,
    cache: C,
    key: String,
}

impl<C> SearchEngine<C>
where
    C: Cache,
{
    /// Creates a new `SearchEngine`
    pub async fn try_new(mut cache: C, key: String) -> Result<SearchEngine<C>, TaucetiError> {
        let mut trie = Trie::new();
        trie.insert("cpp");
        trie.insert("clisp");
        trie.insert("clojure");

        let mut tree = IndexTree::new();
        tree.insert("cpp".into(), 1u32);
        tree.insert("clisp".into(), 1);
        tree.insert("clojure".into(), 1);

        cache
            .put(&key, &serde_json::to_string(&tree).unwrap())
            .await;

        Ok(SearchEngine {
            //listener: EventListener::try_new(ctxt).await?,
            trie,
            tree: Arc::new(RwLock::new(tree)),
            cache,
            key,
        })
    }

    /// Starts all the required services required to serve searching
    pub async fn start(&mut self) -> Result<(), TaucetiError> {
        //self.listener.start().await?;
        Ok(())
    }

    pub async fn prefix_search(&self, prefix: &str) -> Result<Vec<String>, TaucetiError> {
        let results = self.trie.get(prefix);
        Ok(results.ok_or(TaucetiError::NotFoundInDictError)?)
    }

    pub async fn search(&self, query: &str) -> Result<Vec<u32>, TaucetiError> {
        let expr = QueryParser::new(query).parse()?;
        let tree = self.cache.get(&self.key).await?;
        let tree: IndexTree<String, u32> = serde_json::from_str(&tree).unwrap();
        let set = search_helper::walk(&expr, &tree).await;
        Ok(set.into_iter().collect::<Vec<u32>>())
    }
}
