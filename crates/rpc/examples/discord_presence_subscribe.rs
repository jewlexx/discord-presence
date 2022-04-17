use discord_rpc::{models::Event, Client as DiscordRPC};
use simplelog::*;
use std::{thread, time};

fn main() {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let mut drpc = DiscordRPC::new(425407036495495169);

    drpc.on_activity_join_request(|ctx| {
        println!("Join request: {:?}", ctx.event);
    });

    drpc.on_activity_join(|ctx| {
        println!("Joined: {:?}", ctx.event);
    });

    drpc.on_activity_spectate(|ctx| {
        println!("Spectate: {:?}", ctx.event);
    });

    drpc.start();

    drpc.subscribe(Event::ActivityJoin, |j| j.secret("123456"))
        .expect("Failed to subscribe to event");

    drpc.subscribe(Event::ActivitySpectate, |s| s.secret("123456"))
        .expect("Failed to subscribe to event");

    drpc.subscribe(Event::ActivityJoinRequest, |s| s)
        .expect("Failed to subscribe to event");

    drpc.unsubscribe(Event::ActivityJoinRequest, |j| j)
        .expect("Failed to unsubscribe from event");

    loop {
        thread::sleep(time::Duration::from_millis(500));
    }
}
