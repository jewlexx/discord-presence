extern crate simplelog;
extern crate discord_rpc_client;

use std::{thread, time};
use simplelog::*;
use discord_rpc_client::{
    Client as DiscordRPC,
    models::Event,
};

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    let mut drpc = DiscordRPC::new(425407036495495169);

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

    loop { thread::sleep(time::Duration::from_millis(500)); }
}
