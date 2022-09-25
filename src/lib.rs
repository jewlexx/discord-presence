//! This library provides easy access to the Discord IPC.
//!
//! It provides implementations for both Unix and Windows
//! operating systems, with both implementations using the
//! same API. Thus, this crate can be used in a platform-agnostic
//! manner.
//!
//! # Hello world
//! ```
//! use discord_ipc::{activity, DiscordIpc, DiscordIpcClient};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut client = DiscordIpcClient::new("<some client id>")?;
//!     client.connect()?;
//!
//!     let payload = activity::Activity::new().state("Hello world!");
//!     client.set_activity(payload)?;
//! }
//! ```
#![allow(missing_docs)]

mod discord_ipc;
mod pack_unpack;
mod rpc;
mod utils;

pub mod opcodes;
pub use discord_ipc::*;

/// get all stuff from here
pub use utils::*;

// events
pub use rpc::{Command, Event};

#[cfg(unix)]
mod ipc_unix;
#[cfg(unix)]
use ipc_unix as ipc;
use serde::{Deserialize, Serialize};

pub mod models;
use models::{commands::BasedCommandReturn, events::BasedEvent};

#[cfg(windows)]
mod ipc_windows;
#[cfg(windows)]
use ipc_windows as ipc;

pub use ipc::DiscordIpcClient;

/// Currently this is used to allow for matching of an event or type
/// Not all events/commands are implemented so serializing can fail
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EventReceive {
  Event(BasedEvent),
  CommandReturn(BasedCommandReturn),
}
