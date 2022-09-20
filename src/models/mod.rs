
use serde::{ Serialize, Deserialize };
mod ready;
mod login;

use ready::*;
use login::*;

/// Includes the base props from discord
/// ex: evt, nonce
#[derive(Serialize, Deserialize, Debug)]
pub struct Based {
  pub cmd: String,
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
  VoiceChannel {
    #[serde(flatten)]
    default: Based,
    data: LoginData,    
  },  
  Login {
    #[serde(flatten)]
    default: Based,
    data: LoginData,    
  },  
  Error {
    #[serde(flatten)]
    default: Based,
    data: Value,    
  },  
}
