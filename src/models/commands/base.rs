use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{ChannelData, SpeakingData};


/// Currently this handles all the received commands from the discord socket
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasedCommandReturn {
  GetSelectedVoiceChannel { data: ChannelData },

  /// Get the selected voice channel
  SelectVoiceChannel { data: ChannelData },

  /// Subscribe
  Subscribe { data: HashMap<String, String> },
  /// Dispatch
  Dispatch { data: HashMap<String, String> },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectVoiceChannelArgs {
  id: u64,
}
