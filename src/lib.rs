#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

//! # Messages
//!
//! This crate contains all the message definitions that will be used for inter-board communication
//! and ground-station communication.

use crate::command::Command;
use crate::node::Node;
use crate::sensor::Sensor;
use crate::state::State;
use chrono::NaiveDateTime;
use derive_more::From;
/// This is to help control versions.
pub use mavlink;

use messages_proc_macros_lib::common_derives;

pub mod command;
mod logging;
pub mod node;
pub mod sensor;
pub mod sensor_status;
pub mod state;

pub const MAX_SIZE_CAN: usize = 64;
pub const MAX_SIZE_RADIO: usize = 255;

use defmt::Format;
pub use logging::{ErrorContext, Event, Log, LogLevel};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[cfg_attr(all(feature = "std", test), derive(proptest_derive::Arbitrary))]
pub struct FormattedNaiveDateTime(pub NaiveDateTime);

impl Format for FormattedNaiveDateTime {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "");
    }
}

/// Topmost message. Encloses all the other possible messages, and is the only thing that should
/// be sent over the wire.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[cfg_attr(all(feature = "std", test), derive(proptest_derive::Arbitrary))]
pub struct Message {
    pub timestamp: FormattedNaiveDateTime,

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
    pub fn new(timestamp: FormattedNaiveDateTime, node: Node, data: impl Into<Data>) -> Self {
        Message {
            timestamp,
            node,
            data: data.into(),
        }
    }
}

#[cfg(all(test, feature = "std"))]
mod test {
    use crate::{Message, MAX_SIZE_CAN, MAX_SIZE_RADIO};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn message_size(msg: Message) {
            let bytes = postcard::to_allocvec(&msg).unwrap();

            dbg!(msg.clone());

            match msg.data {
                crate::Data::Sensor(sensor) => {
                    match sensor.data {
                        crate::sensor::SensorData::SbgData(_) => {
                            assert!(bytes.len() <= MAX_SIZE_RADIO);
                        }
                        _ => {
                            assert!(bytes.len() <= MAX_SIZE_CAN);
                        }
                    }
                }
                _ => {
                    assert!(bytes.len() <= MAX_SIZE_CAN);
                }

            }
        }
    }
}
