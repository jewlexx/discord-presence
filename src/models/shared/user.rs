use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub id: String,
  pub username: String,
  pub discriminator: String,
  pub avatar: String,
  pub avatar_decoration: Option<String>,
  pub bot: bool,
  pub flags: u8,
  pub premium_type: Option<u32>,
}
