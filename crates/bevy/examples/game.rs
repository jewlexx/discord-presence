use bevy::prelude::*;

use bevy_discord_presence::RPCPlugin;

fn main() {
    bevy::log::debug!("Hello, world!");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(RPCPlugin(Default::default()));

    app.run();
}
