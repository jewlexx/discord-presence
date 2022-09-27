/// Command struct
pub struct Command;

use crate::{models::rpc_command::RPCCommand, utils::create_json};

/// allow you to create JSON payloads to send commands
impl Command {
    /// create a json payload for the `GET_SELECTED_VOICE_CHANNEL` command
    ///
    /// used to get the current voice channel the client is in
    pub fn get_selected_voice_channel() -> String {
        let json = serde_json::json!({
          "cmd": RPCCommand::GetSelectedVoiceChannel,
          "evt": null
        });

        create_json(json)
    }

    /// create a json payload for the `SELECT_VOICE_CHANNEL` command
    ///
    /// used to get set the clients current channel
    pub fn select_voice_channel(id: &str) -> String {
        let json = serde_json::json!({
          "cmd": RPCCommand::SelectVoiceChannel,
          "args": {
            "channel_id": id
          },
          "evt": null
        });

        create_json(json)
    }
}
