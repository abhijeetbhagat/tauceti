extern crate async_amqp;
extern crate lapin;
use crate::utils::connection_context::ConnectionContext;
use crate::utils::error_structs::TaucetiError;
use async_amqp::*;
use futures::channel::mpsc::UnboundedSender;
use futures::SinkExt;
use lapin::{
    message::DeliveryResult, options::*, publisher_confirm::Confirmation, types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, Consumer, Result,
};
use log::debug;

pub struct RabbitMQWrapper {
    search_queue: String,
    upload_queue: String,
    addr: String,
    conn: Option<Connection>,
    channel: Option<Channel>,
    consumer: Option<Consumer>,
    tx: UnboundedSender<Vec<u8>>,
}

impl RabbitMQWrapper {
    /// Creates a new `RabbitMQWrapper` object
    pub fn new(ctxt: ConnectionContext, tx: UnboundedSender<Vec<u8>>) -> RabbitMQWrapper {
        RabbitMQWrapper {
            addr: ctxt.addr,
            search_queue: ctxt.search_events_queue,
            upload_queue: ctxt.upload_events_queue,
            conn: None,
            tx,
            consumer: None,
            channel: None,
        }
    }

    /// Connects to the broker address and queue
    pub async fn connect(&mut self) -> Result<()> {
        self.conn = Some(
            Connection::connect(&self.addr, ConnectionProperties::default().with_async_std())
                .await?,
        );

        self.channel = Some(self.conn.as_ref().unwrap().create_channel().await?);

        debug!("consumer: subscribing to queue {} ...", self.search_queue);
        self.consumer = Some(
            self.channel
                .as_ref()
                .unwrap()
                .basic_consume(
                    &self.search_queue,
                    "search_events_consumer",
                    BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await?,
        );

        debug!("consumer: created channel...");
        let tx: UnboundedSender<Vec<u8>> = UnboundedSender::clone(&self.tx);
        self.consumer
            .as_ref()
            .unwrap()
            .set_delegate(move |delivery: DeliveryResult| {
                debug!("consumer: received message ...");
                let mut tx = UnboundedSender::clone(&tx);
                let delivery = delivery.expect("error caught in consumer");
                async move {
                    if let Some((channel, delivery)) = delivery {
                        channel
                            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                            .await
                            .expect("failed to ack");
                        let data = delivery.data;
                        debug!("consumer: sending data over channel ...");
                        tx.send(data).await;
                    };
                }
            })?;

        Ok(())
    }

    /// This is a test producer that just pushes a message continuously
    async fn start_test_producer(&mut self) -> Result<()> {
        let channel_a = self.conn.as_ref().unwrap().create_channel().await?;
        let payload = b"Hello world!";

        loop {
            println!("looping");
            let confirm = channel_a
                .basic_publish(
                    "",
                    "search_events",
                    BasicPublishOptions::default(),
                    payload.to_vec(),
                    BasicProperties::default(),
                )
                .await?
                .await?;
            assert_eq!(confirm, Confirmation::NotRequested);
        }
    }
}
