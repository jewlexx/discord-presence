use std::{mem::forget, thread::sleep, time::Duration};

use discord_presence::Client;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let mut client = Client::new(1003450375732482138);

    _ = client.start();

    {
        let ready = client.on_ready({
            let mut client = client.clone();

            move |_ctx| {
                println!("READY!");

                client
                    .set_activity(|a| {
                        a.state("Rust")
                            .details("Programming")
                            .assets(|a| a.large_image("rust"))
                    })
                    .unwrap();
            }
        });

        // we can `std::mem::forget` the event listener's handle to keep it
        // registered until `drpc` is dropped
        forget(ready);
    }

    // an alternative is to store the handle until you're ready to unregister the
    // listener
    let _error = client.on_error(|ctx| {
        eprintln!("An error occured, {}", ctx.event);
    });

    tracing::trace!("Made it to the final line");

    // keep the main thread alive
    loop {
        sleep(Duration::from_secs(100));
    }
}
