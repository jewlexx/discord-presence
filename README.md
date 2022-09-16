# discord-ipc-rust

This is copied from [sardonicism-04/discord-rich-presence](https://github.com/sardonicism-04/discord-rich-presence) with to goal to be able to subscribe/listen to events from the discord IPC socket allowing more than presence control.

### Goals/Progress
- [x] Change the name to `discord-ipc-rust` for clarity
- [ ] Add a listen method to allow the consumer to recv all events sent from discord
- [ ] Remove all the presence specific code
- [ ] Publish to crates.io

### Example

This is not working currently and is the "concept" of how it'd work.

```rust
use discord_ipc_rust::{activity, DiscordIpc, DiscordIpcClient};

fn handle_connection(stream: TcpStream) {
  // TODO: get data frames?
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut client = DiscordIpcClient::new("<some application ID>")?;

  client.connect()?;

  // TOOD: some rust code for a callback/listener?
  // to be clear I know almost nothing about rust so this is 🍝
  client.on_message(handle_connection)
  
  client.close()?;

  Ok(())
}
```

### Debugging for now
`make` to run the default test