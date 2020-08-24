use super::rabbitmq_wrapper::RabbitMQWrapper;

use crate::utils::{
    connection_context::ConnectionContext, error_structs::TaucetiError, events::TaucetiEvent,
    reader_enums::DocType, storage_enums::StorageType,
};
use crate::{tasks::build_task, trees::index_tree::IndexTree};
use async_std::{
    prelude::*,
    sync::{Arc, RwLock},
    task,
};
use futures::channel::mpsc;
use log::{debug, error, info};

/// Listens to the event coming from a broker.
pub(crate) struct EventListener {
    broker: RabbitMQWrapper,
    rx: mpsc::UnboundedReceiver<Vec<u8>>,
    tree: Arc<RwLock<IndexTree<String, u32>>>,
}

impl EventListener {
    /// Creates an `EventListener`
    ///
    /// `ConnectionContext` contains the address and the queue name
    pub async fn new(ctxt: ConnectionContext) -> Result<EventListener, TaucetiError> {
        let (tx, rx) = mpsc::unbounded();
        Ok(EventListener {
            broker: RabbitMQWrapper::new(ctxt, tx),
            rx,
            tree: Arc::new(RwLock::new(IndexTree::new())),
        })
    }

    /// Starts event listening
    pub async fn start(&mut self) -> Result<(), TaucetiError> {
        self.broker
            .connect()
            .await
            .map_err(|_| TaucetiError::MessageBrokerError)?;

        debug!("Connected to broker ...");
        while let Some(msg) = self.rx.next().await {
            let tree = self.tree.clone();
            task::spawn(async move {
                if Self::handle_event(msg, tree).await.is_err() {
                    error!("Error occurred during handling event");
                }
            });
        }
        Ok(())
    }

    /// Handles an event.
    ///
    /// TODO: abhi: this should handle a type like `Event`
    async fn handle_event(
        msg: Vec<u8>,
        tree: Arc<RwLock<IndexTree<String, u32>>>,
    ) -> Result<(), std::io::Error> {
        let msg = String::from_utf8(msg).unwrap();
        let event: TaucetiEvent = serde_json::from_str(&msg).unwrap();

        debug!("msg received from broker {}", msg,);
        match event {
            TaucetiEvent::UploadEvent(storage_type, doc_type, uri, doc_id) => {
                Self::handle(storage_type, uri, tree, doc_id, doc_type).await?;
            }
            TaucetiEvent::SearchEvent(query) => todo!(),
        }

        Ok(())
    }

    /// Kicks off document processing
    async fn handle(
        storage_type: StorageType,
        uri: String,
        tree: Arc<RwLock<IndexTree<String, u32>>>,
        doc_id: u32,
        doc_type: DocType,
    ) -> Result<(), std::io::Error> {
        build_task::build(storage_type, uri, tree, doc_id, doc_type).await?;
        Ok(())
    }
}
