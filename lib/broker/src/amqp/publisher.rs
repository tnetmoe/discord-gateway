// SPDX-License-Identifier: AGPL-3.0-only
use crate::amqp::Result;
use amqprs::{
    channel::{BasicPublishArguments, Channel},
    BasicProperties,
};

pub struct MessageOptions {
    pub routing_key: String,
    pub content_type: Option<String>,
    pub encoding: Option<String>,
    pub persistence: Option<bool>,
    pub priority: Option<u8>,
    pub correlation_id: Option<String>,
    pub reply_to: Option<String>,
    pub message_id: Option<String>,
    pub timestamp: Option<u64>,
    pub message_type: Option<String>,
    pub app_id: Option<String>,
    pub cluster_id: Option<String>,
}

/// Publish a message.
pub async fn publish_message(
    channel: &Channel,
    exchange_name: String,
    options: MessageOptions,
    data: Vec<u8>,
) -> Result<()> {
    let args = BasicPublishArguments::default()
        .exchange(exchange_name)
        .routing_key(options.routing_key.clone())
        .mandatory(false)
        .immediate(false)
        .finish();
    let properties = publish_properties(options).await;
    channel.basic_publish(properties, data, args).await
}

/// AMQP message properties.
/// https://docs.rs/amqprs/latest/amqprs/struct.BasicProperties.html
async fn publish_properties(options: MessageOptions) -> BasicProperties {
    let mut properties = BasicProperties::default();
    if let Some(content_type) = options.content_type {
        properties.with_content_type(&content_type); // e.g. application/json or application/protobuf
    }
    if let Some(encoding) = options.encoding {
        properties.with_content_encoding(&encoding); // e.g. base64, binary, gzip or br
    }
    if let Some(persistence) = options.persistence {
        properties.with_persistence(persistence);
    }
    if let Some(priority) = options.priority {
        properties.with_priority(priority);
    }
    if let Some(correlation_id) = options.correlation_id {
        properties.with_correlation_id(&correlation_id);
    }
    if let Some(reply_to) = options.reply_to {
        properties.with_reply_to(&reply_to);
    }
    if let Some(message_id) = options.message_id {
        properties.with_message_id(&message_id);
    }
    if let Some(timestamp) = options.timestamp {
        properties.with_timestamp(timestamp);
    }
    if let Some(message_type) = options.message_type {
        properties.with_message_type(&message_type);
    }
    if let Some(app_id) = options.app_id {
        properties.with_app_id(&app_id);
    }
    if let Some(cluster_id) = options.cluster_id {
        properties.with_cluster_id(&cluster_id);
    }

    properties.finish()
}
