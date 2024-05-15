use amqprs::{
    channel::Channel,
    connection::Connection,
};
use log::{error, debug};
use crate::amqp::{
    Result,
    connection::create_connection,
    channel::create_channel,
    publisher::{MessageOptions, publish_message},
    consumer::{ConsumerCallback, create_consumer},
    exchange::create_exchange,
    queue::{create_queue, bind_queue}
};

pub struct Broker {
    options: RabbitMQBrokerOptions,
    connection: Connection,
    channel: Channel
}

#[derive(Clone)]
pub struct RabbitMQBrokerOptions {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub virtual_host: String,
    pub queue_name: String,
    pub stream_name: String,
    pub exchange_name: String,
    pub routing_key: String,
}

impl Broker {
    /// Create a new RabbitMQ message broker instance.
    pub async fn new(options: &RabbitMQBrokerOptions) -> anyhow::Result<Self> {
        // Create connection.
        let connection = create_connection(
            options.host.clone(),
            options.port,
            options.username.clone(),
            options.password.clone(),
            None,
            None
        ).await.unwrap();

        // Create new channel.
        let channel = create_channel(&connection, None, None).await.unwrap();

        // Create exchange.
        match create_exchange(&channel, &options.exchange_name).await {
            Ok(_) => debug!("Declared exchange: {}", options.exchange_name),
            Err(err) => {
                error!("Failed to declare exchange: {}", err);
                return Err(anyhow::anyhow!("Failed to declare exchange: {}", err));
            }
        };
        
        // Create queue.
        let _ = create_queue(&channel, &options.queue_name).await;

        // TODO: create & bind multiple queues, one for events and (optionally) one for the backend and/or (specific events to) custom queues.
        // Bind the queue to the exchange.
        bind_queue(
            &channel,
            options.queue_name.clone(),
            options.exchange_name.clone(),
            options.routing_key.clone()
        ).await.unwrap();
        debug!("Connected to exchange: {}", options.exchange_name);

        Ok(Self {
            options: options.clone(),
            connection,
            channel
        })
    }

    /// Close the RabbitMQ server connection.
    pub async fn close(&mut self) -> anyhow::Result<()> {
        unimplemented!()
    }

    /// Publish a (single) message to the broker.
    pub async fn send(
        &mut self,
        options: MessageOptions,
        data: Vec<u8>,
    ) -> Result<()> {
        publish_message(
            &self.channel,
            self.options.exchange_name.clone(),
            options,
            data
        ).await
    }

    /// Listen for messages from the broker.
    pub async fn listen(&mut self, consumer_tag: String, consumer_callback: ConsumerCallback) -> Result<String> {
        create_consumer(
            &self.channel,
            self.options.queue_name.clone(),
            consumer_tag,
            consumer_callback
        ).await
    }
}