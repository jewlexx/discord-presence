# Discord RPC

[![crates.io](https://img.shields.io/crates/v/discord-presence.svg)](https://crates.io/crates/discord-presence)
[![crates.io](https://img.shields.io/crates/d/discord-presence.svg)](https://crates.io/crates/discord-presence)

Discord RPC client for Rust forked from [Discord RPC Client](https://gitlab.com/valeth/discord-rpc-client.rs)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
discord-rpc-client = "^0.4"
```

## Example

```rust
use std::{env, thread, time};
use discord_rpc_client::{Client, Event};

fn main() {
    // Get our main status message
    let state_message = env::args().nth(1).expect("Requires at least one argument");

    // Create the client
    let mut drpc = Client::new(425407036495495169);

    // Register event handlers with the corresponding methods
    drpc.on_ready(|_ctx| {
        println!("ready?");
    });

    // or

    drpc.on_event(Event::Ready, |ctx| {
        println!("READY!");
    });

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();

    // Set the activity
    drpc.set_activity(|act| act.state(state_message))
        .expect("Failed to set activity");

    // Wait 10 seconds before exiting
    thread::sleep(time::Duration::from_secs(10));
}
```

> More examples can be found in the examples directory.

## Contributions

See [CONTRIBUTING.md](CONTRIBUTING.md)
