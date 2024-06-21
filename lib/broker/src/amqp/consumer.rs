// SPDX-License-Identifier: AGPL-3.0-only
//! RabbitMQ consumer callback module.
use amqprs::{
    channel::{BasicAckArguments, BasicConsumeArguments, Channel},
    consumer::AsyncConsumer,
    BasicProperties,
    Deliver,
};
use async_trait::async_trait;
use log::{info, error};
use crate::{
    amqp::Result,
    proto::{
        gateway::Event,
        deserialize_event
    }
};

pub type ConsumerCallback = fn(queue_name: String, event: Event) -> Result<()>;

pub async fn create_consumer(channel: &Channel, queue_name: String, consumer_tag: String, callback: ConsumerCallback) -> Result<String> {
    let args = BasicConsumeArguments::new(
        &queue_name,
        &consumer_tag
    );

    channel.basic_consume(AMQPConsumer::new(
        callback
    ), args).await
}

pub struct AMQPConsumer {
    pub callback: ConsumerCallback
}

impl AMQPConsumer {
    pub fn new(callback: ConsumerCallback) -> Self {
        Self {
            callback
        }
    }
}

#[async_trait]
impl AsyncConsumer for AMQPConsumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        #[cfg(feature = "traces")]
        info!(
            "consume delivery {} on channel {}, content size: {}",
            deliver,
            channel,
            content.len()
        );

        // check content type
        let content_type = _basic_properties.content_type();
        if content_type.is_none() {
            error!("Missing content type");
            // TODO: reject message
            //channel.basic_reject(args).await.unwrap();
            return;
        }

        let event: Event = match content_type.unwrap().as_str() {
            "application/json" => {
                // TODO: reject message
                //channel.basic_reject(args).await.unwrap();
                return;
            },
            "application/protobuf" => {
                deserialize_event(&content).await.unwrap()
            },
            _ => {
                error!("Unsupported content type: {}", content_type.unwrap());
                // TODO: reject message
                //channel.basic_reject(args).await.unwrap();
                return;
            }
        };

        // TODO: call callback, forward message as arg
        (self.callback)(deliver.routing_key().to_string(), event).unwrap();

        // acknowledge message after callback is done and returned Ok(())
        // TODO: callback
        let args = BasicAckArguments::new(deliver.delivery_tag(), false);
        channel.basic_ack(args).await.unwrap();
    }
}