use discord_presence::models::{
    Activity, ActivityAssets, ActivityParty, ActivitySecrets, ActivityTimestamps,
};

/// The state that holds the Discord activity
#[derive(Default, Clone)]
pub struct ActivityState {
    /// The player's current party status
    pub state: Option<String>,
    /// What the player is currently doing
    pub details: Option<String>,
    /// Whether this activity is an instanced context, like a match
    pub instance: Option<bool>,
    /// Helps create elapsed/remaining timestamps on a player's profile
    pub timestamps: Option<ActivityTimestamps>,
    /// Assets to display on the player's profile
    pub assets: Option<ActivityAssets>,
    /// Information about the player's party. NOTE: Joining a party is not currently supported
    pub party: Option<ActivityParty>,
    /// Secret passwords for joining and spectating the player's game. NOTE: Joining a party is not currently supported
    pub secrets: Option<ActivitySecrets>,
}

impl From<ActivityState> for Activity {
    /// Converts the ActivityState into a Discord Presence
    fn from(state: ActivityState) -> Self {
        Activity {
            state: state.state,
            assets: state.assets,
            details: state.details,
            party: state.party,
            secrets: state.secrets,
            timestamps: state.timestamps,
            instance: state.instance,
        }
    }
}
