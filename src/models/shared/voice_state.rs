use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceState {
  pub nick: String,
  pub mute: bool,
  pub volume: u8,
  pub pan: VoicePan,
  #[serde(rename = "voice_state")]
  pub state: VoiceStateData,
  pub user: Option<Value>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoicePan {
  pub left: u8,
  pub right: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceStateData {
  pub mute: bool,
  pub deaf: bool,
  pub self_mute: bool,
  pub self_deaf: bool,
  pub suppress: bool,
}
