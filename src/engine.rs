use crate::eventing::event_listener::EventListener;
use crate::utils::connection_context::ConnectionContext;
use crate::utils::error_structs::TaucetiError;

/// Main engine that takes care of spawning event listening and other tasks.
pub(crate) struct SearchEngine {
    listener: EventListener,
}

impl SearchEngine {
    /// Creates a new `SearchEngine`
    pub async fn try_new(ctxt: ConnectionContext) -> Result<SearchEngine, TaucetiError> {
        Ok(SearchEngine {
            listener: EventListener::try_new(ctxt).await?,
        })
    }

    /// Starts all the required services required to serve searching
    pub async fn start(&mut self) -> Result<(), TaucetiError> {
        self.listener.start().await?;
        Ok(())
    }
}
