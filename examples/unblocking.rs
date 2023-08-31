use discord_presence::Client;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let mut client = Client::new(1003450375732482138);

    client.start();

    tracing::error!("Due to the way unblocking activity setting works, this example does not seem to work currently (at least on Windows).");

    client
        .set_activity(|a| {
            a.state("Rust")
                .details("Programming")
                .assets(|a| a.large_image("rust"))
        })
        .unwrap();

    tracing::trace!("Made it to the final line");
}
