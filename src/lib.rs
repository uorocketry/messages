#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![no_main]

//! # Messages
//!
//! This crate contains all the message definitions that will be used for inter-board communication
//! and ground-station communication.

use crate::command::Command;
use crate::node::Node;
use crate::sensor::Sensor;
use crate::state::State;
use derive_more::From;
/// This is to help control versions.
pub use mavlink;
use chrono::NaiveDateTime;
use messages_proc_macros_lib::common_derives;

pub mod command;
mod logging;
pub mod node;
pub mod sensor;
pub mod sensor_status;
pub mod state;
use chrono::{NaiveDate, NaiveTime};
#[cfg(test)]
use proptest::prelude::*;
pub const MAX_SIZE: usize = 64;

pub use logging::{ErrorContext, Event, Log, LogLevel};
use defmt::Format;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct FormattedNaiveDateTime(pub NaiveDateTime);

impl Format for FormattedNaiveDateTime {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "");
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ArbitraryNaiveDateTime(pub FormattedNaiveDateTime);

#[cfg(test)]
impl Arbitrary for ArbitraryNaiveDateTime {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (0..=9999i32, 1u32..=12, 1u32..=31, 0u32..24, 0u32..60, 0u32..60, 0u32..1_000_000_000)
            .prop_map(|(year, month, day, hour, minute, second, nanosecond)| {
                ArbitraryNaiveDateTime(
                    FormattedNaiveDateTime(NaiveDate::from_ymd(year, month, day).and_hms_nano(hour, minute, second, nanosecond))
                )
            })
            .boxed()
    }
}

/// Topmost message. Encloses all the other possible messages, and is the only thing that should
/// be sent over the wire.
#[common_derives(NoFormat)]
pub struct Message {
    pub timestamp: ArbitraryNaiveDateTime,

    /// The original sender of this message.
    pub node: Node,

    /// The data contained in this message.
    pub data: Data,
}

#[common_derives]
#[derive(From)]
#[serde(rename_all = "lowercase")]
pub enum Data {
    State(State),
    Sensor(Sensor),
    Log(Log),
    Command(Command),
}

impl Message {
    pub fn new(timestamp: ArbitraryNaiveDateTime, node: Node, data: impl Into<Data>) -> Self {
        Message {
            timestamp,
            node,
            data: data.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Message, MAX_SIZE};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn message_size(msg: Message) {
            let bytes = postcard::to_allocvec(&msg).unwrap();

            dbg!(msg);
            assert!(dbg!(bytes.len()) <= MAX_SIZE);
        }
    }
}
