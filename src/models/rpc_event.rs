use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::{Value};

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


pub struct RPCTest {
}

impl RPCTest {
  // TODO: do something with this?
  // fn generate_base_payload() -> String {
  //   return String::from("test");
  // }

  /// send a command to sub to the SPEAKING_START event
  pub fn speaking_start_event(id: &str) -> String {
    let uuid = Uuid::new_v4().to_string();
    let raw_payload = serde_json::json!({
      "cmd": "SUBSCRIBE",
      "evt": "SPEAKING_START",
      "args": {
        "channel_id": id
      },
      "nonce": Value::String(uuid)
    });

    // println!("{:#?}", raw_payload);
    raw_payload.to_string()    
  }

  /// send a command to sub to the SPEAKING_STOP event
  pub fn speaking_stop_event(id: &str) -> String {
    let uuid = Uuid::new_v4().to_string();
    let raw_payload = serde_json::json!({
      "cmd": "SUBSCRIBE",
      "evt": "SPEAKING_STOP",
      "args": {
        "channel_id": id
      },
      "nonce": Value::String(uuid)
    });

    // println!("{:#?}", raw_payload);
    raw_payload.to_string()    
  }
}