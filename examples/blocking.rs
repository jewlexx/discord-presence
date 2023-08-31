use discord_presence::{Client, Event};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let mut drpc = Client::new(1003450375732482138);

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

    drpc.start();

    drpc.block_until_event(Event::Ready)?;

    assert!(Client::is_ready());

    // Set the activity
    drpc.set_activity(|act| act.state("rusting frfr"))?;

    // TODO: Implement "remote" shutdown
    // ctrlc::set_handler(move || {
    //     println!("Exiting...");
    //     drpc.clear_activity().unwrap();
    //     std::process::exit(0);
    // })?;

    drpc.block_on()?;

    Ok(())
}
