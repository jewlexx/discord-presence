use discord_presence::{Client, Event};

fn main() -> anyhow::Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::TRACE)
    //     .init();

    let mut drpc = Client::new(1003450375732482138);

    drpc.on_ready(|_ctx| {
        println!("ready?");
    })
    .persist();

    drpc.start();

    drpc.block_until_event(Event::Ready)?;

    assert!(Client::is_ready());

    // Set the activity
    // drpc.set_activity(|act| {
    //     act.state("rusting frfr")
    //         .append_buttons(|button| button.label("Click Me!").url("https://google.com/"))
    // })
    // .unwrap();

    drpc.shutdown()?;

    Ok(())
}
