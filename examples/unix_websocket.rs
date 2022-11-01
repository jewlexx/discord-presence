//! Temporary example to test the websocket server.

use discord_presence::Client as DiscordRPC;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let mut drpc = DiscordRPC::new(1003450375732482138);

    drpc.on_ready(|_ctx| {
        println!("ready?");
    });

    drpc.on_activity_join_request(|ctx| {
        println!("Join request: {:?}", ctx.event);
    });

    drpc.on_activity_join(|ctx| {
        println!("Joined: {:?}", ctx.event);
    });

    drpc.on_activity_spectate(|ctx| {
        println!("Spectate: {:?}", ctx.event);
    });

    let drpc_thread = drpc.start();

    // Set the activity
    drpc.set_activity(|act| act.state("rusting frfr"))
        .expect("Failed to set activity");

    ctrlc::set_handler(move || {
        println!("Exiting...");
        drpc.clear_activity().unwrap();
        std::process::exit(0);
    })
    .unwrap();

    drpc_thread.join().unwrap();
}
