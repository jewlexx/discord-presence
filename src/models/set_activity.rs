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
