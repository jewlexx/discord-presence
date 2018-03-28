mod message;
mod command;
mod handshake;

use serde::Serialize;
pub use self::message::{Message, OpCode};
pub use self::command::Command;
pub use self::handshake::Handshake;

pub trait Payload: Serialize {}
