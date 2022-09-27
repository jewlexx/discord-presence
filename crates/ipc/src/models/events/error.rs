use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorData {
  pub code: u32,
  pub message: String,
}
