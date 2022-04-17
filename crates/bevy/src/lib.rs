use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use discord_presence::*;
use serde_json::Value;

#[derive(Default, Debug, Clone)]
pub struct RPCConfig {
    pub client_id: u64,
    pub show_time: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ActivityState {
    Message(String),
    Blank,
}

pub struct RPCPlugin(RPCConfig);

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

        app.init_resource::<RPCResource>();
        app.insert_resource(client_config);

        app.add_state(ActivityState::Message("DiscordRPC".into()));
    }
}

fn startup_client(client: Res<RPCResource>) {
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
            println!("Failed to set presence: {}", why);
        }
    }

    client.start();
}
