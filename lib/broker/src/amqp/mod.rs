// SPDX-License-Identifier: AGPL-3.0-only
pub mod broker;
mod channel;
mod connection;
pub mod consumer;
mod exchange;
mod publisher;
mod queue;
use amqprs::error::Error;

pub type Result<T> = std::result::Result<T, Error>;
