#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

//! # Messages
//!
//! This crate contains all the message definitions that will be used for inter-board communication
//! and ground-station communication.

use crate::node::Node;
use crate::state::State;
use chrono::NaiveDateTime;
use derive_more::From;

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
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, defmt::Format)]
pub struct RadioMessage<'a> {
    pub timestamp: FormattedNaiveDateTime,

    /// The original sender of this message.
    pub node: Node,

    /// The data contained in this message.
    #[serde(borrow)]
    pub data: RadioData<'a>,
}

/// Topmost message. Encloses all the other possible messages, and is the only thing that should
/// be sent over the wire.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, defmt::Format)]
#[cfg_attr(all(feature = "std", test), derive(proptest_derive::Arbitrary))]
pub struct CanMessage {
    pub timestamp: FormattedNaiveDateTime,

    /// The original sender of this message.
    pub node: Node,

    /// The data contained in this message.
    pub data: CanData,
}


#[common_derives]
#[derive(From)]
pub enum Common {
    ResetReason(sensor::ResetReason),
    Command(command::Command), 
    Log(Log),
}

#[common_derives]
#[serde(rename_all = "lowercase")]
pub enum CanData {
    Common(Common),
    Temperature(u8, f32), // sensor id, temperature
    Pressure(u8, f32),
    Strain(u8, f32),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[derive(From)]
#[serde(rename_all = "lowercase")]
pub enum RadioData<'a> {
    Common(Common),
    State(State),
    Sbg(sensor::SbgData),
    #[serde(borrow)]
    Gps(sensor::Gps<'a>),
}

impl CanMessage {
    pub fn new(timestamp: FormattedNaiveDateTime, node: Node, data: impl Into<CanData>) -> Self {
        CanMessage {
            timestamp,
            node,
            data: data.into(),
        }
    }
}

impl<'a> RadioMessage<'a> {
    pub fn new(timestamp: FormattedNaiveDateTime, node: Node, data: impl Into<RadioData<'a>>) -> Self {
        RadioMessage {
            timestamp,
            node,
            data: data.into(),
        }
    }
}

#[cfg(all(test, feature = "std"))]
mod test {
    use crate::{CanMessage, MAX_SIZE_CAN};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn can_message_size(msg: CanMessage) {
            let bytes = postcard::to_allocvec(&msg).unwrap();

            dbg!(msg.clone());

            assert!(bytes.len() <= MAX_SIZE_CAN);
        }
    }
}
