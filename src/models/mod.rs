#![allow(missing_docs)]
use serde::{ Serialize, Deserialize };

// TODO: move this to the commands mod
mod login;
use login::*;

mod commands;
mod events;

// event types
use events::*;

// commands types
use commands::*;

/// Includes the base props from discord
/// ex: evt, nonce
#[derive(Serialize, Deserialize, Debug)]
pub struct Based {
  pub cmd: Option<String>,
  pub nonce: Option<String>
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
#[serde(rename_all = "UPPERCASE")]
pub enum BasedCommands {  
  GetSelectedVoiceChannel {
    #[serde(flatten)]
    default: Based,
    data: GetSelectedVoiceChannelData,
  }
}
