/// Event struct
pub struct Event;

use crate::{models::rpc_event::RPCEvent, utils::create_json};

/// allow you to create JSON payloads to send to the socket for subscribing to events
impl Event {
    /// create a json payload for the `SPEAKING_START` event
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

        create_json(json)
    }

    /// create a json payload for the `SPEAKING_STOP` event
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

        create_json(json)
    }

    // /// create a json payload for the GET_SELECTED_VOICE_CHANNEL command
    // ///
    // /// used to get the current voice channel the client is in
    // pub fn get_selected_voice_channel() -> String {
    //   // let uuid = Uuid::new_v4().to_string();
    //   // let raw_payload = serde_json::json!({
    //   //   "cmd": "GET_SELECTED_VOICE_CHANNEL",
    //   //   "evt": null,
    //   //   "nonce": Value::String(uuid)
    //   // });

    //   // raw_payload.to_string()

    //   let json = serde_json::json!({
    //     "cmd": RPCCommand::GetSelectedVoiceChannel,
    //     "evt": null
    //   });

    //   create_json(json)
    // }
}
