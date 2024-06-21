// SPDX-License-Identifier: AGPL-3.0-only
//
use prost::Message;
pub mod gateway {
    include!(concat!(env!("OUT_DIR"), "/dg.gateway.rs"));
}
use gateway::{Event, EventData, EventType, Gateway};

pub async fn serialize_event(event: &Event) -> Vec<u8> {
    let mut buf = Vec::with_capacity(event.encoded_len());
    event.encode(&mut buf).unwrap();
    buf
}

pub async fn deserialize_event(data: &[u8]) -> Result<Event, prost::DecodeError> {
    Event::decode(data)
}