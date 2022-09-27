use serde::{Deserialize, Serialize};

// TODO: move this to somewhere else
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RPCEvent {
  CurrentUserUpdate,
  VoiceChannelSelect,
  VoiceStateCreate,
  VoiceStateDelete,
  VoiceStateUpdate,
  VoiceSettingsUpdate,
  VoiceConnectionStatus,
  SpeakingStart,
  SpeakingStop,
  Ready,
  Error,
}
