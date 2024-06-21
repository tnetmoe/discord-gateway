// SPDX-License-Identifier: AGPL-3.0-only
//! RabbitMQ connection callback module.
use std::time::Duration;

use crate::amqp::Result;
use amqprs::{
    callbacks::ConnectionCallback,
    connection::{Connection, OpenConnectionArguments},
    Close,
};
use async_trait::async_trait;
use log::{error, info};

/// Create a new connection to the AMQP server.
pub async fn create_connection(
    host: String,
    port: u16,
    username: String,
    password: String,
    fail_wait_time: Option<u64>,
    retries: Option<u16>,
) -> Result<Connection> {
    // Open a connection to the RabbitMQ server. Retries every 5 seconds if it fails.
    let mut retries_count = retries.unwrap_or(0);
    let wait_time = fail_wait_time.unwrap_or(5);
    loop {
        match Connection::open(&OpenConnectionArguments::new(
            &host, port, &username, &password,
        ))
        .await
        {
            Ok(connection) => {
                connection
                    .register_callback(AMQPConnectionCallback)
                    .await
                    .unwrap();
                info!("Connected to RabbitMQ server at {}:{}", host, port);
                return Ok(connection);
            }
            Err(err) => {
                if retries_count == 0 {
                    error!("Failed to connect to RabbitMQ server after all retries.");
                    return Err(err);
                }

                retries_count -= 1;
                error!(
                    "Failed to connect to RabbitMQ server: {}. {} retries left. Retrying in {}s...",
                    err, retries_count, wait_time
                );
                tokio::time::sleep(Duration::from_secs(wait_time)).await;
            }
        }
    }
}

pub struct AMQPConnectionCallback;

// TODO: do something
#[async_trait]
impl ConnectionCallback for AMQPConnectionCallback {
    async fn close(&mut self, connection: &Connection, close: Close) -> Result<()> {
        error!(
            "handle close request for connection {}, cause: {}",
            connection, close
        );
        Ok(())
    }

    async fn blocked(&mut self, connection: &Connection, reason: String) {
        info!(
            "handle blocked notification for connection {}, reason: {}",
            connection, reason
        );
    }

    async fn unblocked(&mut self, connection: &Connection) {
        info!(
            "handle unblocked notification for connection {}",
            connection
        );
    }
}
