use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use discord_presence::Client;

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
