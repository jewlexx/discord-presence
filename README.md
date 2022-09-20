# discord-ipc-rust

Copied from [sardonicism-04/discord-rich-presence](https://github.com/sardonicism-04/discord-rich-presence)

### Why/Goals?
- Login with `access_token`
- Send RPC commands
- Receive events/commands

### Example

Simple demo of how to use this

```rust
use discord_ipc::{DiscordIpc, DiscordIpcClient, EventType, models::BasedCommands::* };

// get all messages from the client
fn hadle_message(event_type: EventType) {
  println!("Data: {:?}", event_type);
}

fn main() {
  // load env vars
  dotenv::dotenv().ok();

  // access token from env
  let access_token = dotenv::var("ACCESS_TOKEN").unwrap();
  // client id from env
  let client_id = dotenv::var("CLIENT_ID").unwrap();

  // connect to discord client with overlayed id
  let mut client = DiscordIpcClient::new(&client_id).unwrap();

  // login to the client
  client.login(access_token).unwrap();

  // send a simple event to the discord client
  client.send_cmd(GetSelectedVoiceChannel).ok();

  // sub to all events to via this listener
  client.add_event_handler(hadle_message);
}
```

### Setup
Make sure to add an `.env` file with a valid access token and client id.
```
ACCESS_TOKEN="dank_meme"
CLIENT_ID="42069420"
```

### Run locally
`make` or `carco run` to run the main file

### See also
https://github.com/sardonicism-04/discord-rich-presence
https://gitlab.com/valeth/discord-rpc-client.rs
https://github.com/ldesgoui/discord_game_sdk
https://github.com/jewlexx/discord-presence

https://discord.com/developers/docs/topics/rpc
