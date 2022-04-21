# Bevy Discord Presence Plugin

[![crates.io](https://img.shields.io/crates/v/bevy-discord-presence.svg)](https://crates.io/crates/bevy-discord-presence)
[![crates.io](https://img.shields.io/crates/d/bevy-discord-presence.svg)](https://crates.io/crates/bevy-discord-presence)

A simplistic bevy plugin for discord presence integration within the bevy game engine

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bevy-discord-presence = "0.2"
```

or run:

```shell
cargo add bevy-discord-presence
```

if you have `cargo-edit` installed

## Example

```rust
use bevy::prelude::*;

use bevy_discord_presence::{RPCConfig, RPCPlugin};

fn main() {
    println!("hello world!");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(RPCPlugin(RPCConfig {
        app_id: 965125975941709834,
        show_time: true,
    }));

    app.run();
}
```

> More examples can be found in the examples directory.

## Changelog

See [CHANGELOG.md](CHANGELOG.md)

## Contributions

See [CONTRIBUTING.md](/CONTRIBUTING.md)
