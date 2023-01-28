use discord_presence::Client as DiscordRPC;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let mut drpc = DiscordRPC::new(1003450375732482138);

    let _ready = drpc.on_ready(|_ctx| {
        println!("ready?");
    });

    let _activity_join_request = drpc.on_activity_join_request(|ctx| {
        println!("Join request: {:?}", ctx.event);
    });

    let _activity_join = drpc.on_activity_join(|ctx| {
        println!("Joined: {:?}", ctx.event);
    });

    let _activity_spectate = drpc.on_activity_spectate(|ctx| {
        println!("Spectate: {:?}", ctx.event);
    });

    let drpc_thread = drpc.start();

    drpc_thread.join().unwrap()
}
