mod message;
mod command;
mod handshake;
#[cfg(feature = "rich_presence")]
mod set_activity;

use serde::Serialize;
pub use self::message::{Message, OpCode};
pub use self::command::Command;
pub use self::handshake::Handshake;
#[cfg(feature = "rich_presence")]
pub use self::set_activity::*;

pub trait Payload: Serialize {}

pub mod prelude {
    pub use super::set_activity::{
        SetActivity,
        SetActivityAssets,
        SetActivityParty,
        SetActivitySecrets,
        SetActivityTimestamps
    };
}
