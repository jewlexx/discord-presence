use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SelectedChannelData {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub topic: String,
    pub bitrate: i64,
    pub user_limit: i64,
    pub guild_id: String,
    pub position: i64,
    pub messages: Value,
}
