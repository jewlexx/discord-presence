use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::GetSelectedVoiceChannelData;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasedCommandReturn {
  GetSelectedVoiceChannel { data: GetSelectedVoiceChannelData },
  SelectVoiceChannel { id: u32 },
  Subscribe {
    data: HashMap<String, String>
  },
  Dispatch {
    data: HashMap<String, String>
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectVoiceChannelArgs {
  id: u64,
}
