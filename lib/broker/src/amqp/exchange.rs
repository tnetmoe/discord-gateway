// SPDX-License-Identifier: AGPL-3.0-only
use amqprs::channel::{Channel, ExchangeDeclareArguments};
use crate::amqp::Result;

/// Create a new RabbitMQ exchange.
pub async fn create_exchange(channel: &Channel, exchange_name: &str) -> Result<()> {
    let mut args = ExchangeDeclareArguments::new(
        exchange_name,
        // Use topic exchange so events can be routed to multiple queues.
        // This is useful since e.g. the voice gateway as well as as the workers have to receive voice state updates.
        "topic"
    );
    args.durable(true);
    channel.exchange_declare(args).await
}