use serde::{Deserialize, Serialize};

use crate::models::commands::SpeakingData;

use super::error::ErrorData;
use super::login::LoginData;
use super::ready::ReadyData;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "evt")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasedEvent {
  Ready { data: ReadyData },
  Login { data: LoginData },
  Error { data: ErrorData },

  // speaking start
  SpeakingStart {
    data: SpeakingData
  },
  // speaking stop
  SpeakingStop {
    data: SpeakingData
  },
}
