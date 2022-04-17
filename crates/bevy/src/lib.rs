use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use discord_presence::{models::Activity, *};
use serde_json::Value;

pub struct RPCConfig {
    pub client_id: u64,
    pub show_time: bool,
}

pub struct RPCPlugin(RPCConfig);

impl Plugin for RPCPlugin {
    fn build(&self, app: &mut App) {
        let is_ready = Arc::new(Mutex::new(false));
        let error = Arc::new(Mutex::<Option<Value>>::new(None));
        let client_config = &self.0;
        let mut client = Client::new(client_config.client_id);

        client.on_ready(move |_| {
            let is_ready = Arc::clone(&is_ready);
            *is_ready.lock().unwrap() = true;
        });

        client.on_error(move |e| {
            let error = Arc::clone(&error);
            *error.lock().unwrap() = Some(e.event);
        });

        app.add_state("DiscordRPC");

        client.set_activity(|e| {
            let updated_activity = e.clone();

            updated_activity
        });

        client.start();
    }
}
