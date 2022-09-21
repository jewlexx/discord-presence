use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginConfig {
  pub test: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginData {
  pub config: LoginConfig,
}
