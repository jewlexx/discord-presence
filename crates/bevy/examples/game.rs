use bevy::prelude::*;

use bevy_discord_presence::{RPCConfig, RPCPlugin};

fn main() {
    println!("hello world!");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(RPCPlugin(RPCConfig {
        app_id: 425407036495495169,
        show_time: true,
    }));

    app.run();
}
