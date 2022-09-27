use serde::{Deserialize, Serialize};

use crate::models::shared::VoiceState;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelData {
  pub id: String,
  pub name: String,
  /// This is the "type" field that comes from the discord api
  /// but we have to rename this shit cause rust is "good"
  #[serde(rename = "type")]
  pub event_type: u32,
  pub topic: String,
  pub bitrate: u32,
  pub user_limit: u32,
  pub guild_id: String,
  pub position: u32,
  pub voice_states: Vec<VoiceState>,
}
