
/// These are commands for discord
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RPCCommands {
  GetSelectedVoiceChannel,
  Subscribe,
  Unsubscribe,
}
