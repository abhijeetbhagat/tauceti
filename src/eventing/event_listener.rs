use super::rabbitmq_wrapper::RabbitMQWrapper;

use crate::storage::{storage_interface::Storage, storage_utils::create_storage_interface};
use crate::utils::{
    connection_context::ConnectionContext, error_structs::TaucetiError, events::TaucetiEvent,
};
use crate::{
    parsing::{reader::DocReader, reader_utils::create_doc_reader},
    tasks::build_task,
    trees::index_tree::IndexTree,
};
use async_std::prelude::*;
use async_std::task;
use futures::channel::mpsc;
use log::{debug, error, info};
use std::sync::{Arc, Mutex};

/// Listens to the event coming from a broker.
pub(crate) struct EventListener {
    broker: RabbitMQWrapper,
    rx: mpsc::UnboundedReceiver<Vec<u8>>,
    tree: Arc<Mutex<IndexTree<String, u32>>>,
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
            tree: Arc::new(Mutex::new(IndexTree::new())),
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
        tree: Arc<Mutex<IndexTree<String, u32>>>,
    ) -> Result<(), std::io::Error> {
        let msg = String::from_utf8(msg).unwrap();
        let event: TaucetiEvent = serde_json::from_str(&msg).unwrap();

        debug!("msg received from broker {}", msg,);
        match event {
            TaucetiEvent::UploadEvent(storage_type, doc_type, uri, doc_id) => {
                let mut storage = create_storage_interface(&storage_type, uri.clone());
                let mut reader = create_doc_reader(&doc_type, uri.clone());

                Self::handle(&mut storage, &mut reader, tree, doc_id).await?;
            }
            TaucetiEvent::SearchEvent(terms) => {}
        }

        Ok(())
    }

    /// Kicks off document processing
    async fn handle(
        s: &mut Box<dyn Storage>,
        r: &mut Box<dyn DocReader>,
        tree: Arc<Mutex<IndexTree<String, u32>>>,
        doc_id: u32,
    ) -> Result<(), std::io::Error> {
        build_task::build(s.as_mut(), r.as_mut(), tree, doc_id).await?;
        Ok(())
    }
}
