#![cfg(feature = "rich_presence")]

use std::default::Default;

use super::shared::PartialUser;
use utils;


#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SetActivityArgs {
    pid: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity: Option<Activity>,
}

impl SetActivityArgs {
    pub fn new<F>(f: F) -> Self
        where F: FnOnce(Activity) -> Activity
    {
        Self { pid: utils::pid(), activity: Some(f(Activity::new())) }
    }
}

impl Default for SetActivityArgs {
    fn default() -> Self {
        Self { pid: utils::pid(), activity: None }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SendActivityJoinInviteArgs {
    pub user_id: String,
}

pub type CloseActivityRequestArgs = SendActivityJoinInviteArgs;

impl SendActivityJoinInviteArgs {
    pub fn new(user_id: u64) -> Self {
        Self { user_id: user_id.to_string() }
    }
}

builder!{ActivityJoinEvent
    secret: String,
}

builder!{ActivitySpectateEvent
    secret: String,
}

builder!{ActivityJoinRequestEvent
    user: PartialUser,
}


builder!{Activity
    state: String,
    details: String,
    instance: bool,
    timestamps: ActivityTimestamps func,
    assets: ActivityAssets func,
    party: ActivityParty func,
    secrets: ActivitySecrets func,
}

builder!{ActivityTimestamps
    start: u64,
    end: u64,
}

builder!{ActivityAssets
    large_image: String,
    large_text: String,
    small_image: String,
    small_text: String,
}

builder!{ActivityParty
    id: u32,
    size: (u32, u32),
}

builder!{ActivitySecrets
    join: String,
    spectate: String,
    game: String alias = "match",
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    const FULL_JSON: &'static str =
r###"{
  "state": "rusting",
  "details": "detailed",
  "instance": true,
  "timestamps": {
    "start": 1000,
    "end": 2000
  },
  "assets": {
    "large_image": "ferris",
    "large_text": "Ferris",
    "small_image": "rusting",
    "small_text": "Rusting..."
  },
  "party": {
    "id": 1,
    "size": [
      3,
      6
    ]
  },
  "secrets": {
    "join": "025ed05c71f639de8bfaa0d679d7c94b2fdce12f",
    "spectate": "e7eb30d2ee025ed05c71ea495f770b76454ee4e0",
    "match": "4b2fdce12f639de8bfa7e3591b71a0d679d7c93f"
  }
}"###;

    #[test]
    fn test_serialize_full_activity() {
        let activity = Activity::new()
            .state("rusting")
            .details("detailed")
            .instance(true)
            .timestamps(|t| t
                .start(1000)
                .end(2000))
            .assets(|a| a
                .large_image("ferris")
                .large_text("Ferris")
                .small_image("rusting")
                .small_text("Rusting..."))
            .party(|p| p
                .id(1)
                .size((3, 6)))
            .secrets(|s| s
                .join("025ed05c71f639de8bfaa0d679d7c94b2fdce12f")
                .spectate("e7eb30d2ee025ed05c71ea495f770b76454ee4e0")
                .game("4b2fdce12f639de8bfa7e3591b71a0d679d7c93f"));

        let json = serde_json::to_string_pretty(&activity).unwrap();

        assert_eq![json, FULL_JSON];
    }

    #[test]
    fn test_serialize_empty_activity() {
        let activity = Activity::new();
        let json = serde_json::to_string(&activity).unwrap();
        assert_eq![json, "{}"];
    }
}
