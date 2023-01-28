use discord_presence::Client;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let mut client = Client::new(1003450375732482138);

    let client_handle = client.start();

    let handle = client.on_ready(|_| {
        tracing::info!("Discord client is ready!");
    });

    client_handle.join().unwrap();
}
