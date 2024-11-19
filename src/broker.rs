use anyhow::{bail, Result};
use async_trait::async_trait;
use bb8::{CustomizeConnection, Pool};
use bb8_lapin::prelude::*;
use futures_lite::StreamExt;
use lapin::{
    message::Delivery,
    options::*,
    protocol::{AMQPError, AMQPErrorKind, AMQPHardError},
    publisher_confirm::PublisherConfirm,
    types::{FieldTable, ShortString},
    BasicProperties, Connection, ConnectionState,
};
use secrecy::{ExposeSecret, Secret};
use serde::Serialize;
use serde_json::ser::to_string;
use tracing::{debug, info};

use crate::configuration::RabbitMqSettings;

#[derive(Debug)]
struct Customizer;

#[derive(Clone)]
pub struct MessageBroker {
    pool: Pool<CustomLapinConnectionManager>,
    queue: Secret<String>,
}

impl MessageBroker {
    pub async fn new(settings: &RabbitMqSettings) -> Self {
        let addr = format!("amqp://{}:{}", &settings.host, &settings.port);
        let lapin_manager = CustomLapinConnectionManager::new(
            addr.clone(),
            ConnectionProperties::default(),
            settings.consumer.clone(),
            settings.queue.clone(),
        );

        let pool = Pool::builder()
            .connection_customizer(Box::new(Customizer))
            .max_size(1)
            .min_idle(1)
            .retry_connection(true)
            .build(lapin_manager)
            .await
            .expect("build error");

        info!(
            "POOL of rabbitmq running on address: {} and queue: {}",
            &addr, settings.queue
        );
        Self {
            pool,
            queue: Secret::new(settings.queue.clone()),
        }
    }

    pub async fn publish<T: Serialize>(&self, message: T) -> Result<PublisherConfirm> {
        let conn = self.pool.get().await?;

        match &conn.channel {
            Some(channel) => {
                let json = to_string(&message)?.clone();
                dbg!(&json);
                Ok(channel
                    .basic_publish(
                        "",
                        self.queue.expose_secret(),
                        BasicPublishOptions::default(),
                        json.as_bytes(),
                        BasicProperties::default(),
                    )
                    .await?)
            },
            None => {
                bail!("Unable to acquire a new channel")
            },
        }
    }

    pub async fn on_message(&mut self) -> Result<Option<Result<Delivery, lapin::Error>>> {
        let mut conn = self.pool.get().await?;

        match &mut conn.consumer {
            Some(consumer) => {
                let res = consumer.next().await;
                debug!("CONSUMING message",);
                Ok(res)
            },
            None => {
                bail!("Unable to consume ")
            },
        }
    }
}

pub struct CustomLapinConnection {
    inner: Connection,
    channel: Option<lapin::Channel>,
    consumer: Option<lapin::Consumer>,
    consumer_name: String,
    queue_name: String,
}
impl CustomLapinConnection {
    pub fn new(conn: lapin::Connection, consumer_name: String, queue_name: String) -> Self {
        Self {
            inner: conn,
            channel: None,
            consumer: None,
            consumer_name,
            queue_name,
        }
    }
}

pub struct CustomLapinConnectionManager {
    uri: String,
    conn_properties: ConnectionProperties,
    consumer_name: String,
    queue_name: String,
}
impl CustomLapinConnectionManager {
    pub fn new(
        uri: String,
        conn_properties: ConnectionProperties,
        consumer_name: String,
        queue_name: String,
    ) -> Self {
        Self {
            uri,
            conn_properties,
            consumer_name,
            queue_name,
        }
    }
}
#[async_trait]
impl CustomizeConnection<CustomLapinConnection, lapin::Error> for Customizer {
    async fn on_acquire(&self, conn: &mut CustomLapinConnection) -> Result<(), lapin::Error> {
        let channel = conn.inner.create_channel().await?;
        // Confirm acknowledge of queue
        channel
            .confirm_select(ConfirmSelectOptions::default())
            .await?;
        conn.channel = Some(channel);
        let f = conn.channel
            .as_ref()
            .unwrap()
            .basic_qos(1, BasicQosOptions::default())
            .await?;
        conn.consumer = Some(
            conn.channel
                .as_ref()
                .unwrap()
                .basic_consume(
                    &conn.queue_name,
                    &conn.consumer_name,
                    BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await?,
        );
        info!(
            "ACQUIRED connection and consumer: {} on queue: {}",
            conn.consumer_name, conn.queue_name
        );
        Ok(())
    }
}

#[async_trait]
impl bb8::ManageConnection for CustomLapinConnectionManager {
    type Connection = CustomLapinConnection;
    type Error = lapin::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let conn = lapin::Connection::connect(&self.uri, self.conn_properties.clone()).await?;
        Ok(CustomLapinConnection::new(
            conn,
            self.consumer_name.clone(),
            self.queue_name.clone(),
        ))
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let valid_states = [
            ConnectionState::Initial,
            ConnectionState::Connecting,
            ConnectionState::Connected,
        ];
        if valid_states.contains(&conn.inner.status().state()) {
            Ok(())
        } else {
            Err(lapin::Error::ProtocolError(AMQPError::new(
                AMQPErrorKind::Hard(AMQPHardError::CONNECTIONFORCED),
                ShortString::from("Invalid connection"),
            )))
        }
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        let broken_states = [ConnectionState::Closed, ConnectionState::Error];
        broken_states.contains(&conn.inner.status().state())
    }
}
