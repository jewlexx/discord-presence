use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SpeakingData {
  pub channel_id: String,
  pub user_id: String,
}
