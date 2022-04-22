use bevy::prelude::*;

use bevy_discord_presence::{
    config::{RPCConfig, RPCPlugin},
    state::ActivityState,
};

fn main() {
    println!("hello world!");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(RPCPlugin(RPCConfig {
        app_id: 425407036495495169,
        show_time: true,
    }));
    app.add_system(update_presence);

    app.run();
}

fn update_presence(mut state: ResMut<ActivityState>) {
    state.instance = Some(true);
    state.details = Some("Hello World".to_string());
    state.state = Some("This is state".to_string());
}
