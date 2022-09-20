use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub cmd: String,
    pub data: Data,
    pub evt: String,
    pub nonce: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub config: Config,
    pub user: User,
    pub v: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub api_endpoint: String,
    pub cdn_host: String,
    pub environment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub avatar: String,
    pub bot: bool,
    pub discriminator: String,
    pub flags: i64,
    pub id: String,
    pub premium_type: i64,
    pub username: String,
}



/// ready event
