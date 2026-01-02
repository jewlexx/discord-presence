use std::time::{Duration, SystemTime, UNIX_EPOCH};

use discord_presence::{models::ActivityTimestamps, Client, Event};

mod helpers;

fn main() -> anyhow::Result<()> {
    helpers::logging::init_logging();

    // wait until the rpc client is ready
    let mut client = Client::new(1286481105410588672);
    client.start();
    client.block_until_event(Event::Ready)?;

    // set an activity every second for 30 seconds
    // we expect that it only updates ~every 15 seconds
    let timestamps =
        ActivityTimestamps::new().start(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs());
    for i in 0..=30 {
        let state = format!("{} seconds", i);
        client.queue_activity(|act| act.state(&state).timestamps(|_| timestamps.clone()));
        std::thread::sleep(Duration::from_secs(1));
    }

    // we can also override the queued activity by using set_activity
    // which will always try to send immediately
    client.queue_activity(|act| {
        act.state("this will never appear...")
            .timestamps(|_| timestamps.clone())
    });
    std::thread::sleep(Duration::from_secs(3));
    client.set_activity(|act| act.state("done!").timestamps(|_| timestamps.clone()))?;
    client.block_on()?;

    Ok(())
}
