use discord_presence::Client as DiscordRPC;
use std::io;

fn main() {
    let mut drpc = DiscordRPC::new(425407036495495169);

    drpc.on_ready(|_ctx| {
        println!("READY!");
    });

    drpc.on_error(|ctx| {
        eprintln!("An error occured, {}", ctx.event);
    });

    drpc.start();

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
    loop {}
}
