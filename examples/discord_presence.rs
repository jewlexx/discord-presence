extern crate simplelog;
extern crate discord_rpc_client;

use std::io;
use simplelog::*;
use discord_rpc_client::Client as DiscordRPC;

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    let mut drpc = DiscordRPC::new(425407036495495169);

    drpc.start();

    loop {
        let mut buf = String::new();

        io::stdin().read_line(&mut buf).unwrap();
        buf.pop();

        if buf.is_empty() {
            if let Err(why) = drpc.clear_activity() {
                println!("Failed to clear presence: {}", why);
            }
        } else {
            if let Err(why) = drpc.set_activity(|a| a
                .state(buf)
                .assets(|ass| ass
                    .large_image("ferris_wat")
                    .large_text("wat.")
                    .small_image("rusting")
                    .small_text("rusting...")))
            {
                println!("Failed to set presence: {}", why);
            }
        }
    };
}
