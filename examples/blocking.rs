use discord_presence::{Client as DiscordRPC, Event};

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let mut drpc = DiscordRPC::new(1003450375732482138);

    let drpc_thread = drpc.start();

    drpc.block_until_event(Event::Ready).unwrap();

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
