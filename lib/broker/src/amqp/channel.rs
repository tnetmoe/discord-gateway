// SPDX-License-Identifier: AGPL-3.0-only
//! RabbitMQ channel callback module.
use std::time::Duration;

use amqprs::{
    callbacks::ChannelCallback,
    channel::Channel,
    connection::Connection,
    Ack, BasicProperties, Cancel, CloseChannel, Nack, Return,
};
use async_trait::async_trait;
use log::{debug, error};
use crate::amqp::Result;

pub async fn create_channel(connection: &Connection, fail_wait_time: Option<u64>, retries: Option<u16>) -> Result<Channel> {
    let mut retries_count = retries.unwrap();
    let wait_time = fail_wait_time.unwrap_or(5);
    loop {
        match connection.open_channel(None).await {
            Ok(channel) => {
                channel.register_callback(AMQPChannelCallback).await.unwrap();
                debug!("Opened a new channel, ID: {}", channel.channel_id());
                return Ok(channel);
            },
            Err(err) => {
                if retries_count == 0 {
                    error!("Failed to open a channel after all retries.");
                    return Err(err);
                }

                retries_count -= 1;
                error!("Failed to open a channel: {}. {} retries left. Retrying in {}s...", err, retries_count, wait_time);
                tokio::time::sleep(Duration::from_secs(wait_time)).await;
            }
        }
    }
}
pub struct AMQPChannelCallback;

// TODO: do something
#[async_trait]
impl ChannelCallback for AMQPChannelCallback {
    async fn close(&mut self, channel: &Channel, close: CloseChannel) -> Result<()> {
        Ok(())
    }

    async fn cancel(&mut self, channel: &Channel, cancel: Cancel) -> Result<()> {
        Ok(())
    }

    async fn flow(&mut self, channel: &Channel, active: bool) -> Result<bool> {
        Ok(true)
    }

    async fn publish_ack(&mut self, channel: &Channel, ack: Ack) {}

    async fn publish_nack(&mut self, channel: &Channel, nack: Nack) {}

    async fn publish_return(
        &mut self, channel: &Channel,
        ret: Return,
        basic_properties: BasicProperties,
        content: Vec<u8>
    ) {}
}