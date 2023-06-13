use discord_presence::Client;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let mut drpc = Client::new(1003450375732482138);

    drpc.on_ready(|_ctx| {
        println!("READY!");
    });

    drpc.on_error(|ctx| {
        eprintln!("An error occured, {}", ctx.event);
    });

    let drpc_thread = drpc.start();

    if let Err(why) = drpc.set_activity(|a| {
        a.state("Running examples").assets(|ass| {
            ass.large_image("ferris_wat")
                .large_text("wat.")
                .small_image("rusting")
                .small_text("rusting...")
        })
    }) {
        println!("Failed to set presence: {}", why);
    }

    drpc_thread.join().unwrap()
}
