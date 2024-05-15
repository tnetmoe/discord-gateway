// SPDX-License-Identifier: AGPL-3.0-only
pub mod broker;
mod connection;
mod channel;
mod exchange;
mod queue;
mod consumer;
mod publisher;
use amqprs::error::Error;

pub type Result<T> = std::result::Result<T, Error>;