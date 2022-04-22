#![warn(missing_docs)]

//! A Bevy plugin that allows the developer to interact with the Discord Presence API with ease
//!
//! This plugin is a Bevy wrapper around the [Discord Presence](https://docs.rs/crate/discord-presence) crate which in turn is a wrapper around the [Discord Presence API](https://discordapp.com/developers/docs/game-sdk/discord-presence).
//! # Examples
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_discord_presence::{ActivityState, RPCConfig, RPCPlugin};
//!
//! fn main() {
//!     println!("hello world!");
//!     let mut app = App::new();
//!     app.add_plugins(DefaultPlugins);
//!     app.add_plugin(RPCPlugin(RPCConfig {
//!         app_id: 425407036495495169,
//!         show_time: true,
//!     }));
//!     app.add_system(update_presence);
//!
//!     app.run();
//! }
//!
//! fn update_presence(mut state: ResMut<ActivityState>) {
//!     state.details = Some("Hello World".to_string());
//! }
//! ```

use std::{
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::{log::prelude::*, prelude::*};
use discord_presence::{
    models::{Activity, ActivityAssets, ActivityParty, ActivitySecrets, ActivityTimestamps},
    Client,
};

/// Configuration for the RPC plugin
#[derive(Clone)]
pub struct RPCConfig {
    /// The Discord application ID
    pub app_id: u64,
    /// Whether to show the current time in the activity
    pub show_time: bool,
}

impl Default for RPCConfig {
    fn default() -> Self {
        Self {
            app_id: 425407036495495169,
            show_time: true,
        }
    }
}

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

// TODO: Add guide on how to get `app_id`

/// The main RPC plugin
///
/// # Arguments
///
/// * `config` - The configuration for the plugin. Vital field is `app_id`, as the Discord interactions cannot work without it.
pub struct RPCPlugin(pub RPCConfig);

/// The resource that holds the Discord Client
pub struct RPCResource {
    /// The actual Discord client used to interact with Discord APIs
    pub client: Arc<Mutex<Client>>,
}

impl FromWorld for RPCResource {
    fn from_world(world: &mut World) -> Self {
        let config = world.get_resource::<RPCConfig>().unwrap();
        Self {
            client: Arc::new(Mutex::new(Client::new(config.app_id))),
        }
    }
}

/// Implements the Bevy plugin trait
impl Plugin for RPCPlugin {
    fn build(&self, app: &mut App) {
        let client_config = self.0.clone();

        app.add_startup_system(startup_client);
        app.add_system(check_activity_changed);
        debug!("Added systems");

        app.insert_resource::<RPCConfig>(client_config);

        app.init_resource::<ActivityState>();
        app.init_resource::<RPCResource>();

        debug!("Initialized resources");
    }

    fn name(&self) -> &str {
        "RPCPlugin"
    }
}

/// Initializes the client and starts it running
fn startup_client(
    mut activity: ResMut<ActivityState>,
    client: ResMut<RPCResource>,
    config: Res<RPCConfig>,
) {
    let mut client = client.client.lock().unwrap();

    if config.show_time {
        activity.timestamps = Some(ActivityTimestamps {
            start: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ),
            end: None,
        });
    }

    client.on_ready(move |_| {
        debug!("Client is ready");
    });

    client.on_error(move |e| {
        debug!("Client error: {:?}", e);
    });

    client.start();
    debug!("Client has started");
}

/// Runs whenever the activity has been changed, and at startup
fn check_activity_changed(activity: Res<ActivityState>, client: ResMut<RPCResource>) {
    if activity.is_changed() {
        let mut client = client.client.lock().unwrap();

        let res = client.set_activity(|_| activity.clone().into());

        if let Err(why) = res {
            error!("Failed to set presence: {}", why);
        }
    }
}
