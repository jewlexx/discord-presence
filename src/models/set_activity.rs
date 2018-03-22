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

#[derive(Debug, Default, Serialize)]
pub struct SetActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<ActivityTimestamps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<ActivityAssets>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<ActivityParty>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<ActivitySecrets>,
}

impl SetActivity {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state<S>(mut self, s: S) -> Self
        where S: Into<String>
    {
        self.state = Some(s.into());
        self
    }

    pub fn details<S>(mut self, d: S) -> Self
        where S: Into<String>
    {
        self.details = Some(d.into());
        self
    }

    pub fn instance(mut self, i: bool) -> Self {
        self.instance = Some(i);
        self
    }

    pub fn timestamps<F>(mut self, f: F) -> Self
        where F: FnOnce(ActivityTimestamps) -> ActivityTimestamps
    {
        self.timestamps = Some(f(ActivityTimestamps::default()));
        self
    }

    pub fn assets<F>(mut self, f: F) -> Self
        where F: FnOnce(ActivityAssets) -> ActivityAssets
    {
        self.assets = Some(f(ActivityAssets::default()));
        self
    }

    pub fn party<F>(mut self, f: F) -> Self
        where F: FnOnce(ActivityParty) -> ActivityParty
    {
        self.party = Some(f(ActivityParty::default()));
        self
    }

    pub fn secrets<F>(mut self, f: F) -> Self
        where F: FnOnce(ActivitySecrets) -> ActivitySecrets
    {
        self.secrets = Some(f(ActivitySecrets::default()));
        self
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ActivityTimestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u32>,
}

impl ActivityTimestamps {
    pub fn start(mut self, i: u32) -> Self {
        self.start = Some(i);
        self
    }

    pub fn end(mut self, i: u32) -> Self {
        self.end = Some(i);
        self
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ActivityAssets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
}

impl ActivityAssets {
    pub fn large_image<S>(mut self, i: S) -> Self
        where S: Into<String>
    {
        self.large_image = Some(i.into());
        self
    }

    pub fn large_text<S>(mut self, t: S) -> Self
        where S: Into<String>
    {
        self.large_text = Some(t.into());
        self
    }

    pub fn small_image<S>(mut self, i: S) -> Self
        where S: Into<String>
    {
        self.small_image = Some(i.into());
        self
    }

    pub fn small_text<S>(mut self, t: S) -> Self
        where S: Into<String>
    {
        self.small_text = Some(t.into());
        self
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ActivityParty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<[u32; 2]>,
}

impl ActivityParty {
    pub fn id(mut self, i: u32) -> Self {
        self.id = Some(i);
        self
    }

    pub fn size(mut self, current: u32, max: u32) -> Self {
        self.size = Some([current, max]);
        self
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ActivitySecrets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectate: Option<String>,

    // NOTE: think of a better name for this
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "match")]
    pub shoubu: Option<String>,
}

impl ActivitySecrets {
    pub fn join<S>(mut self, secret: S) -> Self
        where S: Into<String>
    {
        self.join = Some(secret.into());
        self
    }

    pub fn spectate<S>(mut self, secret: S) -> Self
        where S: Into<String>
    {
        self.spectate = Some(secret.into());
        self
    }

    pub fn game<S>(mut self, secret: S) -> Self
        where S: Into<String>
    {
        self.shoubu = Some(secret.into());
        self
    }
}
