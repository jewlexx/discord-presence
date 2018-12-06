[![Build Status][travis-ci-badge]][travis-ci-page] [![Build status][appveyor-ci-badge]][appveyor-ci-page] [![crates.io][crates-io-badge-ver]][crates-io-page] [![crates.io][crates-io-badge-dl]][crates-io-page] [![Discord][discord-badge]][discord-invite]


# Discord RPC Client

Discord RPC client for Rust


## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
discord-rpc-client = "^0.2"
```


## Example

```rust
extern crate discord_rpc_client;

use std::{env, thread, time};
use discord_rpc_client::Client;

fn main() {
    // Get our main status message
    let state_message = env::args().nth(1).expect("Requires at least one argument");

    // Create the client
    let mut drpc = Client::new(425407036495495169);

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


<!-- links -->

[gitlab-ci-badge]: https://gitlab.com/valeth/discord-rpc-client.rs/badges/master/pipeline.svg
[gitlab-repo-master]: https://gitlab.com/valeth/discord-rpc-client.rs/commits/master
[crates-io-badge-ver]: https://img.shields.io/crates/v/discord-rpc-client.svg
[crates-io-badge-dl]: https://img.shields.io/crates/d/discord-rpc-client.svg
[crates-io-page]: https://crates.io/crates/discord-rpc-client
[travis-ci-badge]: https://travis-ci.org/valeth/discord-rpc-client.rs.svg?branch=master
[travis-ci-page]: https://travis-ci.org/valeth/discord-rpc-client.rs
[appveyor-ci-badge]: https://ci.appveyor.com/api/projects/status/3fba86eipx0sgsjp?svg=true
[appveyor-ci-page]: https://ci.appveyor.com/project/valeth/discord-rpc-client-rs
[discord-invite]: https://discordapp.com/invite/zfavwrA
[discord-badge]: https://discordapp.com/api/guilds/200751504175398912/widget.png
