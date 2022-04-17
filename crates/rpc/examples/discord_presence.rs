use discord_rpc::Client as DiscordRPC;
use simplelog::*;
use std::io;

fn main() {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let mut drpc = DiscordRPC::new(425407036495495169);

    drpc.on_ready(|_ctx| {
        println!("READY!");
    });

    drpc.on_error(|_ctx| {
        eprintln!("An error occured");
    });

    drpc.start();

    loop {
        let mut buf = String::new();

        io::stdin().read_line(&mut buf).unwrap();
        buf.pop();

        if buf.is_empty() {
            if let Err(why) = drpc.clear_activity() {
                println!("Failed to clear presence: {}", why);
            }
        } else if let Err(why) = drpc.set_activity(|a| {
            a.state(buf).assets(|ass| {
                ass.large_image("ferris_wat")
                    .large_text("wat.")
                    .small_image("rusting")
                    .small_text("rusting...")
            })
        }) {
            println!("Failed to set presence: {}", why);
        }
    }
}
