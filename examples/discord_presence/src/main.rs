extern crate simplelog;
extern crate discord_rpc_client;

use std::io;
use simplelog::*;
use discord_rpc_client::{
    Client as DiscordRPC,
    models::Event,
};

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    let mut drpc = DiscordRPC::new(425407036495495169)
        .expect("Failed to create client");

    drpc.start();

    drpc.subscribe(Event::ActivityJoin, |j| j
        .secret("123456"))
        .expect("Failed to subscribe to event");

    drpc.subscribe(Event::ActivitySpectate, |s| s
        .secret("123456"))
        .expect("Failed to subscribe to event");

    drpc.subscribe(Event::ActivityJoinRequest, |s| s)
        .expect("Failed to subscribe to event");

    drpc.unsubscribe(Event::ActivityJoinRequest, |j| j)
        .expect("Failed to unsubscribe from event");

    loop {
        let mut buf = String::new();

        io::stdin().read_line(&mut buf).unwrap();


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
    };
}
