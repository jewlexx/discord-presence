use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::{Value};

use super::rpc_command::RPCCommand;

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


pub struct Event;

impl Event {
  pub fn create_json(mut value: serde_json::Value) -> String {    
    let uuid = Uuid::new_v4().to_string();
    
    let payload = value.as_object_mut().unwrap();
    payload.insert("nonce".to_string(), Value::String(uuid));

    // TODO: RISKY NEED TO FIX ERROR HANDLING
    serde_json::to_string(&payload).unwrap()
  }

  /// create a json payload for the SPEAKING_START event
  /// which will subscribe to the channel supplied
  /// 
  /// Arguments: 
  /// * `id`: channel id to join
  pub fn speaking_start_event(id: &str) -> String {
    let json = serde_json::json!({
      "cmd": "SUBSCRIBE",
      "evt": RPCEvent::SpeakingStart,
      "args": {
        "channel_id": id
      },
    });
    
    Self::create_json(json)
  }

  /// create a json payload for the SPEAKING_STOP event
  /// which will subscribe to the channel supplied
  /// 
  /// Arguments: 
  /// * `id`: channel id to join
  pub fn speaking_stop_event(id: &str) -> String {
    let json = serde_json::json!({
      "cmd": "SUBSCRIBE",
      "evt": RPCEvent::SpeakingStop,
      "args": {
        "channel_id": id
      },
    });
    
    Self::create_json(json)
  }

  /// create a json payload for the GET_SELECTED_VOICE_CHANNEL command
  /// 
  /// used to get the current voice channel the client is in
  pub fn get_selected_voice_channel() -> String {
    // let uuid = Uuid::new_v4().to_string();
    // let raw_payload = serde_json::json!({
    //   "cmd": "GET_SELECTED_VOICE_CHANNEL",
    //   "evt": null,
    //   "nonce": Value::String(uuid)
    // });

    // raw_payload.to_string()    

    let json = serde_json::json!({
      "cmd": RPCCommand::GetSelectedVoiceChannel,
      "evt": null
    });
    
    Self::create_json(json)
  }
}