use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceChannelConfig {
  pub test: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceChannelData {
  pub config: VoiceChannelConfig
}
