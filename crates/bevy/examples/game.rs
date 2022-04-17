use bevy::prelude::*;

use bevy_discord_presence::{RPCConfig, RPCPlugin};

fn main() {
    println!("hello world!");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(RPCPlugin(RPCConfig {
        app_id: 965125975941709834,
        show_time: true,
    }));

    app.run();
}
