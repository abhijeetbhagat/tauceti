use crate::eventing::event_listener::EventListener;
use crate::trees::trie::Trie;
use crate::utils::connection_context::ConnectionContext;
use crate::utils::error_structs::TaucetiError;

#[derive(Clone, Debug)]
/// Main engine that takes care of spawning event listening and other tasks.
pub(crate) struct SearchEngine {
    //listener: EventListener,
    trie: Trie,
}

impl SearchEngine {
    /// Creates a new `SearchEngine`
    pub async fn try_new(ctxt: ConnectionContext) -> Result<SearchEngine, TaucetiError> {
        let mut trie = Trie::new();
        trie.insert("cpp");
        trie.insert("clisp");
        trie.insert("clojure");

        Ok(SearchEngine {
            //listener: EventListener::try_new(ctxt).await?,
            trie,
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
}
