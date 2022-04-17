use std::sync::{Arc, Mutex};

use bevy::{log::prelude::*, prelude::*};
use discord_presence::{
    models::{ActivityAssets, ActivityParty, ActivitySecrets, ActivityTimestamps},
    *,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct RPCConfig {
    pub client_id: u64,
    pub show_time: bool,
}

impl Default for RPCConfig {
    fn default() -> Self {
        Self {
            client_id: 425407036495495169,
            show_time: true,
        }
    }
}

#[derive(Default, Debug, Hash, Eq, PartialEq, Clone)]
pub struct ActivityState {
    pub state: Option<String>,
    pub details: Option<String>,
    pub instance: Option<bool>,
    pub timestamps: Option<ActivityTimestamps>,
    pub assets: Option<ActivityAssets>,
    pub party: Option<ActivityParty>,
    pub secrets: Option<ActivitySecrets>,
}

pub struct RPCPlugin(pub RPCConfig);

pub struct RPCResource {
    pub client: Arc<Mutex<Client>>,
}

impl FromWorld for RPCResource {
    fn from_world(world: &mut World) -> Self {
        let config = world.get_resource::<RPCConfig>();
        match config {
            Some(config) => RPCResource {
                client: Arc::new(Mutex::new(Client::new(config.client_id))),
            },
            None => RPCResource {
                client: Arc::new(Mutex::new(Client::new(425407036495495169))),
            },
        }
    }
}

impl Plugin for RPCPlugin {
    fn build(&self, app: &mut App) {
        let client_config = self.0.clone();

        app.add_startup_system(startup_client);
        app.add_system(update_presence);
        debug!("Added systems");

        app.insert_resource::<RPCConfig>(client_config);
        app.init_resource::<RPCResource>();
        debug!("Initialized resources");

        app.add_state(ActivityState::default());
    }
}

fn startup_client(client: ResMut<RPCResource>) {
    let mut client = client.client.lock().unwrap();

    let is_ready = Arc::new(Mutex::new(false));
    let error = Arc::new(Mutex::<Option<Value>>::new(None));

    client.on_ready(move |_| {
        let is_ready = Arc::clone(&is_ready);
        *is_ready.lock().unwrap() = true;
    });

    client.on_error(move |e| {
        let error = Arc::clone(&error);
        *error.lock().unwrap() = Some(e.event);
    });

    let res = client.set_activity(|e| e.state("poggies"));

    match res {
        Ok(_) => {}
        Err(why) => {
            error!("Failed to set presence: {}", why);
        }
    }

    client.start();
}

fn update_presence(client: ResMut<RPCResource>, client_config: Res<RPCConfig>) {
    let mut client = client.client.lock().unwrap();
}
