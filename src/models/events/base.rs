use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::commands::SpeakingData;

// use super::selected_channel::SelectedChannelData;
use super::error::ErrorData;
use super::login::LoginData;
use super::ready::ReadyData;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "evt")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasedEvent {
  Ready { data: ReadyData },
  Login { data: LoginData },
  Error { data: ErrorData },

  /// speaking start
  SpeakingStart { data: SpeakingData },
  /// speaking stop
  SpeakingStop { data: SpeakingData },

  // TODO: type these payloads
  GetSelelectedVoiceChannel {
    data: HashMap<String, Value>,
  },
  VoiceStateUpdate {
    data: HashMap<String, Value>,
  },
  VoiceStateCreate {
    data: HashMap<String, Value>,
  },
  VoiceStateDelete {
    data: HashMap<String, Value>,
  },
  VoiceChannelSelect {
    data: HashMap<String, Value>,
  }
}
