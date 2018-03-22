extern crate discord_rpc_client;

use std::{thread, time};
use discord_rpc_client::Client as DiscordRPC;

fn main() {
    let mut drpc =
        DiscordRPC::new(425407036495495169)
            .and_then(|rpc| rpc.start())
            .expect("Failed to start client");

    drpc.set_activity(|a| a
        .state("Rusting")
        .assets(|ass| ass
            .large_image("ferris_wat")
            .large_text("wat.")
            .small_image("rusting")
            .small_text("rusting...")))
        .expect("Failed to set presence");

    loop { thread::sleep(time::Duration::from_secs(10)) };
}
