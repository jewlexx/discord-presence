use serde::{Deserialize, Serialize};

use crate::models::shared::User;
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyEvent {
  pub cmd: String,
  pub data: ReadyData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub cdn_host: String,
  pub api_endpoint: String,
  pub environment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyData {
  pub v: u32,
  pub config: Config,
  pub user: User,
}
