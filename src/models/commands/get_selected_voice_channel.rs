use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSelectedVoiceChannelData {
  pub channel_id: String,
}
