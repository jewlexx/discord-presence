#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

mod commands;
mod events;
mod shared;

// event types
use events::*;

use self::commands::*;

// commands types
// use commands::*;

/// Includes the base props from discord
/// ex: evt, nonce
#[derive(Serialize, Deserialize, Debug)]
pub struct Based {
  pub cmd: Option<String>,
  pub nonce: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "evt")]
#[serde(rename_all = "UPPERCASE")]
pub enum BasedEvents {
  Ready {
    #[serde(flatten)]
    default: Based,
    data: ReadyData,
  },
  Login {
    #[serde(flatten)]
    default: Based,
    data: LoginData,
  },
  Error {
    #[serde(flatten)]
    default: Based,
    data: ErrorData,
  },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasedCommandsReturn {
  GetSelectedVoiceChannel  {
    data: GetSelectedVoiceChannelData
  },
  SelectVoiceChannel { id: u32 },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasedCommands {
  GetSelectedVoiceChannel,
  SelectVoiceChannel { id: String },
}
