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

pub mod opcodes;
pub use discord_ipc::*;

#[cfg(unix)]
mod ipc_unix;
#[cfg(unix)]
use ipc_unix as ipc;
use serde::{Deserialize, Serialize};

pub mod models;
use models::*;

#[cfg(windows)]
mod ipc_windows;
#[cfg(windows)]
use ipc_windows as ipc;

pub use ipc::DiscordIpcClient;

/// Not used anymore?
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EventType {
  Command(BasedCommands),
  Event(BasedEvents),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EventReceive {
  Event(BasedEvents),
  CommandReturn(BasedCommandsReturn),
}
