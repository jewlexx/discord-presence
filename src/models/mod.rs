#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

mod commands;
mod events;
pub mod rpc_command;
pub mod rpc_event;
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

/// ex: evt, nonce
#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceStateUpdateData {
  pub evt: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "evt")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasedEvents {
  Ready {
    data: ReadyData,
  },
  Login {
    data: LoginData,
  },
  Error {
    data: ErrorData,
  },

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasedCommandsReturn {
  GetSelectedVoiceChannel { data: GetSelectedVoiceChannelData },
  SelectVoiceChannel { id: u32 },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectVoiceChannelArgs {
  id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasedCommands {
  GetSelectedVoiceChannel,
  SelectVoiceChannel { args: SelectVoiceChannelArgs },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RPCArg {
  // argless
  GetSelectedVoiceChannel,
  // takes args
  SetUserVoiceSettings { user_id: String, mute: bool },
  VoiceStateUpdate { channel_id: String },
}
