// SPDX-License-Identifier: AGPL-3.0-only
//! RabbitMQ consumer callback module.
use amqprs::{
    channel::{BasicAckArguments, BasicConsumeArguments, Channel},
    consumer::AsyncConsumer,
    BasicProperties,
    Deliver,
};
use async_trait::async_trait;

//pub type ConsumerCallback = Box<dyn FnMut(&Channel, &str, Vec<u8>) + Send>;
pub type ConsumerCallback = fn(channel: &Channel, queue_name: String, data: Vec<u8>) -> Result<()>;

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

        let content_type = _basic_properties.content_type();
        if content_type.is_none() {
            error!("Missing content type");
            // TODO: reject message
            //channel.basic_reject(args).await.unwrap();
            return;
        }

        // TODO: call callback, forward message as arg
        (self.callback)(channel, deliver.routing_key().to_string(), content).unwrap();

        // acknowledge message after callback is done and returned Ok(())
        // TODO: callback
        let args = BasicAckArguments::new(deliver.delivery_tag(), false);
        channel.basic_ack(args).await.unwrap();
    }
}