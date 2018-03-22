use std::process::id as pid;
use models::prelude::Command;

#[derive(Debug, Default, Serialize)]
pub struct SetActivityArgs {
    pub pid: u32,
    pub activity: SetActivity,
}

impl SetActivityArgs {
    pub fn command(args: SetActivity) -> Command<Self> {
        Command::new("SET_ACTIVITY", Self {
            pid: pid(),
            activity: args
        })
    }
}

message_format![SetActivity
    state:      String,
    details:    String,
    instance:   bool,
    timestamps: SetActivityTimestamps func,
    assets:     SetActivityAssets func,
    party:      SetActivityParty func,
    secrets:    SetActivitySecrets func,
];

message_format![SetActivityTimestamps
    start: u32,
    end: u32,
];

message_format![SetActivityAssets
    large_image: String,
    large_text: String,
    small_image: String,
    small_text: String,
];

message_format![SetActivityParty
    id: u32,
    size: (u32, u32),
];

message_format![SetActivitySecrets
    join: String,
    spectate: String,
    game: String alias = "match",
];

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_set_activity_serialize() {
        let activity = SetActivity::new()
            .state("rusting")
            .instance(true);
        let json = serde_json::to_string(&activity).unwrap();
        assert_eq![json, r#"{"instance":true,"state":"rusting"}"#];
    }

    #[test]
    fn test_set_activity_timestamps_serialize() {
        let timestamps = SetActivityTimestamps::new()
            .start(1000)
            .end(2000);
        let json = serde_json::to_string(&timestamps).unwrap();
        assert_eq![json, r#"{"end":2000,"start":1000}"#];
    }

    #[test]
    fn test_set_activity_assets_serialize() {
        let assets = SetActivityAssets::new()
            .large_image("ferris")
            .small_image("rusting");
        let json = serde_json::to_string(&assets).unwrap();
        assert_eq![json, r#"{"small_image":"rusting","large_image":"ferris"}"#];
    }

    #[test]
    fn test_set_activity_party_serialize() {
        let party = SetActivityParty::new()
            .id(1)
            .size((1, 10));
        let json = serde_json::to_string(&party).unwrap();
        assert_eq![json, r#"{"size":[1,10],"id":1}"#];
    }

    #[test]
    fn test_set_activity_secrets_serialize() {
        let secrets = SetActivitySecrets::new()
            .join("j1")
            .spectate("s1")
            .game("g1");
        let json = serde_json::to_string(&secrets).unwrap();
        assert_eq![json, r#"{"match":"g1","spectate":"s1","join":"j1"}"#.to_string()];
    }
}
