use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub cdn_host: String,
  pub api_endpoint: String,
  pub environment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub id: String,
  pub username: String,
  pub discriminator: String,
  pub avatar: String,
  pub avatar_decoration: Option<String>,
  pub bot: bool,
  pub flags: u8,
  pub premium_type: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyData {
  pub v: u32,
  pub config: Config,
  pub user: User,
}
