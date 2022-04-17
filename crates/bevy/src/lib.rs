use std::{
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::{log::prelude::*, prelude::*};
use discord_presence::{
    models::{ActivityAssets, ActivityParty, ActivitySecrets, ActivityTimestamps},
    Client,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct RPCConfig {
    pub app_id: u64,
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
        let config = world.get_resource::<RPCConfig>().unwrap();
        Self {
            client: Arc::new(Mutex::new(Client::new(config.app_id))),
        }
    }
}

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
}

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

    let is_ready = Arc::new(Mutex::new(false));
    let error = Arc::new(Mutex::<Option<Value>>::new(None));

    client.on_ready(move |_| {
        debug!("Client is ready");
        let is_ready = Arc::clone(&is_ready);
        *is_ready.lock().unwrap() = true;
    });

    client.on_error(move |e| {
        debug!("Client error: {:?}", e);
        let error = Arc::clone(&error);
        *error.lock().unwrap() = Some(e.event);
    });

    client.start();
    debug!("Client has started");
}

fn check_activity_changed(activity: Res<ActivityState>, client: ResMut<RPCResource>) {
    if activity.is_changed() {
        let mut client = client.client.lock().unwrap();

        let res = client.set_activity(|mut e| {
            e.state = activity.state.clone();
            e.assets = activity.assets.clone();
            e.details = activity.details.clone();
            e.party = activity.party.clone();
            e.secrets = activity.secrets.clone();
            e.timestamps = activity.timestamps.clone();
            e.instance = activity.instance;

            e
        });

        if let Err(why) = res {
            error!("Failed to set presence: {}", why);
        }
    }
}
