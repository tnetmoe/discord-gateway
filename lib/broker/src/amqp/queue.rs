// SPDX-License-Identifier: AGPL-3.0-only
use crate::amqp::Result;
use amqprs::channel::{Channel, QueueBindArguments, QueueDeclareArguments};
use log::{debug, error};

// TODO: support retries
/// Create a new RabbitMQ queue.
pub async fn create_queue(channel: &Channel, queue_name: &str) -> Result<(String, u32, u32)> {
    let args = QueueDeclareArguments::durable_client_named(queue_name);

    match channel.queue_declare(args).await {
        Ok(Some(queue)) => {
            debug!(
                "Declared queue: {}; Messages: {}; Consumers: {}",
                queue_name, queue.1, queue.2
            );
            Ok(queue)
        }
        Ok(None) => {
            let err = format!("Failed to declare queue: {} already exists", queue_name);
            error!("Failed to declare queue: {} already exists", queue_name);
            Err(amqprs::error::Error::ChannelOpenError(err))
        }
        Err(err) => {
            error!("Failed to declare queue {}: {}", queue_name, err);
            Err(err)
        }
    }
}

pub async fn bind_queue(
    channel: &Channel,
    queue: String,
    exchange: String,
    routing_key: String,
) -> Result<()> {
    let args = QueueBindArguments::default()
        .queue(queue)
        .exchange(exchange)
        .routing_key(routing_key)
        .arguments(Default::default())
        .finish();
    channel.queue_bind(args).await
}
